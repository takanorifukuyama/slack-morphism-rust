//!
//! Support for Slack Pins API methods
//!

use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::*;
use crate::ratectl::*;
use crate::*;
use futures::future::{BoxFuture, FutureExt};
use std::collections::HashSet;

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/pins.add
    ///
    pub async fn pins_add(
        &self,
        req: &SlackApiPinsAddRequest,
    ) -> ClientResult<SlackApiPinsAddResponse> {
        self.http_session_api
            .http_post("pins.add", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiPinsAddRequest {
    pub channel: SlackChannelId,
    pub timestamp: SlackTs,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiPinsAddResponse {
    pub ok: bool,
}
