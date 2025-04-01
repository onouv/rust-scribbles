use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "CheckCanDoResp")]
pub struct CheckCanDoReq;

#[derive(Message)]
#[rtype(result = "()")]
pub struct CheckCanDoResp { can_do: bool }


#[derive(Message)]
#[rtype(result = "()")]
pub struct Config {
    pub upstream_check: Option<Recipient<CheckCanDoResp>>,
    pub downstream_check: Option<Recipient<CheckCanDoReq>>
}