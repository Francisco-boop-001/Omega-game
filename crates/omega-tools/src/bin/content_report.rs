use anyhow::Result;
use omega_content::{load_default_content, validate_content_pack};

fn main() -> Result<()> {
    let pack = load_default_content()?;
    let report = validate_content_pack(&pack);
    println!("{}", serde_json::to_string_pretty(&report)?);

    if report.has_errors() {
        anyhow::bail!(
            "content validation failed: {} errors, {} warnings",
            report.error_count,
            report.warning_count
        );
    }
    Ok(())
}
