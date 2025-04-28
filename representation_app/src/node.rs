//node.rs





#[derive(Copy, Clone, Debug)]
pub struct Node {
    id: u16, //id of the node
    pos: Vec2,        // World position (x, y)
    vel: Vec2,        // Velocity
    mass: u16,        // Mass (for force calculation)
    radius: f32,      // Visual size maybe
    name: &'static str,  // static lifetime
    friends: Vec::with_capacity(20) , // Indices of other nodes
}




impl Node {
    // Constructor (recommended)
    pub fn new(id: u16, pos: Vec2, vel: Vec2, mass: u16, radius: f32, name: &'static str) -> Self {
        Node {
            id,
            cell: (0, 0),
            pos,
            vel,
            mass,
            radius,
            name,
            friends: Vec::with_capacity(20),
        }
    }



    // Access to friends (read-only)
    pub fn friends(&self) -> &Vec<usize> {
        &self.friends
    }

    // Add a friend
    pub fn add_friend(&mut self, friend_id: usize) {
        self.friends.push(friend_id);
    }

    // Clear friends
    pub fn clear_friends(&mut self) {
        self.friends.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 👈 Import the parent module

    #[test]
    fn friends() {
        let mut node = Node::new();
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