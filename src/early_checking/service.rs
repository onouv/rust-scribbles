use actix::prelude::*;

use super::{controller::*, messages::{InlineConfig, SourceConfig, Start}, service_a::*, service_b::*, service_c::*};
pub use super::controller::ServiceError;

pub struct Service {
    controller: Addr<Controller>
}

impl Service {
    pub async fn new() -> Self {
        let controller = Controller::new().start();
        let service_a = ServiceA::new().start();
        let service_b = ServiceB::new().start();
        let service_c = ServiceC::new().start();

        _ = controller.send(SourceConfig{
            downstream_check: service_a.clone().recipient(),
            downstream_service: service_a.clone().recipient()
        }).await;

        _ = service_a.send(InlineConfig {
            downstream_check: service_b.clone().recipient(),
            downstream_service: service_b.clone().recipient()
        }).await;

        _ = service_b.send(InlineConfig {
            downstream_check: service_c.clone().recipient(),
            downstream_service: service_c.clone().recipient()
        }).await;

        Self { controller }
    }

    pub async fn start_chain(&self) -> Result<String, ServiceError> {
        
        let result = self.controller.send(Start{}).await;
        match result {
            Ok(res) => {
                match res {
                    Ok(()) => {
                        Ok("Actor system started.".to_string())
                    },
                    Err(error) => {
                        Err(error)
                    }
                }
            },
            Err(e) => {
                Err(ServiceError::ServiceDown(format!("{:?}", e)))
            }
        }
    }
}