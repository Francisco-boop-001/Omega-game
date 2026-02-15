use crossterm::event::{KeyCode, MouseEvent, MouseEventKind};
use omega_core::simulation::catastrophe::Catastrophe;
use omega_core::simulation::grid::CaGrid;
use omega_core::simulation::snapshot::{ArenaSnapshot, SnapshotManager};
use omega_core::simulation::wind::WindGrid;
use omega_core::{Position, Stats};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, Paragraph, Wrap};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushMode {
    Fire,
    Water,
    Ash,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerCategory {
    Monster,
    Item,
    Hazard,
}

#[derive(Debug, Clone)]
pub struct ArenaUi {
    pub brush_mode: BrushMode,
    pub show_perf_hud: bool,
    pub show_logs: bool,
    pub spawner_selected: usize,
    pub spawner_catalog: Vec<String>,
    pub item_catalog: Vec<String>,
    pub hazard_catalog: Vec<String>,
    pub spawner_category: SpawnerCategory,
    pub item_selected: usize,
    pub hazard_selected: usize,
    pub snapshot_manager: SnapshotManager,
    pub event_log: Vec<String>,
    pub fps: f64,
    pub ca_update_ms: f64,
    pub projectile_count: usize,
    pub particle_count: usize,
    pub turret_active: bool,
    pub tooling_enabled: bool,
    pub last_paint_time: Instant,
}

impl Default for ArenaUi {
    fn default() -> Self {
        Self {
            brush_mode: BrushMode::None,
            show_perf_hud: true,
            show_logs: false,
            spawner_selected: 0,
            spawner_catalog: vec!["rat".to_string(), "goblin".to_string(), "ogre".to_string()],
            item_catalog: vec![
                "short sword".to_string(),
                "buckler".to_string(),
                "healing potion".to_string(),
                "identify scroll".to_string(),
                "fire".to_string(),
            ],
            hazard_catalog: vec!["poison trap".to_string(), "fire trap".to_string()],
            spawner_category: SpawnerCategory::Monster,
            item_selected: 0,
            hazard_selected: 0,
            snapshot_manager: SnapshotManager::default(),
            event_log: Vec::new(),
            fps: 0.0,
            ca_update_ms: 0.0,
            projectile_count: 0,
            particle_count: 0,
            turret_active: false,
            tooling_enabled: true,
            last_paint_time: Instant::now(),
        }
    }
}

#[derive(Debug)]
pub enum ArenaAction {
    SpawnMonster { name: String, stats: Stats },
    SpawnItem { name: String },
    SpawnHazard { effect_id: String, damage: i32 },
    ClearMonsters,
    ClearItems,
    ToggleAiPaused,
    ResetArenaFixture,
    Consumed,
    None,
}

impl ArenaUi {
    pub fn render_controls_panel(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(10), // Catastrophe
                Constraint::Length(6),  // Elemental Brush
                Constraint::Length(8),  // Monster Spawner
                Constraint::Min(5),     // Performance HUD
            ])
            .split(area);

        // a. Catastrophe Controls
        let catastrophe_text = vec![
            Line::from(vec![Span::raw("[1] Great Flood   [2] Forest Fire")]),
            Line::from(vec![Span::raw("[3] Windstorm     [4] Fuel Field")]),
            Line::from(vec![Span::styled("[5] DOOMSDAY", Style::default().fg(Color::Red))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("[S] Snapshot      [R] Restore")]),
            Line::from(vec![Span::raw("[M] Clear Monsters [I] Clear Items")]),
            Line::from(vec![Span::raw("[P] Pause AI      [0] Reset Fixture")]),
            Line::from(vec![
                Span::raw("[T] Turret: "),
                if self.turret_active {
                    Span::styled("ON", Style::default().fg(Color::Green))
                } else {
                    Span::styled("OFF", Style::default().fg(Color::Gray))
                },
            ]),
            Line::from(vec![Span::raw(format!(
                "[`] Tooling: {}",
                if self.tooling_enabled { "ON" } else { "OFF" }
            ))]),
        ];
        let catastrophe = Paragraph::new(catastrophe_text)
            .block(Block::default().title("CATASTROPHE CONTROLS").borders(Borders::ALL));
        frame.render_widget(catastrophe, chunks[0]);

        // b. Elemental Brush
        let brush_style = |mode: BrushMode| {
            if self.brush_mode == mode {
                Style::default().add_modifier(Modifier::BOLD).bg(match mode {
                    BrushMode::Fire => Color::Red,
                    BrushMode::Water => Color::Blue,
                    BrushMode::Ash => Color::DarkGray,
                    BrushMode::None => Color::Gray,
                })
            } else {
                Style::default()
            }
        };

        let brush_text = vec![Line::from(vec![
            Span::raw("Brush: "),
            Span::styled("[F]ire", brush_style(BrushMode::Fire)),
            Span::raw(" "),
            Span::styled("[W]ater", brush_style(BrushMode::Water)),
            Span::raw(" "),
            Span::styled("[A]sh", brush_style(BrushMode::Ash)),
            Span::raw(" "),
            Span::styled("[N]one", brush_style(BrushMode::None)),
        ])];
        let brush = Paragraph::new(brush_text)
            .block(Block::default().title("ELEMENTAL BRUSH").borders(Borders::ALL));
        frame.render_widget(brush, chunks[1]);

        // c. Monster Spawner
        let active_list = match self.spawner_category {
            SpawnerCategory::Monster => &self.spawner_catalog,
            SpawnerCategory::Item => &self.item_catalog,
            SpawnerCategory::Hazard => &self.hazard_catalog,
        };
        let selected_idx = match self.spawner_category {
            SpawnerCategory::Monster => self.spawner_selected,
            SpawnerCategory::Item => self.item_selected,
            SpawnerCategory::Hazard => self.hazard_selected,
        };
        let items: Vec<ratatui::widgets::ListItem> = active_list
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let style = if i == selected_idx {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                ratatui::widgets::ListItem::new(name.as_str()).style(style)
            })
            .collect();
        let title = match self.spawner_category {
            SpawnerCategory::Monster => "MONSTER SPAWNER [TAB]",
            SpawnerCategory::Item => "ITEM SPAWNER [TAB]",
            SpawnerCategory::Hazard => "HAZARD SPAWNER [TAB]",
        };
        let spawner = List::new(items).block(Block::default().title(title).borders(Borders::ALL));
        frame.render_widget(spawner, chunks[2]);

        // d. Performance HUD
        let traffic_light_color = if self.fps >= 58.0 {
            Color::Green
        } else if self.fps >= 45.0 {
            Color::Yellow
        } else {
            Color::Red
        };

        let mut perf_lines = vec![
            Line::from(vec![
                Span::styled("● ", Style::default().fg(traffic_light_color)),
                Span::raw(format!("FPS: {:.1}  ", self.fps)),
            ]),
            Line::from(vec![Span::raw(format!(
                "Proj: {}  Part: {}  CA: {:.1}ms",
                self.projectile_count, self.particle_count, self.ca_update_ms
            ))]),
        ];

        if self.show_logs {
            perf_lines.push(Line::from(vec![Span::raw("--- EVENT LOG ---")]));
            for log in self.event_log.iter().rev().take(10) {
                perf_lines.push(Line::from(vec![Span::raw(log.as_str())]));
            }
        } else {
            perf_lines.push(Line::from(vec![Span::raw("[L] Logs (collapsed)")]));
        }

        let perf_hud = Paragraph::new(perf_lines)
            .block(Block::default().title("PERFORMANCE HUD").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(perf_hud, chunks[3]);
    }

    pub fn handle_arena_input(
        &mut self,
        key: KeyCode,
        grid: &mut CaGrid,
        wind_grid: &mut WindGrid,
        player_pos: Position,
    ) -> ArenaAction {
        if key == KeyCode::Char('`') {
            self.tooling_enabled = !self.tooling_enabled;
            self.log_event(if self.tooling_enabled {
                "Test Ground controls enabled."
            } else {
                "Test Ground controls disabled."
            });
            return ArenaAction::Consumed;
        }
        if !self.tooling_enabled {
            return ArenaAction::None;
        }
        match key {
            KeyCode::Char('1') => {
                Catastrophe::great_flood(grid, (player_pos.x as usize, player_pos.y as usize));
                self.log_event("Great Flood triggered!");
                ArenaAction::Consumed
            }
            KeyCode::Char('2') => {
                Catastrophe::forest_fire_jump(grid, (player_pos.x as usize, player_pos.y as usize));
                self.log_event("Forest Fire triggered!");
                ArenaAction::Consumed
            }
            KeyCode::Char('3') => {
                Catastrophe::massive_windstorm(wind_grid);
                self.log_event("Windstorm triggered!");
                ArenaAction::Consumed
            }
            KeyCode::Char('4') => {
                Catastrophe::fuel_field(grid);
                self.log_event("Fuel Field triggered!");
                ArenaAction::Consumed
            }
            KeyCode::Char('5') => {
                Catastrophe::doomsday(grid, wind_grid);
                self.log_event("DOOMSDAY triggered!");
                ArenaAction::Consumed
            }
            KeyCode::Char('s') => {
                self.snapshot_manager
                    .push(ArenaSnapshot::capture(grid, "Manual Snapshot".to_string()));
                self.log_event("Snapshot saved.");
                ArenaAction::Consumed
            }
            KeyCode::Char('r') => {
                if let Some(s) = self.snapshot_manager.pop() {
                    s.restore(grid);
                    self.log_event("Snapshot restored.");
                } else {
                    self.log_event("No snapshots to restore.");
                }
                ArenaAction::Consumed
            }
            KeyCode::Char('t') => {
                self.turret_active = !self.turret_active;
                self.log_event(&format!(
                    "Turret mode: {}",
                    if self.turret_active { "ON" } else { "OFF" }
                ));
                ArenaAction::Consumed
            }
            KeyCode::Char('m') => ArenaAction::ClearMonsters,
            KeyCode::Char('i') => ArenaAction::ClearItems,
            KeyCode::Char('p') => ArenaAction::ToggleAiPaused,
            KeyCode::Char('0') => ArenaAction::ResetArenaFixture,
            KeyCode::Char('f') => {
                self.brush_mode = BrushMode::Fire;
                ArenaAction::Consumed
            }
            KeyCode::Char('w') => {
                self.brush_mode = BrushMode::Water;
                ArenaAction::Consumed
            }
            KeyCode::Char('a') => {
                self.brush_mode = BrushMode::Ash;
                ArenaAction::Consumed
            }
            KeyCode::Char('n') => {
                self.brush_mode = BrushMode::None;
                ArenaAction::Consumed
            }
            KeyCode::Char('l') => {
                self.show_logs = !self.show_logs;
                ArenaAction::Consumed
            }
            KeyCode::Up => {
                match self.spawner_category {
                    SpawnerCategory::Monster => {
                        if self.spawner_selected > 0 {
                            self.spawner_selected -= 1;
                        }
                    }
                    SpawnerCategory::Item => {
                        if self.item_selected > 0 {
                            self.item_selected -= 1;
                        }
                    }
                    SpawnerCategory::Hazard => {
                        if self.hazard_selected > 0 {
                            self.hazard_selected -= 1;
                        }
                    }
                }
                ArenaAction::Consumed
            }
            KeyCode::Down => {
                match self.spawner_category {
                    SpawnerCategory::Monster => {
                        if self.spawner_selected + 1 < self.spawner_catalog.len() {
                            self.spawner_selected += 1;
                        }
                    }
                    SpawnerCategory::Item => {
                        if self.item_selected + 1 < self.item_catalog.len() {
                            self.item_selected += 1;
                        }
                    }
                    SpawnerCategory::Hazard => {
                        if self.hazard_selected + 1 < self.hazard_catalog.len() {
                            self.hazard_selected += 1;
                        }
                    }
                }
                ArenaAction::Consumed
            }
            KeyCode::Tab => {
                self.spawner_category = match self.spawner_category {
                    SpawnerCategory::Monster => SpawnerCategory::Item,
                    SpawnerCategory::Item => SpawnerCategory::Hazard,
                    SpawnerCategory::Hazard => SpawnerCategory::Monster,
                };
                ArenaAction::Consumed
            }
            KeyCode::Enter => match self.spawner_category {
                SpawnerCategory::Monster => {
                    let name = self.spawner_catalog[self.spawner_selected].clone();
                    let stats = match name.as_str() {
                        "rat" => Stats {
                            hp: 6,
                            max_hp: 6,
                            attack_min: 1,
                            attack_max: 2,
                            defense: 0,
                            weight: 20,
                        },
                        "goblin" => Stats {
                            hp: 12,
                            max_hp: 12,
                            attack_min: 2,
                            attack_max: 4,
                            defense: 1,
                            weight: 50,
                        },
                        "ogre" => Stats {
                            hp: 20,
                            max_hp: 20,
                            attack_min: 3,
                            attack_max: 6,
                            defense: 2,
                            weight: 80,
                        },
                        _ => Stats {
                            hp: 10,
                            max_hp: 10,
                            attack_min: 2,
                            attack_max: 3,
                            defense: 0,
                            weight: 40,
                        },
                    };
                    ArenaAction::SpawnMonster { name, stats }
                }
                SpawnerCategory::Item => {
                    let name = self.item_catalog[self.item_selected].clone();
                    ArenaAction::SpawnItem { name }
                }
                SpawnerCategory::Hazard => {
                    let name = self.hazard_catalog[self.hazard_selected].as_str();
                    let (effect_id, damage) = match name {
                        "poison trap" => ("poison".to_string(), 6),
                        "fire trap" => ("fire".to_string(), 8),
                        _ => ("poison".to_string(), 4),
                    };
                    ArenaAction::SpawnHazard { effect_id, damage }
                }
            },
            _ => ArenaAction::None,
        }
    }

    pub fn handle_brush_paint(&mut self, mouse: MouseEvent, map_area: Rect, grid: &mut CaGrid) {
        if !self.tooling_enabled || self.brush_mode == BrushMode::None {
            return;
        }

        if self.last_paint_time.elapsed() < Duration::from_millis(100) {
            return;
        }

        match mouse.kind {
            MouseEventKind::Down(_) | MouseEventKind::Drag(_) => {
                let x = mouse.column.saturating_sub(map_area.x) as i32;
                let y = mouse.row.saturating_sub(map_area.y) as i32;

                if x >= 0 && x < grid.width() as i32 && y >= 0 && y < grid.height() as i32 {
                    let mut cell = *grid.get(x as usize, y as usize);
                    match self.brush_mode {
                        BrushMode::Fire => {
                            cell.heat = 150;
                        }
                        BrushMode::Water => {
                            cell.wet = 80;
                            cell.liquid = Some(omega_core::simulation::Liquid::Water);
                        }
                        BrushMode::Ash => {
                            cell.solid = Some(omega_core::simulation::Solid::Ash);
                        }
                        BrushMode::None => {}
                    }
                    grid.set_immediate(x as usize, y as usize, cell);
                    self.last_paint_time = Instant::now();
                }
            }
            _ => {}
        }
    }

    pub fn update_perf_data(
        &mut self,
        fps: f64,
        ca_update_ms: f64,
        projectile_count: usize,
        particle_count: usize,
    ) {
        self.fps = fps;
        self.ca_update_ms = ca_update_ms;
        self.projectile_count = projectile_count;
        self.particle_count = particle_count;
    }

    pub fn log_event(&mut self, msg: &str) {
        self.event_log.push(msg.to_string());
        if self.event_log.len() > 20 {
            self.event_log.remove(0);
        }
    }
}
