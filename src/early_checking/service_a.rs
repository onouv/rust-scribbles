
use actix::prelude::*;
use super::messages::{CheckReq, CheckResp, InlineConfig, ServiceReq, ServiceResult};

pub struct ServiceA {
    downstream_check: Option<Recipient<CheckReq>>,
    downstream_service: Option<Recipient<ServiceReq>>,
    can_do: bool,
}

impl Actor for ServiceA {
    type Context = Context<Self>;
}

impl ServiceA {
    pub fn new() -> Self {
        Self {
            downstream_check: None,
            downstream_service: None,
            can_do: true,
        }
    }

    fn can_do(&mut self) -> bool {
        self.can_do
    }
}

impl Handler<CheckReq> for ServiceA {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: CheckReq, _ctx: &mut Self::Context) -> Self::Result {
        println!("Service A processing CheckReq...");
        if !self.can_do() {
            println!("Service A: cannot provide service for my own reasons.");

            // Send the response back via the channel
            let _ = msg.reply_with.send(CheckResp { can_do: false });

            return Box::pin(async { () }.into_actor(self));
        }

        println!("Service A: Forwarding CheckReq to downstream...");
        let downstream = self.downstream_check.clone();
        Box::pin(
            async move {
                match downstream {
                    Some(rcp) => {
                        // Create a channel to receive the CheckResp
                        let (tx, rx) = tokio::sync::oneshot::channel::<CheckResp>();
                        let check_future = rcp.send(CheckReq { reply_with: tx});
                    
                        match check_future.await {
                            Ok(_) => {
                                match rx.await {
                                    Ok(check_response) => {
                                        if check_response.can_do {
                                            let _ = msg.reply_with.send(check_response);
                                            return ;
                                        }
                                        let _ = msg.reply_with.send(CheckResp { can_do: false });
                                    },
                                    Err(error) => {
                                        println!("Service A: Error, Failed to receive response while checking service chain: {:?}", error);
                                    }
                                }
                            },
                            Err(error) => {
                                println!("Service A: Error, could not send request to check service chain: {}", error);
                            }
                        }
                    },
                    None => {
                        println!("ServiceA: No downstream, cannot provide service.");
                        
                        // Send the response back via the channel
                        let _ = msg.reply_with.send(CheckResp { can_do: false });
                    }
                }
            }.into_actor(self)
        )        
    }
}

impl Handler<ServiceReq> for ServiceA {
    type Result = ResponseActFuture<Self, Result<ServiceResult, String>>;

    fn handle(&mut self, msg: ServiceReq, _ctx: &mut Self::Context) -> Self::Result {
        println!("Service A: received ServiceReq: {}", msg.data);
        let service = self.downstream_service.clone();

        Box::pin(
            async move {
                match service {
                    Some(service) => {
                        let service_req = ServiceReq {
                            data: format!("{}: ServiceA", msg.data),
                        };
                        match service.send(service_req).await.unwrap() {
                            Ok(res) => {
                                Ok(res)
                            },
                            Err(err) => {
                                Err(format!("Error: ServiceA received downstream {}.", err))
                            }
                        }
                    }
                    None => Err(format!("Error: ServiceA has no downstream service.")),
                }
            }
            .into_actor(self),
        )
    }
}

impl Handler<InlineConfig> for ServiceA {
    type Result = ();

    fn handle(&mut self, msg: InlineConfig, _ctx: &mut Self::Context) -> Self::Result {
        self.downstream_check = Some(msg.downstream_check);
        self.downstream_service = Some(msg.downstream_service);
        println!("ServiceA configured.");
    }
}