use actix::prelude::*;

mod ping;
use ping::{ PingActor, PingCfg };

mod pong;
use pong::*;

#[actix::main]
async fn main() {
    let ping = PingActor::new().start();
    let pong = PongActor::new(ping.clone().recipient()).start();

    _ = ping.send(PingCfg { pong: pong.clone().recipient() }).await;

    tokio::signal::ctrl_c().await.unwrap();

}
