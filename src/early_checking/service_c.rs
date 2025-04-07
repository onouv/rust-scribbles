use actix::prelude::*;
use log::trace;

use super::messages::{CheckReq, CheckResp, ServiceReq, ServiceResult};

pub struct ServiceC {
    can_do: bool,
}

impl Actor for ServiceC {
    type Context = Context<Self>;
}

impl ServiceC {
    pub fn new() -> Self {
        Self {
            can_do: true,
        }
    }

    fn can_do(&mut self) -> bool {
        // self.can_do = !self.can_do;
        self.can_do
    }
}

impl Handler<CheckReq> for ServiceC {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: CheckReq, _ctx: &mut Self::Context) -> Self::Result {
        trace!("Service C processing CheckReq...");

        if self.can_do() {
            trace!("Service C: can provide service.");
            let _ = msg.reply_with.send(CheckResp { can_do: true});
        } else {
            trace!("Service C: cannot provide service !");
            let _ = msg.reply_with.send(CheckResp { can_do: false});
        }

        // since we reply with the recipient delivereed in the msg, we always just return ()
        return Box::pin(async { () }.into_actor(self));
    }
}

impl Handler<ServiceReq> for ServiceC {
    type Result = Result<ServiceResult, String>;
    fn handle(&mut self, msg: ServiceReq, _ctx: &mut Self::Context) -> Self::Result {
        trace!("Service C processing ServiceReq: {}", msg.data);
        
        // this Service never fails...
        Ok(ServiceResult { 
            result: format!("{}: Service C", msg.data)
        })
            
    }
}