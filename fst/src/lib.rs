use std::fmt;
use std::fmt::Formatter;
use wasm_bindgen::prelude::*;

use std::os::raw::c_char;

pub type NodeIndex = usize;
pub type TransitionIndex = usize;


#[wasm_bindgen]
pub struct FiniteStateMachine {
    //nameFSM: String,
    transitions: Vec<Transition>,
    nodes: Vec<Node>
}

#[wasm_bindgen]
pub struct Node {
    pub name: i32,
    outputTransition: Vec<TransitionIndex>,
    inputTransition: Vec<TransitionIndex>
}

#[wasm_bindgen]
pub struct Transition {
    pub letter: char,
    outputNodes: NodeIndex,
    inputNodes: NodeIndex,
}

#[wasm_bindgen]
impl FiniteStateMachine {
    pub fn new() -> FiniteStateMachine {
        return FiniteStateMachine {
            // nameFSM,
            transitions: Vec::new(),
            nodes: Vec::new()
        };
    }

    pub fn addNode(&mut self, name: i32) -> NodeIndex {
        let index: usize = self.nodes.len();
        self.nodes.push(Node {
            name,
            outputTransition: Vec::new(),
            inputTransition: Vec::new(),
        });
        return index;
    }

    pub fn addTransition(&mut self, letter: char, inputNodes: NodeIndex, outputNodes: NodeIndex) -> TransitionIndex {
        let index: usize = self.transitions.len();
        let new_trans = Transition {
            letter,
            outputNodes,
            inputNodes,
        };
        self.transitions.push(new_trans);

        self.nodes[inputNodes].outputTransition.push(index);
        self.nodes[outputNodes].inputTransition.push(index);
        return index;
    }


    pub fn existTransition(&mut self, transitionTest: TransitionIndex, c_char: char) -> bool {

        if self.transitions[transitionTest].letter == c_char {
            return true;
        } else {
            return false;
        }
    }

    pub fn displayNode(&self,nodeI: NodeIndex) {
        for nodeNumber in &self.nodes{
            print!("{} {}",nodeNumber.name, " ");
        }
    }

    pub fn displayTransition(&self) {

        println!();
        for transitionNumber in &self.transitions{
            println!("{} {} {} {} {}",self.nodes[transitionNumber.inputNodes].name, "-", transitionNumber.letter, "->", self.nodes[transitionNumber.outputNodes].name);
        }
    }

    pub fn displayFSM(&self) {
        println!("{}","Finite state machine display : ");

        for nodeNumber in &self.nodes{
            print!("{} {}",nodeNumber.name, " ");
        }
    }
}