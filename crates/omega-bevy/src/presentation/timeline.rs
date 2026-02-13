use crate::RenderFrame;

pub fn compose_timeline_lines(frame: &RenderFrame) -> Vec<String> {
    let mut lines = vec!["Recent Outcomes".to_string()];
    if frame.timeline_lines.is_empty() {
        lines.push("No outcomes yet.".to_string());
        lines.push("Actions will appear here in chronological order.".to_string());
        return lines;
    }

    let start = frame.timeline_lines.len().saturating_sub(10);
    for line in frame.timeline_lines.iter().skip(start) {
        lines.push(format!("- {line}"));
    }

    lines
}
