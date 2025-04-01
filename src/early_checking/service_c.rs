
use actix::prelude::*;

use super::messages::{ CheckCanDoReq, CheckCanDoResp, Config};

pub struct ServiceC {
    upstream_check: Option<Recipient<CheckCanDoResp>>,
}

impl Actor for ServiceC {
    type Context = Context<Self>;
}

impl ServiceC {
    pub fn new() -> Self {
        Self {
            upstream_check: None,
        }
    }

    pub fn setUpstreamCheck(&mut self, recp: Recipient<CheckCanDoResp>) {
        self.upstream_check = Some(recp);
    }

    pub fn setDownstreamCheck(&mut self, req: Recipient<CheckCanDoReq>) {
        self.downstream_check = Some(req);
    }
}

impl Handler<CheckCanDoReq> for ServiceC {
    type Result = ();

    fn handle(&mut self, msg: CheckCanDoReq, ctx: &mut Self::Context) -> Self::Result {
        
    }
}

impl Handler<Config> for ServiceC {
    type Result = ();

    fn handle(&mut self, msg: Config, ctx: &mut Self::Context) -> Self::Result {

        println!("Servicec configured.");

        match msg.upstream_check {
            Some(rcp) => {
                self.upstream_check = Some(rcp);
            },
            _ => {}
        }
    }
}