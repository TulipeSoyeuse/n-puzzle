use crate::{
    algorithm::heuristics::{self, PContainer},
    arena::{Mouvement, Puzzle},
};
use std::rc::Rc;
use std::{fmt::Display, iter::Enumerate};

// Tree
pub struct Arena {
    nodes: Vec<Node>,

    heuristic: heuristics::EHeuristic,
    reference: Rc<PContainer>,

    openlist: Vec<usize>,
    closelist: Vec<usize>,
}

impl Arena {
    pub fn new(heuristic: heuristics::EHeuristic, reference: Rc<PContainer>) -> Self {
        Arena {
            nodes: vec![],
            openlist: vec![],
            closelist: vec![],
            heuristic,
            reference,
        }
    }

    pub fn init(&mut self, state: Puzzle) {
        let root = Node::new(state, self.heuristic.clone(), &self.reference, 0);
        self.nodes.push(root);
    }

    pub fn generate_children(&mut self, parent_idx: usize) {
        for v in [
            self.nodes[parent_idx].state.clone_up(),
            self.nodes[parent_idx].state.clone_down(),
            self.nodes[parent_idx].state.clone_left(),
            self.nodes[parent_idx].state.clone_right(),
        ] {
            match v {
                Ok(state) => {
                    let len = self.nodes.len();
                    let node = Node::new(state, self.heuristic.clone(), &self.reference, len);
                    self.nodes.push(node);
                    self.nodes[parent_idx].children.push(len);
                }
                Err(()) => (),
            };
        }
    }

    pub fn solve_puzzle(&mut self) -> Option<Puzzle> {
        // checking if arena is initialized
        if self.nodes.is_empty() {
            return None;
        }
        let current_node_idx = 0;

        // A* algorithm loop
        loop {
            let children = self.generate_children(current_node_idx);
        }
    }
}

// Node
pub struct Node {
    pub state: Puzzle,

    pub reference: Rc<PContainer>,
    pub heuristic: heuristics::EHeuristic,
    pub cost: u32,

    pub idx: usize,
    pub children: Vec<usize>,
    is_children_generated: bool,
}

impl Node {
    fn new(
        state: Puzzle,
        heuristic: heuristics::EHeuristic,
        reference: &Rc<PContainer>,
        idx: usize,
    ) -> Self {
        let mut node = Node {
            state,
            heuristic,
            cost: 0,
            children: Vec::with_capacity(1000),
            reference: Rc::clone(reference),
            is_children_generated: false,
            idx,
        };
        node.calculate_cost();
        node
    }

    fn calculate_cost(&mut self) {
        heuristics::set_heuristics(&self.heuristic, &self.state, &self.reference);
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Puzzle Node")?;
        write!(f, "{}", self.state)?;
        writeln!(f, "{}", "--------------------".repeat(3))?;
        writeln!(f, "cost: {}", self.cost)?;
        writeln!(f, "number of children: {}", self.children.len())?;
        Ok(())
    }
}
