
use actix::prelude::*;

use super::messages::{CheckReq, CheckResp, InlineConfig, ServiceReq, ServiceResult};

pub struct ServiceA {
    //upstream_check: Option<Recipient<CheckResp>>,
    downstream_check: Option<Recipient<CheckReq>>,
    can_do: bool,
}

impl Actor for ServiceA {
    type Context = Context<Self>;
}

impl ServiceA {
    pub fn new() -> Self {
        Self {
            //upstream_check: None,
            downstream_check: None,
            can_do: true,
        }
    }

    fn can_do(&mut self) -> bool {
        self.can_do = !self.can_do;
        self.can_do
    }
}

impl Handler<CheckReq> for ServiceA {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: CheckReq, ctx: &mut Self::Context) -> Self::Result {
        println!("ServiceA processing CheckReq...");

        let downstream = self.downstream_check.clone();
        let upstream = self.upstream_check.clone();
        let can_do_self = self.can_do();

        if !can_do_self {
            println!("ServiceA: cannot provide service.");
            if let Some(rcp) = upstream {
                let resp = CheckResp { can_do: can_do_self };
                return Box::pin(
                    async move {
                        rcp.send(resp).await.unwrap();
                    }
                    .into_actor(self),
                );
            } else {
                println!("ServiceA: No upstream to respond to.");
                return Box::pin(async {}.into_actor(self)); // Respond with an empty future
            }
        }

        Box::pin(
            async move {
                match downstream {
                    Some(rcp) => {
                        println!("ServiceA: Forwarding CheckReq to downstream.");
                        rcp.send(msg).await.unwrap();
                    }
                    None => {
                        println!("ServiceA: No downstream, cannot provide service.");
                        match upstream {
                            Some(rcp) => {
                                let resp = CheckResp { can_do: false };
                                rcp.send(resp).await.unwrap();
                            }
                            None => {
                                println!("ServiceA: No upstream to respond to.");
                                return {};  
                            }
                        }
                    }
                }
            }
            .into_actor(self),
        )
    }
}

// impl Handler<CheckResp> for ServiceA {
//     type Result = ();

//     fn handle(&mut self, msg: CheckResp, ctx: &mut Self::Context) -> Self::Result {
//         println!("ServiceA received CheckCanDoResp: {}", msg.can_do);
//     }
// }

impl Handler<ServiceReq> for ServiceA {
    type Result = ServiceResult;

    fn handle(&mut self, msg: ServiceReq, ctx: &mut Self::Context) -> Self::Result {
        
        return ServiceResult {
            result: format!("{}: ServiceA", msg.data)
        };
    }
}


impl Handler<InlineConfig> for ServiceA {
    type Result = ();

    fn handle(&mut self, msg: InlineConfig, ctx: &mut Self::Context) -> Self::Result {
        self.downstream_check = Some(msg.downstream_check);
        self.upstream_check = Some(msg.upstream_check);
        println!("ServiceA configured.");

        ()
    }
}