use std::fmt::Display;

use actix::prelude::*;
use super::messages::{CheckReq, CheckResp, ServiceReq, SourceConfig, Start};

pub enum ServiceError {
    ServiceDown(String),
    ServiceBlocked(String),
    // Unknown
}

impl Display for ServiceError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ServiceError::Unknown => {
            //     println!("Error: Unknown Error");
            // },
            ServiceError::ServiceBlocked(s) => {
                println!("Error: Service Blocked -> {}", s);
            },
            ServiceError::ServiceDown(s) => {
                println!("Error: Service Down -> {}", s);
            }
        }

        Ok(())
    }
}

pub struct Controller {
    downstream_check: Option<Recipient<CheckReq>>,
    downstream_service: Option<Recipient<ServiceReq>>,
}

impl Actor for Controller {
    type Context = Context<Self>;
}

impl Controller {
    pub fn new() -> Self {
        Self {
            downstream_check: None,
            downstream_service: None,
        }
    }
}

impl Handler<Start> for Controller {
    type Result = ResponseActFuture<Self, Result<(), ServiceError>>;

    fn handle(&mut self, _msg: Start, _ctx: &mut Self::Context) -> Self::Result {
        println!("Controller: processing Start message...");

        let service = self.downstream_service.clone().unwrap();

        // Create a channel to receive the CheckResp
        let (tx, rx) = tokio::sync::oneshot::channel::<CheckResp>();
        
        // Initiate the check
        println!("Controller: initiating check chain...");
        let check = self.downstream_check.clone().unwrap();
        let check_future = check.send(CheckReq { reply_with: tx });

        Box::pin(
            async move {
                match check_future.await {
                    Ok(_) => {
                        // Await the CheckResp from the channel
                        match rx.await {
                            Ok(result) => {
                                if result.can_do {
                                    // Start the service chain if all services can proceed
                                    println!("Controller: initiating service chain...");

                                    let result = service.send(ServiceReq {
                                        data: "Start".to_string(),
                                    }).await;

                                    match result {
                                        Ok(res) => {
                                            match res {
                                                Ok(svc_res) => {
                                                    println!("Controller: Start result: {:?}", svc_res.result);
                                                    return Ok(());
                                                },
                                                Err(err) => {
                                                    return Err(ServiceError::ServiceDown(format!("Error: Controller {:?}", err)));
                                                }
                                            }
                                        },
                                        Err(error) => {
                                            return Err(ServiceError::ServiceDown(format!("Error: Controller {:?}", error)));
                                        }
                                    }
                                } else {
                                    return Err(ServiceError::ServiceBlocked(
                                        "Controller: Cannot do service. Downstream chain is blocked.".to_string()));
                                }
                            },
                            Err(e) => {
                                return Err(ServiceError::ServiceDown(format!("Controller: Error: {:?}", e)))
                            }
                        }
                    }
                    Err(e) => {
                        return Err(ServiceError::ServiceDown(format!(
                            "Controller: Error, could not send request to check service chain: {}", e)));
                    }
                }
            }
            .into_actor(self),
        )
    }
}

impl Handler<SourceConfig> for Controller {
    type Result = ();

    fn handle(&mut self, msg: SourceConfig, _ctx: &mut Self::Context) -> Self::Result {
        self.downstream_check = Some(msg.downstream_check);
        self.downstream_service = Some(msg.downstream_service);
        println!("Controller: configured.");
    }
}