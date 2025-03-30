use std::time::Duration;

use actix::prelude::*;
use tokio::time::sleep;

use crate::ping::Ping;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Pong{
    pub count: u128
}

pub struct PongActor {
    ping: Recipient<Pong>
}

impl PongActor {
    pub fn new(ping: Recipient<Pong>) -> Self {
        Self {
            ping
        }
    }
}

impl Actor for PongActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Pong started ...");
    }
}

impl Handler<Ping> for PongActor {
    type Result = MessageResult<Ping>;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        let count = msg.count + 1;

        println!("Pong: received {:?}", msg);
        let ping = self.ping.clone();

        let fut = Box::pin(async move {
            sleep(Duration::from_secs(1)).await;
            ping.do_send(Pong { count });
        });

        ctx.spawn(fut.into_actor(self)); 

        MessageResult(())
    }
}