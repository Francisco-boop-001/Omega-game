use anyhow::Result;

#[path = "../mechanics_shared.rs"]
mod mechanics_shared;

use mechanics_shared::{
    MechanicTier, MechanicsParityMatrix, MissingDefect, MissingDefectBoard, ParityStatus,
    defect_board_to_markdown, ensure_target_dir, read_json, write_json, write_text,
};

fn severity_for(tier: &MechanicTier, status: &ParityStatus) -> &'static str {
    match (tier, status) {
        (MechanicTier::Main, ParityStatus::Missing) => "P0",
        (MechanicTier::Main, ParityStatus::Partial) => "P1",
        (MechanicTier::Secondary, ParityStatus::Missing) => "P1",
        (MechanicTier::Secondary, ParityStatus::Partial) => "P2",
        (MechanicTier::Tertiary, ParityStatus::Missing) => "P2",
        (MechanicTier::Tertiary, ParityStatus::Partial) => "P3",
        _ => "P3",
    }
}

fn main() -> Result<()> {
    ensure_target_dir()?;
    let matrix = read_json::<MechanicsParityMatrix>("target/mechanics-parity-matrix.json")?;
    let mut defects = Vec::new();
    let mut counter = 1usize;
    for row in &matrix.rows {
        if row.parity_status != ParityStatus::Missing && row.parity_status != ParityStatus::Partial
        {
            continue;
        }
        defects.push(MissingDefect {
            id: format!("MECH-MISS-{counter:04}"),
            severity: severity_for(&row.tier, &row.parity_status).to_string(),
            tier: row.tier.clone(),
            domain: row.domain.clone(),
            mechanic_id: row.mechanic_id.clone(),
            parity_status: row.parity_status.clone(),
            anchor: row.evidence_static.clone(),
            notes: row.notes.clone(),
        });
        counter += 1;
    }
    defects.sort_by(|a, b| a.severity.cmp(&b.severity).then(a.mechanic_id.cmp(&b.mechanic_id)));

    let board = MissingDefectBoard {
        generated_at_utc: mechanics_shared::now_utc_unix(),
        total: defects.len(),
        open: defects.len(),
        defects,
    };
    write_json("target/mechanics-missing-defect-board.json", &board)?;
    write_text(
        "target/mechanics-missing-defect-board.md",
        &defect_board_to_markdown(&board, "Mechanics Missing Defect Board"),
    )?;
    let mut plan = String::new();
    plan.push_str("# Mechanics Closure Sequencing Plan\n\n");
    plan.push_str("- Phase 1: close all `P0` main-mechanic missing rows.\n");
    plan.push_str("- Phase 2: close `P1` main/secondary rows and convert to equivalent.\n");
    plan.push_str("- Phase 3: close remaining secondary rows, then tertiary quality rows.\n");
    plan.push_str("- Exit when `target/mechanics-missing-defect-board.json` open=0 and `target/mechanics-parity-matrix.json` pass=true.\n");
    write_text("target/mechanics-closure-sequencing.md", &plan)?;

    println!("mechanics missing board: total={} open={}", board.total, board.open);
    Ok(())
}
