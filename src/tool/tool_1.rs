

use std::{collections::VecDeque, marker::PhantomData};

use super::part::*;

pub struct Stopped;
pub struct Idle;
pub struct Running;

pub struct Tool<State = Stopped> {
    inport: VecDeque<Part>,
    process: Vec<Part>,
    outport: VecDeque<Part>,
    state: std::marker::PhantomData<State>
}

impl Tool<Stopped> {

    pub fn start(self) -> Tool<Idle> {
        Tool {
            inport: self.inport,
            process: self.process,
            outport: self.outport,
            state: PhantomData
        }
    }
}

impl Tool<Idle> {
    pub fn stop(self) -> Tool<Stopped> {
        Tool {
            inport: self.inport,
            process: self.process,
            outport: self.outport,
            state: PhantomData
        }
    }

    pub fn load_inport(mut self, part: Part) -> Tool {
        self.inport.push_front(part);
        self.process.push(self.inport.pop_back().unwrap());

        Tool {
            inport: self.inport,
            process: self.process,
            outport: self.outport,
            state: PhantomData
        }
    }
}

impl Tool<Running> {
    
}

impl Tool {
    pub fn new() -> Self {
        Tool {
            inport: VecDeque::new(),
            process: vec![],
            outport: VecDeque::new(),
            state: PhantomData
        }
    }
}
