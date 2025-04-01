
use actix::prelude::*;

use super::messages::{CheckCanDoReq, CheckCanDoResp, Config};

pub struct ServiceA {
    upstream_check: Option<Recipient<CheckCanDoResp>>,
    downstream_check: Option<Recipient<CheckCanDoReq>>,
    can_do: bool
}

impl Actor for ServiceA {
    type Context = Context<Self>;
}

impl ServiceA {
    pub fn new() -> Self {
        Self {
            upstream_check: None,
            downstream_check: None,
            can_do: true
        }
    }

    fn can_do(&mut self) -> bool {
        self.can_do = !self.can_do;
        
        self.can_do
    }
}

impl Handler<CheckCanDoReq> for ServiceA {
    type Result = ();

    fn handle(&mut self, msg: CheckCanDoReq, ctx: &mut Self::Context) -> Self::Result {
        if self.can_do() {
            
        }
    }
}

impl Handler<CheckCanDoResp> for ServiceA {
    type Result = ();

    fn handle(&mut self, msg: CheckCanDoResp, ctx: &mut Self::Context) -> Self::Result {
        
    }
}

impl Handler<Config> for ServiceA {
    type Result = ();

    fn handle(&mut self, msg: Config, ctx: &mut Self::Context) -> Self::Result {

        println!("ServiceA configured.");

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