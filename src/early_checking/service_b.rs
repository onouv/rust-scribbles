
use actix::prelude::*;

use super::messages::{CheckCanDoReq, CheckCanDoResp, Config};

pub struct ServiceB {
    upstream_check: Option<Recipient<CheckCanDoResp>>,
    downstream_check: Option<Recipient<CheckCanDoReq>>
}

impl Actor for ServiceB {
    type Context = Context<Self>;
}

impl ServiceB {
    pub fn new() -> Self {
        Self {
            upstream_check: None,
            downstream_check: None
        }
    }
}

impl Handler<CheckCanDoReq> for ServiceB {
    type Result = ();

    fn handle(&mut self, msg: CheckCanDoReq, ctx: &mut Self::Context) -> Self::Result {
        
    }
}


impl Handler<CheckCanDoResp> for ServiceB {
    type Result = ();

    fn handle(&mut self, msg: CheckCanDoResp, ctx: &mut Self::Context) -> Self::Result {
        
    }
}


impl Handler<Config> for ServiceB {
    type Result = ();

    fn handle(&mut self, msg: Config, ctx: &mut Self::Context) -> Self::Result {

        println!("ServiceB configured.");

        match msg.downstream_check {
            Some(rcp) => {
                self.downstream_check = Some(rcp);
            },
            _ => {}
            
        }

        match msg.upstream_check {
            Some(rcp) => {
                self.upstream_check = Some(rcp);
            },
            _ => {}
        }
    }
}