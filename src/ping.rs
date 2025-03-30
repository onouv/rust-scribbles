use std::time::Duration;

use actix::prelude::*;
use tokio::time::sleep;

use crate::pong::Pong;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Ping{
    pub count: u128
}

pub struct PingActor {
    pong: Option<Recipient<Ping>>
}

impl PingActor {
    pub fn new() -> Self {
        Self {
            pong: None
        }
    }
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct PingCfg {
    pub pong: Recipient<Ping>
}

impl Actor for PingActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Ping started ...");
    }
}

impl Handler<PingCfg> for PingActor {
    type Result = MessageResult<PingCfg>;
    
    fn handle(&mut self, msg: PingCfg, _ctx: &mut Self::Context) -> Self::Result {
        
        let pong = msg.pong;
        pong.do_send(Ping {
            count: 0
        });
    
        self.pong = Some(pong);
        println!("Ping configured ...");
        
        MessageResult(())
    }
}

impl Handler<Pong> for PingActor {
    type Result = ();

    fn handle(&mut self, msg: Pong, ctx: &mut Self::Context) -> Self::Result {
        println!("Ping: received {:?}", msg);

        let count = msg.count + 1;

        match self.pong.clone() {
            Some(pong) => {
                let fut = Box::pin(async move {
                    sleep(Duration::from_secs(1)).await;
                    pong.do_send(Ping { count });
                });
                ctx.spawn(fut.into_actor(self)); 

                ()
            },
            None => {
                panic!("Ping: missing pong, cannot ping pong!");
            }
        }
    }
}

