use crate::RenderFrame;

pub fn compose_interaction_lines(frame: &RenderFrame) -> Vec<String> {
    let mut lines = Vec::new();
    if frame.interaction_lines.is_empty() {
        lines.push("Idle Channel".to_string());
        lines.push("No active prompt.".to_string());
        lines.push("Tip: move, inspect, or open inventory to continue.".to_string());
        return lines;
    }

    let mut iter = frame.interaction_lines.iter();
    if let Some(active) = iter.next() {
        let active_text = active.strip_prefix("ACTIVE: ").unwrap_or(active);
        lines.push(format!(">> {active_text} <<"));
    }

    for entry in iter.take(8) {
        if let Some(input) = entry.strip_prefix("INPUT: ") {
            lines.push(format!("Typed: {input}"));
        } else if let Some(objective) = entry.strip_prefix("OBJECTIVE: ") {
            lines.push(format!("Objective: {objective}"));
        } else if let Some(next) = entry.strip_prefix("NEXT: ") {
            lines.push(format!("Next: {next}"));
        } else {
            lines.push(entry.clone());
        }
    }

    lines
}
