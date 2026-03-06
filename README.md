# Rust Scribbles

A  system of 4 Actix actors  
- `Controller`
- `ServiceA`
- `ServiceB`
- `ServiceC` 

which are wrapped behind an asynchronous facade `Service`.


![Overview](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/onouv/rust-scribbles/early-async-checking/doc/setup.cmp.puml)


The three service actors represent a processing chain of stateful but asynchronously independent participants which can be blocked by any one of the three members (note: the services have no state machine in this simplified example). The Controller whishes to check if the chain can process a command, in our case `Start`, before sending it.

The chain is intended to be extendable, thus the controller should be oblivious to the individual parts of the chain, as long as it can access the first chain element. 

Therefore, when receiving a  `Start` message, the `Controller` runs a quick check request through the chain, and launches the `ServiceReq` into the chain only whence the check is Ok. 


![Sequence](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/onouv/rust-scribbles/early-async-checking/doc/setup.seq.puml)

The actors can be controlled to block the precheck from the command line like so:
```
RUST_LOG=info FAILING_SERVICE=A cargo run
RUST_LOG=info FAILING_SERVICE=B cargo run
RUST_LOG=info FAILING_SERVICE=C cargo run
```

To try this happy-path example, open a shell and run 
```
git clone https://github.com/onouv/rust-scribbles.git
git checkout early-async-checking
RUST_LOG=trace cargo run 
```
The output will demonstrate the interactions of the checking and request processing phases for the happy case: 

```
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::controller] Controller: configured.
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::service_a] Service A: configured.
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::service_b] Service B: configured.
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::controller] Controller: processing Start message...
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::controller] Controller: initiating check chain...
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::service_a] Service A processing CheckReq...
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::service_a] Service A: forwarding CheckReq to downstream...
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::service_b] Service B processing CheckReq...
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::service_b] Service B: forwarding CheckReq to downstream...
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::service_c] Service C processing CheckReq...
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::service_c] Service C: can provide service.
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::controller] Controller: initiating service chain...
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::service_a] Service A: received ServiceReq: Start
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::service_b] Service B: processing ServiceReq: Start: ServiceA
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::service_c] Service C processing ServiceReq: Start: ServiceA: Service B
[2026-03-06T13:30:03Z TRACE scribbles::early_checking::controller] Controller: Start result: "Start: ServiceA: Service B: Service C"
[2026-03-06T13:30:03Z INFO  scribbles] Actor system started.

```