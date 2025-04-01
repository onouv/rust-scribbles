use actix::prelude::*;

use super::messages::{CheckReq, CheckResp, Config};

pub struct ServiceB {
    upstream_check: Option<Recipient<CheckResp>>,
    downstream_check: Option<Recipient<CheckReq>>,
    can_do: bool,
}

impl Actor for ServiceB {
    type Context = Context<Self>;
}

impl ServiceB {
    pub fn new() -> Self {
        Self {
            upstream_check: None,
            downstream_check: None,
            can_do: true,
        }
    }

    fn can_do(&mut self) -> bool {
        self.can_do = !self.can_do;
        self.can_do
    }
}

impl Handler<CheckReq> for ServiceB {
    type Result = ResponseActFuture<Self, (), ()>;

    fn handle(&mut self, msg: CheckReq, ctx: &mut Self::Context) -> Self::Result {
        println!("ServiceB received CheckCanDoReq");
        let downstream = self.downstream_check.clone();
        let upstream = self.upstream_check.clone();
        let can_do_self = self.can_do();

        Box::pin(async move {
            match downstream {
                Some(rcp) => {
                    println!("ServiceB: Forwarding CheckCanDoReq to downstream.");
                    rcp.send(msg).await.unwrap();
                }
                None => {
                    println!("ServiceB: No downstream, responding to upstream.");
                    match upstream {
                        Some(rcp) => {
                            let resp = CheckResp { can_do: can_do_self };
                            rcp.send(resp).await.unwrap();
                        }
                        None => {
                            println!("ServiceB: No upstream to respond to.");
                        }
                    }
                }
            }
            Ok(())
        })
    }
}

impl Handler<CheckResp> for ServiceB {
    type Result = ();

    fn handle(&mut self, msg: CheckResp, ctx: &mut Self::Context) -> Self::Result {
        println!("ServiceB received CheckCanDoResp: {}", msg.can_do);
    }
}

impl Handler<Config> for ServiceB {
    type Result = ();

    fn handle(&mut self, msg: Config, ctx: &mut Self::Context) -> Self::Result {
        println!("ServiceB configured.");

        match msg.downstream_check {
            Some(rcp) => {
                self.downstream_check = Some(rcp);
            }
            _ => {}
        }

        match msg.upstream_check {
            Some(rcp) => {
                self.upstream_check = Some(rcp);
            }
            _ => {}
        }
    }
}