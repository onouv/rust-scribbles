use actix::prelude::*;

use super::{controller::*, messages::{CheckCanDoReq, Config}, service_a::*, service_b::*, service_c::*};

pub struct ServiceResult;

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

        _ = controller.send(Config{
            upstream_check: None,
            downstream_check: Some(service_a.clone().recipient())
        }).await;

        _ = service_a.send(Config {
            upstream_check: Some(controller.clone().recipient()),
            downstream_check: Some(service_b.clone().recipient())
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
        
        let can_do = self.controller.send(CheckCanDoReq).await;
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