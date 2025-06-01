use crate::{
    algorithm::heuristics::{self, PContainer},
    arena::{Mouvement, Puzzle},
};
use std::fmt::Display;
use std::rc::Rc;

// Tree
pub struct Arena {
    pub nodes: Vec<Node>,

    heuristic: heuristics::EHeuristic,
    reference: Rc<PContainer>,

    pub openlist: Vec<usize>,
    pub closelist: Vec<usize>,
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
        let root = Node::new(state, self.heuristic.clone(), &self.reference);
        self.nodes.push(root);
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn generate_children(&mut self, parent_idx: usize) {
        // already generated check
        if self.nodes[parent_idx].is_children_generated {
            return;
        }

        for v in [
            self.nodes[parent_idx].state.clone_up(),
            self.nodes[parent_idx].state.clone_down(),
            self.nodes[parent_idx].state.clone_left(),
            self.nodes[parent_idx].state.clone_right(),
        ] {
            match v {
                Ok(state) => {
                    let len = self.nodes.len();
                    let node = Node::new(state, self.heuristic.clone(), &self.reference);
                    self.nodes.push(node);
                    self.nodes[parent_idx].children.push(len);
                }
                Err(()) => (),
            };
        }
        self.nodes[parent_idx].is_children_generated = true;
    }

    /// pop the element from openlist
    fn pick_best_option(&mut self) -> usize {
        let mut res: (usize, u32) = (usize::MAX, u32::MAX);
        let mut i = 0;
        for (_i, idx) in self.openlist.iter().enumerate() {
            let node = &self.nodes[*idx];
            if node.state.mouv_count as u32 + node.cost < res.1 {
                res.0 = *idx;
                res.1 = node.state.mouv_count as u32 + node.cost;
                i = _i;
            }
        }
        self.openlist.remove(i);
        res.0
    }

    fn push_to_openlist(&mut self, parent_idx: usize) {
        for child in self.nodes[parent_idx].children.clone() {
            let mut skip_child = false;
            for closelist_idx in self.closelist.iter() {
                if self.nodes[child].state == self.nodes[*closelist_idx].state {
                    skip_child = true;
                    break;
                }
            }
            if !skip_child {
                self.openlist.push(child);
            }
        }
    }

    pub fn solve_puzzle(&mut self) -> Option<Puzzle> {
        // checking if arena is initialized
        if self.nodes.is_empty() {
            return None;
        }
        let mut current_node_idx = 0;
        let mut counter = 0;

        // A* algorithm loop
        loop {
            counter += 1;
            // if counter >= 15 {
            //     break;
            // }
            // println!("loop: {}", counter);
            println!("current index: {}", current_node_idx);
            println!("{}", self.nodes[current_node_idx]);

            // solved check
            if self.nodes[current_node_idx]
                .state
                .is_solved(self.reference.to_vec())
            {
                println!("loop done: {}", counter);
                return Some(self.nodes[current_node_idx].state.clone());
            }

            // children generation of current
            self.generate_children(current_node_idx);
            self.closelist.push(current_node_idx);
            self.push_to_openlist(current_node_idx);

            // pick the new current
            current_node_idx = self.pick_best_option();
        }
        None
    }
}

// Node
pub struct Node {
    pub state: Puzzle,

    pub reference: Rc<PContainer>,
    pub heuristic: heuristics::EHeuristic,
    pub cost: u32,

    pub children: Vec<usize>,
    is_children_generated: bool,
}

impl Node {
    fn new(state: Puzzle, heuristic: heuristics::EHeuristic, reference: &Rc<PContainer>) -> Self {
        let mut node = Node {
            state,
            heuristic,
            cost: 0,
            children: Vec::with_capacity(1000),
            reference: Rc::clone(reference),
            is_children_generated: false,
        };
        node.calculate_cost();
        node
    }

    fn calculate_cost(&mut self) {
        self.cost = heuristics::set_heuristics(&self.heuristic, &self.state, &self.reference);
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Puzzle Node")?;
        write!(f, "{}", self.state)?;
        writeln!(f, "cost: {}", self.cost)?;
        writeln!(f, "total: {}", self.cost + self.state.mouv_count as u32)?;
        writeln!(f, "number of children: {}", self.children.len())?;
        writeln!(f, "{}", "--------------------".repeat(3))?;
        Ok(())
    }
}
