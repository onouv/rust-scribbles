use actix::prelude::*;

use super::messages::{CheckReq, CheckResp, Config};

pub struct ServiceC {
    upstream_check: Option<Recipient<CheckResp>>,
    can_do: bool,
}

impl Actor for ServiceC {
    type Context = Context<Self>;
}

impl ServiceC {
    pub fn new() -> Self {
        Self {
            upstream_check: None,
            can_do: true,
        }
    }

    fn can_do(&mut self) -> bool {
        self.can_do = !self.can_do;
        self.can_do
    }
}

impl Handler<CheckReq> for ServiceC {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: CheckReq, ctx: &mut Self::Context) -> Self::Result {
        let upstream = self.upstream_check.clone();
        Box::pin(actix::fut::wrap_future(async move {
            match upstream {
                Some(rcp) => {
                    let can_do = true; // Replace with actual check
                    let resp = CheckResp { can_do };
                    rcp.send(resp).await.unwrap();
                }
                None => {
                    println!("ServiceC: No upstream to respond to.");
                }
            }
            Ok(())
        }))
    }
}

impl Handler<CheckResp> for ServiceC {
    type Result = ();

    fn handle(&mut self, msg: CheckResp, ctx: &mut Self::Context) -> Self::Result {
        println!("ServiceC received CheckCanDoResp: {}", msg.can_do);
    }
}

impl Handler<Config> for ServiceC {
    type Result = ();

    fn handle(&mut self, msg: Config, ctx: &mut Self::Context) -> Self::Result {
        println!("ServiceC configured.");

        match msg.upstream_check {
            Some(rcp) => {
                self.upstream_check = Some(rcp);
            }
            _ => {}
        }
    }
}