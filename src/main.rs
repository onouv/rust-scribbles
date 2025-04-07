mod early_checking;
use early_checking::Service;
use log::{error, info};

#[actix::main]
async fn main() {
    
    env_logger::init();

    let service = Service::new().await;
    
    let result= service.start_chain().await;
    match result {
        Ok(s) => {
            info!("{}", s);
        },
        Err(err) => {
            error!("There has been an error. {err}");
        }
    }
}
    //tokio::signal::ctrl_c().await.unwrap();

