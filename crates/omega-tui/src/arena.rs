use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, Paragraph, Wrap};
use omega_core::simulation::catastrophe::Catastrophe;
use omega_core::simulation::snapshot::{SnapshotManager, ArenaSnapshot};
use omega_core::simulation::grid::CaGrid;
use omega_core::simulation::wind::WindGrid;
use omega_core::{GameState, Position};
use crossterm::event::{KeyCode, MouseEvent, MouseEventKind};
use std::time::{Instant, Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushMode {
    Fire,
    Water,
    Ash,
    None,
}

#[derive(Debug, Clone)]
pub struct ArenaUi {
    pub brush_mode: BrushMode,
    pub show_perf_hud: bool,
    pub show_logs: bool,
    pub spawner_selected: usize,
    pub spawner_catalog: Vec<String>,
    pub snapshot_manager: SnapshotManager,
    pub event_log: Vec<String>,
    pub fps: f64,
    pub ca_update_ms: f64,
    pub projectile_count: usize,
    pub particle_count: usize,
    pub turret_active: bool,
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
            snapshot_manager: SnapshotManager::default(),
            event_log: Vec::new(),
            fps: 0.0,
            ca_update_ms: 0.0,
            projectile_count: 0,
            particle_count: 0,
            turret_active: false,
            last_paint_time: Instant::now(),
        }
    }
}

#[derive(Debug)]
pub enum ArenaAction {
    SpawnMonster(String),
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
            Line::from(vec![
                Span::raw("[T] Turret: "),
                if self.turret_active {
                    Span::styled("ON", Style::default().fg(Color::Green))
                } else {
                    Span::styled("OFF", Style::default().fg(Color::Gray))
                },
            ]),
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

        let brush_text = vec![
            Line::from(vec![
                Span::raw("Brush: "),
                Span::styled("[F]ire", brush_style(BrushMode::Fire)),
                Span::raw(" "),
                Span::styled("[W]ater", brush_style(BrushMode::Water)),
                Span::raw(" "),
                Span::styled("[A]sh", brush_style(BrushMode::Ash)),
                Span::raw(" "),
                Span::styled("[N]one", brush_style(BrushMode::None)),
            ]),
        ];
        let brush = Paragraph::new(brush_text)
            .block(Block::default().title("ELEMENTAL BRUSH").borders(Borders::ALL));
        frame.render_widget(brush, chunks[1]);

        // c. Monster Spawner
        let items: Vec<ratatui::widgets::ListItem> = self.spawner_catalog
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let style = if i == self.spawner_selected {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                ratatui::widgets::ListItem::new(name.as_str()).style(style)
            })
            .collect();
        let spawner = List::new(items)
            .block(Block::default().title("MONSTER SPAWNER").borders(Borders::ALL));
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
            Line::from(vec![
                Span::raw(format!("Proj: {}  Part: {}  CA: {:.1}ms", 
                    self.projectile_count, self.particle_count, self.ca_update_ms)),
            ]),
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
                self.snapshot_manager.push(ArenaSnapshot::capture(grid, "Manual Snapshot".to_string()));
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
                self.log_event(&format!("Turret mode: {}", if self.turret_active { "ON" } else { "OFF" }));
                ArenaAction::Consumed
            }
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
                if self.spawner_selected > 0 {
                    self.spawner_selected -= 1;
                }
                ArenaAction::Consumed
            }
            KeyCode::Down => {
                if self.spawner_selected < self.spawner_catalog.len() - 1 {
                    self.spawner_selected += 1;
                }
                ArenaAction::Consumed
            }
            KeyCode::Enter => {
                ArenaAction::SpawnMonster(self.spawner_catalog[self.spawner_selected].clone())
            }
            _ => ArenaAction::None
        }
    }

    pub fn handle_brush_paint(
        &mut self,
        mouse: MouseEvent,
        map_area: Rect,
        grid: &mut CaGrid,
    ) {
        if self.brush_mode == BrushMode::None {
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
