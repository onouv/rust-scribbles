mod early_checking;
use early_checking::Service;

#[actix::main]
async fn main() {
    
    let service = Service::new().await;
    
    let result= service.start_chain().await;
    match result {
        Ok(s) => {
            println!("{}", s);
        },
        Err(err) => {
            println!("{}", err);
        }
    }
}
    //tokio::signal::ctrl_c().await.unwrap();

