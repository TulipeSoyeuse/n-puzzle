use std::fmt::Display;

use crate::{
    algorithm::heuristics::{self, PContainer},
    arena::{Mouvement, Puzzle},
};

// Tree
pub struct Tree<'a> {
    root: Node<'a>,
    openlist: Vec<&'a Node<'a>>,
}

impl<'a> Tree<'a> {
    pub fn new(
        state: Puzzle,
        heuristic: heuristics::EHeuristic,
        reference: &'a PContainer,
    ) -> Self {
        Tree {
            root: Node::new(state, heuristic, reference),
            openlist: Vec::new(),
        }
    }

    pub fn solve_puzzle(&mut self) -> &Puzzle {
        self.root.generate_child();

        let current_pick = &self.root;
        while current_pick

    }
}

impl Display for Tree<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)?;
        Ok(())
    }
}

// Node
pub struct Node<'a> {
    pub state: Puzzle,
    pub reference: &'a PContainer,
    pub heuristic: heuristics::EHeuristic,
    pub cost: u32,
    pub children: Vec<Node<'a>>,
}
impl<'a> Node<'a> {
    fn new(state: Puzzle, heuristic: heuristics::EHeuristic, reference: &'a PContainer) -> Self {
        Node {
            state,
            heuristic,
            cost: 0,
            children: Vec::new(),
            reference,
        }
    }

    // push a new Option<child> onto the child list
    fn new_child(&mut self, state: Puzzle) {
        let child = Node::new(state, self.heuristic.clone(), self.reference);
        self.children.push(child)
    }

    // generate child Option<child,()> if mouv is possible and return it
    pub fn generate_child(&mut self) {
        for i in [
            self.state.clone_left(),
            self.state.clone_up(),
            self.state.clone_right(),
            self.state.clone_down(),
        ] {
            // match the result of the puzzle creation to create associated child
            match i {
                Ok(puzzle) => self.new_child(puzzle),
                Err(()) => (),
            }
        }

        // calculate heuristic for each child
        for child in self.children.iter_mut() {
            child.cost =
                heuristics::set_heuristics(&child.heuristic, &child.state, child.reference);
        }
    }
}

impl Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Puzzle Node")?;
        write!(f, "{}", self.state)?;
        writeln!(f, "{}", "--------------------".repeat(3))?;
        writeln!(f, "cost: {}", self.cost)?;
        writeln!(f, "number of children: {}", self.children.len())?;
        Ok(())
    }
}
