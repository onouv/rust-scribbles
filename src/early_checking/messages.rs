use actix::prelude::*;
use tokio::sync::oneshot::Sender;
use super::controller::ServiceError;

#[derive(Message)]
#[rtype(result = "()")]
pub struct CheckReq {
    pub reply_with: Sender<CheckResp>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CheckResp { pub can_do: bool }


#[derive(Message)]
#[rtype(result = "()")]
pub struct SourceConfig {
    pub downstream_check: Recipient<CheckReq>,
    pub downstream_service: Recipient<ServiceReq>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct InlineConfig {
    pub downstream_check: Recipient<CheckReq>,
    pub downstream_service: Recipient<ServiceReq>
}

#[derive(Message)]
#[rtype(result = "Result<(), ServiceError>")]
pub struct Start {}

#[derive(Message)]
#[rtype(result = "Result<ServiceResult, String>")]
pub struct ServiceReq {
    pub data: String
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct ServiceResult {
    pub result: String
}