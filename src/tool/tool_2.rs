use std::collections::VecDeque;
use std::fmt::{self, Display};

use super::part::Part;
use super::signal::Signal;

struct ToolEntity {
    id: String,
    inport: VecDeque<Part>,
    process: Vec<Part>,
    outport: VecDeque<Part>,
}

impl ToolEntity {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            inport: VecDeque::default(),
            process: vec![],
            outport: VecDeque::default(),
        }
    }
}

// In each state, a tool can do only certain operations, each of which are identical, between 
// the different states, though. This is modelled as traits with default implementations. 

trait Inport {
    fn load_inport(&mut self,  part: Part) {
        let mut entity = self.entity();
        entity.inport.push_front(part);
    }

    // unfortunately, Rust does not allow non-public functions in traits as in abstract classes,
    // so we have to expose this within this module (!!), so each state can provide its internal 
    // entity instance. 
    fn entity(&self) -> ToolEntity;             
    
}

// not possible: type state pattern with PhantomData forces a single type return from a method, 
// so methods can not model conditional transitions to other state types 
//
// struct Tool<State = Stopped> {
//     state: std::marker::PhantomData<State>
// }

// So we model the state types explicitly. We keep the actual data and low-level behavior in a 
// separate instance ( here of type ToolEntity) and pass is into tuple structs representing 
// the different states of the tool.
struct Stopped(ToolEntity); // nice and short
impl Stopped {
    pub fn new(entity: ToolEntity) -> Self {

        Stopped (entity)
    }
}

struct Idle(ToolEntity);    

// PROBLEM HERE 
// impl Inport for Idle {
//     fn entity(&self) -> ToolEntity {
//         self.0  // BANG: "cannot move out of `self` which is behind a shared reference
//                 // move occurs because `self.0` has type `ToolEntity`, which does not 
//                 // implement the `Copy` trait". 
//                 // 
//                 // Well, I friggin don't want to copy, it's a big fat struct with 
//                 // lots of heap data !
//     }
// }

impl Idle {
    pub fn new(entity: ToolEntity) -> Self {
        Idle (entity)
    }

    // so we have to repeat all this down here in different state type ?!  C'mon, Bro !
    // Plus, we cannot neatly group these things in traits !

    // pub fn load_inport(&mut self, part: Part) {
    //     self.0.inport.push_front(part);
    // }

    // pub fn unload_outport(&mut self) -> Option<Part> {
    //     self.0.outport.pop_back()
    // }
}

struct Running(ToolEntity); 
impl Running {
    pub fn new(entity: ToolEntity) -> Self {
        Running(entity)
    }
}

struct Faulted(ToolEntity); 

impl Faulted {
    pub fn new(entity: ToolEntity) -> Self {
        Faulted(entity)
    }
}

//e machine acts on the ToolEntity instance accordingly. 
//
pub enum Tool {
    Stopped(Stopped),
    Idle(Idle),
    Running(Running),
    Fault(Faulted)
}
impl Tool {

    /// constructor for the initial state
    pub fn new(id: &str) -> Self {
        Tool::Stopped(Stopped::new(ToolEntity::new(id)))
    }

    /// All possible state transitions, triggered by signals
    pub fn handle(self, signal: Signal) -> Self {
        match (self, signal) {
            (Tool::Stopped(tool), Signal::Start) => {
                Tool::Idle(Idle::new(tool.0))
            },
            (Tool::Idle(mut tool), Signal::LoadPart(part)) => {
                load_inport(&mut tool.0, part);
                shift_part_to_process(&mut tool.0);

                Tool::Running(Running::new(tool.0))
            }
            (Tool::Running(mut tool), Signal::LoadPart(part)) => {
                load_inport(&mut tool.0, part);
                

                Tool::Running(Running::new(tool.0))
            }
            _ => todo!()
        }
    }
}


impl Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tool::Stopped(tool) => {
                write!(f, "{} : Stopped, {} parts total", tool.0.id, calc_parts_total(&tool.0))
            },
            Tool::Idle(tool) => {
                write!(f, "{} : Idle, {} parts total", tool.0.id, calc_parts_total(&tool.0))
            },
            Tool::Running(tool) => {
                write!(f, "{} : Running, {} parts total", tool.0.id, calc_parts_total(&tool.0))
            },
            Tool::Fault(tool) => {
                write!(f, "{} : Fault, {} parts total", tool.0.id, calc_parts_total(&tool.0))
            }
        }
        
    }
}


// private functions to implement the actual actions required for the state transitions, 
// acting on the ToolEntity instance passed in from the state tuple structs.
//
// Note these methods trust on consistent internal state of the tool member fields,
// as they are only called from the state machine, which is the only one to have access to
// the ToolEntity instance.

fn shift_part_to_process(tool: &mut ToolEntity) {
    tool.process.push(tool.inport.pop_back().unwrap());
}

fn load_inport(tool: &mut ToolEntity, part: Part) {
    tool.inport.push_front(part);
}

fn calc_parts_total(tool: &ToolEntity) -> usize {
    tool.inport.len() + tool.process.len() + tool.outport.len()
}