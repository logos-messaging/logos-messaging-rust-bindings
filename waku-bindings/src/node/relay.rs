//! Waku [relay](https://rfc.vac.dev/spec/36/#waku-relay) protocol related methods

// std
use std::ffi::CString;
use std::time::Duration;
// internal
use crate::{
    general::{
        contenttopic::{Encoding, WakuContentTopic},
        libwaku_response::{handle_no_response, handle_response, LibwakuResponse},
        messagehash::MessageHash,
        Result, WakuMessage,
    },
    handle_ffi_call,
    node::context::WakuNodeContext,
};

/// Create a content topic according to [RFC 23](https://rfc.vac.dev/spec/23/)
/// As per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_content_topicchar-applicationname-unsigned-int-applicationversion-char-contenttopicname-char-encoding)
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub(crate) async fn _waku_create_content_topic(
    ctx: &WakuNodeContext,
    application_name: &str,
    application_version: u32,
    content_topic_name: &str,
    encoding: Encoding,
) -> Result<WakuContentTopic> {
    let application_name = CString::new(application_name)
        .expect("Application name should always transform to CString");
    let content_topic_name =
        CString::new(content_topic_name).expect("Content topic should always transform to CString");
    let encoding =
        CString::new(encoding.to_string()).expect("Encoding should always transform to CString");

    handle_ffi_call!(
        waku_sys::waku_content_topic,
        handle_response,
        ctx.get_ptr(),
        application_name.as_ptr(),
        application_version,
        content_topic_name.as_ptr(),
        encoding.as_ptr()
    )
}

/// Publish a message using Waku Relay
/// As per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_relay_publishchar-messagejson-char-pubsubtopic-int-timeoutms)
pub(crate) async fn waku_relay_publish_message(
    ctx: &WakuNodeContext,
    message: &WakuMessage,
    pubsub_topic: impl Into<Vec<u8>>,
    timeout: Option<Duration>,
) -> Result<MessageHash> {
    let message = CString::new(
        serde_json::to_string(&message)
            .expect("WakuMessages should always be able to success serializing"),
    )
    .expect("CString should build properly from the serialized waku message");

    let pubsub_topic =
        CString::new(pubsub_topic).expect("CString should build properly from pubsub topic");

    handle_ffi_call!(
        waku_sys::waku_relay_publish,
        handle_response,
        ctx.get_ptr(),
        pubsub_topic.as_ptr(),
        message.as_ptr(),
        timeout
            .map(|duration| {
                duration
                    .as_millis()
                    .try_into()
                    .expect("Duration as milliseconds should fit in a u32")
            })
            .unwrap_or(0)
    )
}

pub(crate) async fn waku_relay_subscribe(
    ctx: &WakuNodeContext,
    pubsub_topic: impl Into<Vec<u8>>,
) -> Result<()> {
    let pubsub_topic =
        CString::new(pubsub_topic).expect("CString should build properly from pubsub topic");

    handle_ffi_call!(
        waku_sys::waku_relay_subscribe,
        handle_no_response,
        ctx.get_ptr(),
        pubsub_topic.as_ptr()
    )
}

pub(crate) async fn waku_relay_unsubscribe(
    ctx: &WakuNodeContext,
    pubsub_topic: impl Into<Vec<u8>>,
) -> Result<()> {
    let pubsub_topic =
        CString::new(pubsub_topic).expect("CString should build properly from pubsub topic");

    handle_ffi_call!(
        waku_sys::waku_relay_unsubscribe,
        handle_no_response,
        ctx.get_ptr(),
        pubsub_topic.as_ptr()
    )
}
