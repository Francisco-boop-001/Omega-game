use super::cell::Cell;
use super::grid::CaGrid;

#[derive(Debug, Clone)]
pub struct ArenaSnapshot {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub label: String,
}

impl ArenaSnapshot {
    pub fn capture(grid: &CaGrid, label: String) -> Self {
        Self {
            cells: grid.front_buffer().to_vec(),
            width: grid.width(),
            height: grid.height(),
            label,
        }
    }

    pub fn restore(&self, grid: &mut CaGrid) {
        if self.width == grid.width() && self.height == grid.height() {
            grid.load_front_buffer(&self.cells);
        }
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotManager {
    snapshots: Vec<ArenaSnapshot>,
    max_snapshots: usize,
}

impl Default for SnapshotManager {
    fn default() -> Self {
        Self { snapshots: Vec::new(), max_snapshots: 5 }
    }
}

impl SnapshotManager {
    pub fn push(&mut self, snapshot: ArenaSnapshot) {
        self.snapshots.push(snapshot);
        if self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
        }
    }

    pub fn pop(&mut self) -> Option<ArenaSnapshot> {
        self.snapshots.pop()
    }

    pub fn list(&self) -> &[ArenaSnapshot] {
        &self.snapshots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_restore_roundtrip() {
        let mut grid = CaGrid::new(10, 10);
        let cell = Cell { heat: 123, ..Cell::default() };
        grid.set_immediate(5, 5, cell);

        let snapshot = ArenaSnapshot::capture(&grid, "test".to_string());

        let mut new_grid = CaGrid::new(10, 10);
        snapshot.restore(&mut new_grid);

        assert_eq!(new_grid.get(5, 5).heat, 123);
    }

    #[test]
    fn test_snapshot_manager_limit() {
        let mut manager = SnapshotManager::default();
        let grid = CaGrid::new(2, 2);

        for i in 0..10 {
            manager.push(ArenaSnapshot::capture(&grid, format!("snap_{}", i)));
        }

        assert_eq!(manager.list().len(), 5);
        assert_eq!(manager.list()[0].label, "snap_5");
        assert_eq!(manager.list()[4].label, "snap_9");
    }
}
