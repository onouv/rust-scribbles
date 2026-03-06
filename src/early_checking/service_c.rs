use super::messages::{CheckReq, CheckResp, ServiceReq, ServiceResult};
use actix::prelude::*;
use log::{error, trace};
use std::env;

pub struct ServiceC {}

impl Actor for ServiceC {
    type Context = Context<Self>;
}

impl ServiceC {
    pub fn new() -> Self {
        Self {}
    }

    fn can_do(&mut self) -> bool {
        match env::var("FAILING_SERVICE") {
            Ok(val) => !matches!(val.as_str(), "C"),
            _ => true,
        }
    }
}

impl Handler<CheckReq> for ServiceC {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: CheckReq, _ctx: &mut Self::Context) -> Self::Result {
        trace!("Service C processing CheckReq...");

        if self.can_do() {
            trace!("Service C: can provide service.");
            let _ = msg.reply_with.send(CheckResp { can_do: true });
        } else {
            error!("Service C: cannot provide service for my own reasons.");
            let _ = msg.reply_with.send(CheckResp { can_do: false });
        }

        // since we reply with the recipient delivereed in the msg, we always just return ()
        Box::pin(async { }.into_actor(self))
    }
}

impl Handler<ServiceReq> for ServiceC {
    type Result = Result<ServiceResult, String>;
    fn handle(&mut self, msg: ServiceReq, _ctx: &mut Self::Context) -> Self::Result {
        trace!("Service C processing ServiceReq: {}", msg.data);

        // this Service never fails...
        Ok(ServiceResult {
            result: format!("{}: Service C", msg.data),
        })
    }
}
