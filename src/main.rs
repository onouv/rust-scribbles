
mod tool;
use tool::*;

// mod state_2;
// use state_2::state_machine;

fn main() {
    let tool = Tool::new("1");
    println!("{}", tool);

    let tool = tool.handle(Signal::Start);
    println!("{}", tool);
    
    let tool = tool.handle(Signal::LoadPart(Part {}));
    println!("{}", tool);
    let tool = tool.handle(tool::signal::Signal::LoadPart(Part {}));
    println!("{}", tool);
    let tool = tool.handle(tool::signal::Signal::LoadPart(Part {}));
    println!("{}", tool);
    // let tool = tool.stop

    // state_machine(12);
}