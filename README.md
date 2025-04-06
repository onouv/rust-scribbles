# Rust Scribbles

A  system of 4 Actix actors  
- `Controller`
- `ServiceA`
- `ServiceB`
- `ServiceC` 

which are wrapped behind a asynchronous facade `Service`.


![Overview](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/onouv/rust-scribbles/early-async-checking/doc/setup.cmp.puml)


The three service actors represent a processing chain of stateful but asynchronously independent participants which can be blocked by any one of the three members (note: the services have no state machine in this simplified example). The Controller therefore whishes to check if the chain can process a command, in our case `Start`, before sending it.

The chain is intended to be extendable, thus the controller should be oblivious to the individual parts of the chain, as long as it can access the first chain element. 

Therefore, when receiving a  `Start` message, the `Controller` runs a quick check request through the chain, and initiates the `ServiceReq` only whence the check is Ok. 


![Sequence](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/onouv/rust-scribbles/early-async-checking/doc/setup.seq.puml)


To try this example, open a shell and run 
```
git clone https://github.com/onouv/rust-scribbles.git
git checkout early-async-checking
cargo run 
```
