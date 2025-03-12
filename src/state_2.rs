use anyhow::Result;
use rand::Rng;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
struct StateA {
    x: u32,
}

#[derive(Debug, Clone, Copy)]
struct StateB {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy)]
struct StateC {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Clone, Copy)]
pub enum StateMachine {
    StateA(StateA),
    StateB(StateB),
    StateC(StateC),
}

trait Run {
    fn run(self) -> Result<StateMachine>;
}

impl Run for StateA {
    fn run(self) -> Result<StateMachine> {
        // Logic for processing in StateA
        println!("State A: {:?}", self);
        std::thread::sleep(Duration::from_secs(1));

        // For demonstration, transitioning to StateB
        let next_state = StateMachine::StateB(StateB { x: self.x, y: 0 });
        is_transition_allowed(StateMachine::StateA(self), next_state)
    }
}

impl Run for StateB {
    fn run(mut self) -> Result<StateMachine> {
        println!("State B: {:?}", self);
        std::thread::sleep(Duration::from_secs(1));
        let mut rng = rand::thread_rng();
        self.y = rng.gen_range(0..20);

        let next_state = if self.y > 10 {
            StateMachine::StateC(StateC {
                x: self.x,
                y: self.y,
                z: 0,
            })
        } else {
            StateMachine::StateA(StateA { x: self.x })
        };

        is_transition_allowed(StateMachine::StateB(self), next_state)
    }
}

impl Run for StateC {
    fn run(mut self) -> Result<StateMachine> {
        // Logic for processing in StateC
        println!("State C: {:?}", self);
        std::thread::sleep(Duration::from_secs(1));

        self.x = self.y + self.z;

        // Transitioning back to StateA for demonstration
        let next_state = StateMachine::StateA(StateA { x: self.x });
        is_transition_allowed(StateMachine::StateC(self), next_state)
    }
}

impl StateMachine {
    fn run(self) -> Result<Self> {
        match self {
            StateMachine::StateA(state) => state.run(),
            StateMachine::StateB(state) => state.run(),
            StateMachine::StateC(state) => state.run(),
        }
    }
    fn is_transition_allowed(
        current_state: StateMachine,
        next_state: StateMachine,
    ) -> Result<StateMachine> {
        match (current_state, next_state) {
            (A(_), B(_)) => Ok(next_state),
            (B(_), A(_)) => Ok(next_state),
            (B(_), C(_)) => Ok(next_state),
            (C(_), A(_)) => Ok(next_state),
            _ => Err(anyhow::anyhow!("Invalid transition")),
        }
    }
}

pub fn state_machine(x: u32) -> Result<StateMachine> {
    let mut state = StateMachine::StateA(StateA { x });

    loop {
        state = state.run()?
        // Optionally, print the current state or perform other actions here
    }
}

use StateMachine::StateA as A;
use StateMachine::StateB as B;
use StateMachine::StateC as C;

fn is_transition_allowed(
    current_state: StateMachine,
    next_state: StateMachine,
) -> Result<StateMachine> {
    match (current_state, next_state) {
        (A(_), B(_)) => Ok(next_state),
        (B(_), A(_)) => Ok(next_state),
        (B(_), C(_)) => Ok(next_state),
        (C(_), A(_)) => Ok(next_state),
        _ => Err(anyhow::anyhow!("Invalid transition")),
    }
}
