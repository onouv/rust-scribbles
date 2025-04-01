use std::{future::Future, process::Output};

use actix::prelude::*;

use super::messages::{CheckReq, CheckResp, Config, ServiceReq, SourceConfig};

pub struct Controller {
    downstream_check: Option<Recipient<CheckReq>>,
    downstream_service: Option<Recipient<ServiceReq>>
}

impl Actor for Controller {
    type Context = Context<Self>;
}

impl Controller {
    pub fn new() -> Self {
        Self { downstream_check: None, downstream_service: None }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Start { }

impl Handler<ServiceReq> for Controller {
    type Result = ();

    fn handle(&mut self, _msg: Start, ctx: &mut Self::Context) -> Self::Result {
        println!("Controller received Start message.");

        let check = self.downstream_check.clone().unwrap();
        let service = self.downstream_service.clone().unwrap();

        // Initiate the check
        let check_future = check.send(CheckReq);

        // Handle the response
        let future = async move {
            match check_future.await {
                Ok(result) => {
                    println!("Controller: Received CheckCanDoResp");
                    if result.can_do {
                        // Start the service chain if all services can proceed
                        service.do_send(ServiceReq { data: "Start".to_string() });
                        return ();
                    }
                }
                Err(e) => {
                    println!("Controller: Error during CheckCanDoReq: {}", e);
                }
            }
        };

        // Execute the future
        ctx.spawn(future.into_actor(self));
    }
}

impl Handler<SourceConfig> for Controller {
    type Result = ();

    fn handle(&mut self, msg: SourceConfig, ctx: &mut Self::Context) -> Self::Result {
        self.downstream_check = Some(msg.downstream_check);
        self.downstream_service = Some(msg.downstream_service);
        println!("Controller received Setup message.");
    }
}