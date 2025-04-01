use actix::prelude::*;

use super::{controller::*, messages::{CheckReq, InlineConfig, ServiceReq, SourceConfig}, service_a::*, service_b::*, service_c::*};


pub enum ServiceError {
    ServiceDown(String),
    ServiceBlocked(String),
    Unknown
}

pub struct Service {
    controller: Addr<Controller>,
    service_a: Addr<ServiceA>,
    service_b: Addr<ServiceB>,
    service_c: Addr<ServiceC>
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
            upstream_check: controller.clone().recipient(),
            downstream_check: service_b.clone().recipient(),
        }).await;

        _ = service_b.send(Config {
            upstream_check: Some(service_a.clone().recipient()),
            downstream_check: Some(service_c.clone().recipient())
        }).await;

        _ = service_c.send(Config {
            upstream_check: Some(service_b.clone().recipient()),
            downstream_check: None
        }).await;

        Self { controller, service_a, service_b, service_c }
    }

    pub async fn do_me_a_service(&self) -> Result<ServiceResult, ServiceError> {
        
        let result = self.controller.send(ServiceReq { data "wink"}).await;
        match can_do {
            Ok(()) => {
                println!("Actor system can do the service.");
                return Ok(ServiceResult);
            },
            Err(error) => {
                println!("Actor system cannot do the service: {:?}", error);
                return Err(ServiceError::Unknown);
            }
        }
    }
}