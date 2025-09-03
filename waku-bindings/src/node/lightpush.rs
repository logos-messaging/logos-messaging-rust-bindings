//! Waku lightpush protocol related methods

// std
use std::ffi::CString;
// internal
use crate::general::libwaku_response::{handle_response, LibwakuResponse};
use crate::general::{messagehash::MessageHash, Result, WakuMessage};
use crate::handle_ffi_call;
use crate::node::context::WakuNodeContext;

use crate::general::pubsubtopic::PubsubTopic;

pub(crate) async fn waku_lightpush_publish_message(
    ctx: &WakuNodeContext,
    message: &WakuMessage,
    pubsub_topic: PubsubTopic,
) -> Result<MessageHash> {
    let message = CString::new(
        serde_json::to_string(&message)
            .expect("WakuMessages should always be able to success serializing"),
    )
    .expect("CString should build properly from the serialized waku message");

    let pubsub_topic =
        CString::new(pubsub_topic).expect("CString should build properly from pubsub topic");

    handle_ffi_call!(
        waku_sys::waku_lightpush_publish,
        handle_response,
        ctx.get_ptr(),
        pubsub_topic.as_ptr(),
        message.as_ptr()
    )
}
