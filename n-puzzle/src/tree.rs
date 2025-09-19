//! Tree
//!
//! regroup structure and algorithm to solve a n puzzle
//!
use crate::{
    error::AppError,
    heuristics::{EHeuristic, PContainer, set_heuristics},
    puzzle::{Mouvement, Puzzle},
};
use colored::Colorize;
use std::fmt::Display;
use std::rc::Rc;

/// Arena structure
/// ---------------
/// hold the ownership of all node and methods
pub struct Arena {
    pub nodes: Vec<Node>,
    pub solved_node: Option<usize>,

    heuristic: EHeuristic,
    reference: Rc<PContainer>,

    pub openlist: Vec<usize>,
    pub closelist: Vec<usize>,
}

/// solving the puzzle use a binary tree to explore all possibility from a certain setup (max 4 new possible state). then the heuristic is used
/// to calculate how far each new state are, from a resolved state. all NEW states are pushed to an openlist (there are exeption to this :
/// if a new state is equal to an already explored state for exemple, for this purpose a closelist is used to keep track of all explored states)
/// then the algorithm find the state in the openlist with the smallest heuristic and repeat this process until the puzzle is solved
impl Arena {
    pub fn new(heuristic: EHeuristic, reference: Rc<PContainer>) -> Self {
        Arena {
            nodes: vec![],
            openlist: vec![],
            closelist: vec![],
            solved_node: None,
            heuristic,
            reference,
        }
    }

    pub fn init(&mut self, state: Puzzle) {
        let root = Node::new(state, 0, self.heuristic.clone(), &self.reference, None);
        self.nodes.push(root);
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// generate all states derived from a node (parent_idx) and add to this node children
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
                    let node = Node::new(
                        state,
                        len,
                        self.heuristic.clone(),
                        &self.reference,
                        Some(parent_idx),
                    );
                    self.nodes.push(node);
                    self.nodes[parent_idx].children.push(len);
                }
                Err(()) => (),
            };
        }
        self.nodes[parent_idx].is_children_generated = true;
    }

    /// iter over openlist to find the optimal step, the item is poped from openlist
    /// and it's index is returned
    fn pick_best_option(&mut self) -> usize {
        let mut res: (usize, usize) = (usize::MAX, usize::MAX);
        let mut i = 0;
        for (_i, idx) in self.openlist.iter().enumerate() {
            let node = &self.nodes[*idx];
            if node.state.mouv_count + node.cost < res.1 {
                res.0 = *idx;
                res.1 = node.state.mouv_count + node.cost;
                i = _i;
            }
        }
        self.openlist.remove(i);
        res.0
    }

    /// iter over closedlist (I.E all the already explored state) and check if for all children of a parent node
    /// the state has not already been explored then push it to the openlist
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

    pub fn solve_puzzle(&mut self) -> Result<(), AppError> {
        // checking if arena is initialized
        if self.nodes.is_empty() {
            return Err(AppError::new("tree is empty"));
        }
        let mut current_node_idx = 0;

        // A* algorithm loop
        loop {
            // solved check
            if self.nodes[current_node_idx]
                .state
                .is_solved(self.reference.to_vec())
            {
                self.solved_node = Some(current_node_idx);
                return Ok(());
            }

            // children generation of current
            self.generate_children(current_node_idx);
            self.closelist.push(current_node_idx);
            self.push_to_openlist(current_node_idx);

            // pick the new current
            current_node_idx = self.pick_best_option();
        }
    }

    pub fn display_solution(&self) {
        match self.solved_node {
            Some(mut idx) => {
                println!("solution: ");
                let mut path = Vec::new();
                while idx != 0 {
                    path.push(idx);
                    idx = self.nodes[idx].parent;
                }
                for i in path.into_iter().rev() {
                    println!("{}", self.nodes[i]);
                }
            }
            None => (),
        }
    }
}

// Node
pub struct Node {
    pub state: Puzzle,
    pub id: usize,

    pub reference: Rc<PContainer>,
    pub heuristic: EHeuristic,
    pub cost: usize,
    pub parent: usize,

    pub children: Vec<usize>,
    is_children_generated: bool,
}

impl Node {
    fn new(
        state: Puzzle,
        id: usize,
        heuristic: EHeuristic,
        reference: &Rc<PContainer>,
        parent: Option<usize>,
    ) -> Self {
        let mut node = Node {
            state,
            id,
            heuristic,
            cost: 0,
            parent: parent.unwrap_or(0),
            children: Vec::with_capacity(1000),
            reference: Rc::clone(reference),
            is_children_generated: false,
        };
        node.calculate_cost();
        node
    }

    fn calculate_cost(&mut self) {
        self.cost = set_heuristics(&self.heuristic, &self.state, &self.reference);
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {}  ────────────────────────────────────",
            "Node".bold().cyan(),
            format!("#{}", self.id).cyan()
        )?;
        writeln!(
            f,
            "{} {} | {} ({},{}) | {} {} | {} {}",
            "Move #:".bold(),
            self.state.mouv_count.to_string().yellow(),
            "Empty:".bold(),
            self.state.empty_cell.y,
            self.state.empty_cell.x,
            "Cost:".bold(),
            if self.cost == 0 {
                self.cost.to_string().green()
            } else if self.cost < 10 {
                self.cost.to_string().yellow()
            } else {
                self.cost.to_string().red()
            },
            "Total:".bold(),
            (self.cost + self.state.mouv_count).to_string().blue()
        )?;
        write!(f, "{}", self.state)
    }
}
