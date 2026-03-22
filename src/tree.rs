//! Tree
//!
//! regroup structure and algorithm to solve a n puzzle
//!
use crate::{
    error::AppError,
    heuristics::EHeuristic,
    puzzle::{Mouvement, Puzzle, gen_solved_ref},
};

use colored::Colorize;
use std::sync::Arc;
use std::sync::Once;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{collections::BinaryHeap, process::exit};
use std::{collections::HashSet, fmt::Display};

static INIT: Once = Once::new();

fn set_signal_handler(r: Arc<AtomicBool>) {
    INIT.call_once(|| {
        ctrlc::set_handler(move || {
            r.store(true, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    });
}

#[derive(PartialEq, Eq)]
pub struct OpenlistEntry {
    cost: usize,
    node_index: usize,
}

impl PartialOrd for OpenlistEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OpenlistEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

/// Arena structure
/// ---------------
/// hold the ownership of all node and methods
pub struct Arena {
    pub nodes: Vec<Node>,
    pub solved_node: Option<usize>,

    heuristic: EHeuristic,

    pub openlist: BinaryHeap<OpenlistEntry>,
    pub closelist: HashSet<Puzzle>,
}

/// solving the puzzle use a binary tree to explore all possibility from a certain setup (max 4 new possible state). then the heuristic is used
/// to calculate how far each new state are, from a resolved state. all NEW states are pushed to an openlist (there are exeption to this :
/// if a new state is equal to an already explored state for exemple, for this purpose a closelist is used to keep track of all explored states)
/// then the algorithm find the state in the openlist with the smallest heuristic and repeat this process until the puzzle is solved
impl Arena {
    pub fn new(heuristic: EHeuristic) -> Self {
        Arena {
            nodes: vec![],
            openlist: BinaryHeap::new(),
            closelist: HashSet::new(),
            solved_node: None,
            heuristic,
        }
    }

    pub fn init(&mut self, state: Puzzle) {
        let root = Node::new(state, 0, self.heuristic.clone(), None);
        println!("{}", root);
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
                    if !self.closelist.contains(&state) {
                        let len = self.nodes.len();
                        let node = Node::new(state, len, self.heuristic.clone(), Some(parent_idx));
                        self.nodes.push(node);
                        self.nodes[parent_idx].children.push(len);
                    }
                }
                Err(()) => (),
            };
        }
        self.nodes[parent_idx].is_children_generated = true;
    }

    /// find the optimal step, easy from an binaryheap with O(1) complexity.
    /// it's index is returned
    fn pick_best_option(&mut self) -> Result<usize, AppError> {
        if let Some(res) = self.openlist.pop() {
            Ok(res.node_index)
        } else {
            Err(AppError::new("Openlist is empty"))
        }
    }

    /// iter over closedlist (I.E all the already explored state) and check if for all children of a parent node
    /// the state has not already been explored then push it to the openlist
    fn push_to_openlist(&mut self, parent_idx: usize) {
        let parent = &self.nodes[parent_idx];
        for child_index in &parent.children {
            let child = &self.nodes[*child_index];
            self.openlist.push(OpenlistEntry {
                cost: child.h_cost + child.state.mouv_count,
                node_index: *child_index,
            });
        }
    }

    pub fn solve_puzzle(&mut self, debug: bool) -> Result<(), AppError> {
        // signal handling
        let running = Arc::new(AtomicBool::new(false));
        set_signal_handler(running.clone());
        // checking if arena is initialized
        if self.nodes.is_empty() {
            return Err(AppError::new("tree is empty"));
        }
        let mut current_node_idx = 0;
        let reference = gen_solved_ref(self.nodes[0].state.dim);

        let mut counter: usize = 0;
        // A* algorithm loop
        loop {
            // signal check
            if running.load(Ordering::SeqCst) {
                println!("{}", self.nodes[current_node_idx]);
                exit(1);
            }

            counter += 1;
            // solved check
            if self.nodes[current_node_idx].state.is_solved(&reference) {
                self.solved_node = Some(current_node_idx);
                return Ok(());
            }

            // children generation of current
            self.generate_children(current_node_idx);
            self.closelist
                .insert(self.nodes[current_node_idx].state.clone());
            self.push_to_openlist(current_node_idx);

            // pick the new current
            current_node_idx = self.pick_best_option()?;

            if debug && counter % 1000 == 0 {
                println!(
                    "dedug mode.. Display current node every 1000 step:\n{}",
                    self.nodes[current_node_idx]
                );
            }
        }
    }

    pub fn display_solution(&self) {
        match self.solved_node {
            Some(mut idx) => {
                let mut path = Vec::new();
                while idx != 0 {
                    path.push(idx);
                    idx = self.nodes[idx].parent;
                }
                for i in path.into_iter().rev() {
                    println!("{}", self.nodes[i]);
                }
                println!("{} {}", "State explored:".bold(), self.closelist.len());
            }
            None => (),
        }
    }
}

// Node
pub struct Node {
    pub state: Puzzle,
    pub id: usize,

    pub heuristic: EHeuristic,
    pub h_cost: usize,
    pub parent: usize,

    pub children: Vec<usize>,
    is_children_generated: bool,
}

impl Node {
    fn new(state: Puzzle, id: usize, heuristic: EHeuristic, parent: Option<usize>) -> Self {
        let mut node = Node {
            state,
            id,
            heuristic,
            h_cost: 0,
            parent: parent.unwrap_or(0),
            children: Vec::with_capacity(1000),
            is_children_generated: false,
        };
        node.calculate_cost();
        node
    }

    fn calculate_cost(&mut self) {
        self.h_cost = self.heuristic.execute(&self.state);
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
            if self.h_cost == 0 {
                self.h_cost.to_string().green()
            } else if self.h_cost < 10 {
                self.h_cost.to_string().yellow()
            } else {
                self.h_cost.to_string().red()
            },
            "Total:".bold(),
            (self.h_cost + self.state.mouv_count).to_string().blue(),
        )?;
        write!(f, "{}", self.state)
    }
}
