use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "CheckCanDoResp")]
pub struct CheckReq;

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
    pub upstream_check: Recipient<CheckResp>,
    pub downstream_check: Recipient<CheckReq>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SinkConfig {
    pub upstream_check: Recipient<CheckResp>
}

#[derive(Message)]
#[rtype(result = "ServiceResult")]
pub struct ServiceReq {
    pub data: String
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct ServiceResult {
    pub result: String
};