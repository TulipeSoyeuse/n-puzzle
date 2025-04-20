use crate::arena::{Mouvement, Puzzle};

// Tree
pub struct Tree {
    root: Option<Node>,
}
impl Tree {
    fn new(root: Option<Node>) -> Self {
        Tree { root }
    }
}

// Node
pub struct Node {
    pub state: Puzzle,
    pub heuristic: u32,
    pub childrens: Vec<Option<Node>>,
}
impl Node {
    fn new(state: Puzzle) -> Self {
        Node {
            state,
            heuristic: 0,
            childrens: Vec::new(),
        }
    }

    fn new_child(&mut self, state: Puzzle) {
        let child = Node::new(state);
        self.childrens.push(Some(child))
    }

    fn generate_child(&mut self) {
        for i in [
            self.state.clone_left(),
            self.state.clone_up(),
            self.state.clone_right(),
            self.state.clone_down(),
        ] {
            match i {
                Ok(puzzle) => self.new_child(puzzle),
                Err(()) => (),
            }
        }

        for child in self.childrens {
            set_heuristics(child);
        }
    }
}
