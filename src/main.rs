use actix::prelude::*;

mod early_checking;
use early_checking::Service;

#[actix::main]
async fn main() {
    
    let service = Service::new().await;
    _ = service.do_me_a_service().await;

    tokio::signal::ctrl_c().await.unwrap();

}
