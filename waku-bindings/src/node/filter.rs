//! Waku filter protocol related methods

// std
use std::ffi::CString;
// internal
use crate::general::contenttopic::WakuContentTopic;
use crate::general::libwaku_response::{handle_no_response, LibwakuResponse};
use crate::general::pubsubtopic::PubsubTopic;
use crate::general::Result;
use crate::handle_ffi_call;
use crate::node::context::WakuNodeContext;

pub(crate) async fn waku_filter_subscribe(
    ctx: &WakuNodeContext,
    pubsub_topic: PubsubTopic,
    content_topics: Vec<WakuContentTopic>,
) -> Result<()> {
    let pubsub_topic =
        CString::new(pubsub_topic).expect("CString should build properly from pubsub topic");
    let content_topics = WakuContentTopic::join_content_topics(content_topics);
    let content_topics =
        CString::new(content_topics).expect("CString should build properly from content topic");

    handle_ffi_call!(
        waku_sys::waku_filter_subscribe,
        handle_no_response,
        ctx.get_ptr(),
        pubsub_topic.as_ptr(),
        content_topics.as_ptr()
    )
}

pub(crate) async fn waku_filter_unsubscribe(
    ctx: &WakuNodeContext,
    pubsub_topic: PubsubTopic,
    content_topics: Vec<WakuContentTopic>, // comma-separated list of content topics
) -> Result<()> {
    let pubsub_topic =
        CString::new(pubsub_topic).expect("CString should build properly from pubsub topic");
    let content_topics = WakuContentTopic::join_content_topics(content_topics);
    let content_topics =
        CString::new(content_topics).expect("CString should build properly from content topic");

    handle_ffi_call!(
        waku_sys::waku_filter_unsubscribe,
        handle_no_response,
        ctx.get_ptr(),
        pubsub_topic.as_ptr(),
        content_topics.as_ptr()
    )
}

pub(crate) async fn waku_filter_unsubscribe_all(ctx: &WakuNodeContext) -> Result<()> {
    handle_ffi_call!(
        waku_sys::waku_filter_unsubscribe_all,
        handle_no_response,
        ctx.get_ptr()
    )
}
