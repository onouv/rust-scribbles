use actix::prelude::*;

use super::messages::{CheckCanDoReq, CheckCanDoResp, Config};

pub struct Controller {
    downstream_check: Option<Recipient<CheckCanDoReq>>
}

impl Actor for Controller {
    type Context = Context<Self>;
}

impl Handler<CheckCanDoReq> for Controller {
    type Result = MessageResult<CheckCanDoReq>;

    fn handle(&mut self, msg: CheckCanDoReq, ctx: &mut Self::Context) -> Self::Result {
        match self.downstream_check {
            Some(recp) => {
                recp.send(msg);
            },
            None => {}
        }
    }
}


impl Handler<CheckCanDoResp> for Controller {
    type Result = ();

    fn handle(&mut self, msg: CheckCanDoResp, ctx: &mut Self::Context) -> Self::Result {
       
    }
}

impl Handler<Config> for Controller {
    type Result = ();

    fn handle(&mut self, msg: Config, ctx: &mut Self::Context) -> Self::Result {
        println!("Controller configured.");
        match msg.downstream_check {
            Some(rcp) => {
                self.downstream_check = Some(rcp);
            },
            _ => {}
            
        }
    }
}

impl Controller {
    pub fn new() -> Self {
        Self { 
            downstream_check: None
         }
    }

    pub fn setDownstreamCheck(&mut self, req: Recipient<CheckCanDoReq>) {
        self.downstream_check = Some(req);
    }
}