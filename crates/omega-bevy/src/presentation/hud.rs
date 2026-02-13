use crate::{AppState, RenderFrame};

fn app_state_label(app_state: AppState) -> &'static str {
    match app_state {
        AppState::Boot => "BOOT",
        AppState::Menu => "MENU",
        AppState::InGame => "IN GAME",
        AppState::WizardArena => "ARENA",
        AppState::Pause => "PAUSE",
        AppState::GameOver => "GAME OVER",
    }
}

fn copy_prefixed(frame: &RenderFrame, lines: &mut Vec<String>, prefix: &str) {
    if let Some(value) = frame.hud_lines.iter().find(|line| line.starts_with(prefix)) {
        lines.push(value.clone());
    }
}

pub fn compose_hud_lines(frame: &RenderFrame, app_state: AppState) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("STATE {}", app_state_label(app_state)));
    lines.push("---- Vitals ----".to_string());

    copy_prefixed(frame, &mut lines, "Mode ");
    copy_prefixed(frame, &mut lines, "Turn ");
    copy_prefixed(frame, &mut lines, "Time ");
    copy_prefixed(frame, &mut lines, "HP ");
    copy_prefixed(frame, &mut lines, "Mana ");
    copy_prefixed(frame, &mut lines, "Gold ");
    copy_prefixed(frame, &mut lines, "Inventory ");
    copy_prefixed(frame, &mut lines, "Quest ");
    if let Some(interaction) = frame
        .hud_lines
        .iter()
        .find(|line| line.starts_with("Interaction "))
        .map(|line| line.replacen("Interaction ", "Focus ", 1))
    {
        lines.push(interaction);
    }

    if let Some(objective) = frame.hud_lines.iter().find(|line| line.starts_with("Objective ")) {
        lines.push(format!(">> {}", objective));
        copy_prefixed(frame, &mut lines, "Next ");
    }

    lines.push("Controls: hjklyubn/arrows to move".to_string());
    lines.push("Actions: bump strike | A activate | Z zap | M cast".to_string());
    if lines.len() > 14 {
        lines.truncate(14);
    }
    lines
}
