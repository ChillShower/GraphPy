//cell.rs
const MAX_NODES_PER_CELL: usize=50;

struct Grid {

    cells: HashMap<(u16, u16), Vec<usize>>, // 🔥 keys are (x,y) cell coordinates
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            cells: HashMap::new(),
        }
    }

    /// Adds a node index to a cell
    pub fn add_node(&mut self, cell: (u16, u16), node_id: usize) {
        self.cells.entry(cell)
            .or_insert_with(Vec::new)
            .push(node_id);
    }

    /// Removes a node index from a cell
    pub fn remove_node(&mut self, cell: (u16, u16), node_id: usize) {
        if let Some(node_list) = self.cells.get_mut(&cell) {
            if let Some(pos) = node_list.iter().position(|&id| id == node_id) {
                node_list.swap_remove(pos); // FAST remove without shifting all elements
            }
            // Clean up empty cell if no nodes left
            if node_list.is_empty() {
                self.cells.remove(&cell);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 👈 Import the parent module

    #[test]
    fn test_add_node() {
        let mut grid = Grid::new();
        grid.add_node((1, 2), 42);
        assert_eq!(grid.cells.get(&(1, 2)).unwrap(), &vec![42]);
    }

    #[test]
    fn test_remove_node() {
        let mut grid = Grid::new();
        grid.add_node((1, 2), 42);
        grid.add_node((1, 2), 43);

        grid.remove_node((1, 2), 42);
        assert_eq!(grid.cells.get(&(1, 2)).unwrap(), &vec![43]);
    }

    #[test]
    fn test_remove_node_and_cleanup_empty_cell() {
        let mut grid = Grid::new();
        grid.add_node((1, 2), 42);

        grid.remove_node((1, 2), 42);
        assert!(grid.cells.get(&(1, 2)).is_none());
    }

    #[test]
    fn test_remove_non_existing_node() {
        let mut grid = Grid::new();
        grid.add_node((1, 2), 42);

        grid.remove_node((1, 2), 99); // Node 99 doesn't exist, should not crash
        assert_eq!(grid.cells.get(&(1, 2)).unwrap(), &vec![42]);
    }

    #[test]
    fn test_add_multiple_nodes() {
        let mut grid = Grid::new();
        grid.add_node((5, 5), 1);
        grid.add_node((5, 5), 2);
        grid.add_node((5, 5), 3);

        assert_eq!(grid.cells.get(&(5, 5)).unwrap().len(), 3);
    }
}