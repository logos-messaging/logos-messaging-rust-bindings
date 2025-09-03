//! Waku node implementation

mod config;
mod context;
mod events;
mod filter;
mod lightpush;
mod management;
mod peers;
mod relay;
mod store;

// std
pub use aes_gcm::Key;
pub use multiaddr::Multiaddr;
pub use secp256k1::{PublicKey, SecretKey};
use std::marker::PhantomData;
use std::time::Duration;
use store::{StoreQueryRequest, StoreWakuMessageResponse};
use tokio::sync::mpsc::unbounded_channel;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::Stream;
// internal
use crate::general::contenttopic::WakuContentTopic;
use crate::general::libwaku_response::LibwakuResponse;
pub use crate::general::pubsubtopic::PubsubTopic;
use crate::general::{messagehash::MessageHash, Result, WakuMessage};

use crate::node::context::WakuNodeContext;
pub use config::RLNConfig;
pub use config::WakuNodeConfig;
pub use events::{WakuEvent, WakuMessageEvent};

// Define state marker types
pub struct Initialized;
pub struct Running;

/// Handle to the underliying waku node
pub struct WakuNodeHandle<State> {
    ctx: WakuNodeContext,
    config: WakuNodeConfig,
    _state: PhantomData<State>,
}

impl<State> WakuNodeHandle<State> {
    /// Get the nwaku version
    pub async fn version(&self) -> Result<String> {
        management::waku_version(&self.ctx).await
    }

    pub async fn waku_destroy(self) -> Result<()> {
        let res = management::waku_destroy(&self.ctx).await;
        self.ctx.reset_ptr();
        res
    }
}

impl WakuNodeHandle<Initialized> {
    /// Spawn a new Waku node with the given configuration (default configuration if `None` provided)
    /// as per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_newchar-jsonconfig)
    pub async fn new(config: Option<WakuNodeConfig>) -> Result<Self> {
        let config = config.unwrap_or_default();

        let ctx = management::waku_new(&config).await?;

        let node = Self {
            ctx,
            config,
            _state: PhantomData,
        };

        Ok(node)
    }

    /// Start a Waku node mounting all the protocols that were enabled during the Waku node instantiation.
    /// as per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_start)
    pub async fn start(self) -> Result<WakuNodeHandle<Running>> {
        management::waku_start(&self.ctx).await?;

        let running_node = WakuNodeHandle {
            ctx: self.ctx,
            config: self.config,
            _state: PhantomData,
        };

        Ok(running_node)
    }

    pub fn set_event_callback<F: FnMut(LibwakuResponse) + 'static + Sync + Send>(
        &self,
        closure: F,
    ) -> Result<()> {
        self.ctx.waku_set_event_callback(closure)
    }

    /// Return a stream of all Waku responses.
    pub fn response_stream(&self) -> impl Stream<Item = LibwakuResponse> {
        let (tx, rx) = unbounded_channel();
        let tx_clone = tx.clone();

        let callback = {
            move |event: LibwakuResponse| {
                let _ = tx_clone.send(event);
            }
        };

        if let Err(error) = self.ctx.waku_set_event_callback(callback) {
            tx.send(LibwakuResponse::Failure(error)).unwrap();
        }

        let stream = UnboundedReceiverStream::new(rx);

        stream
    }
}

impl WakuNodeHandle<Running> {
    /// Stops a Waku node
    /// as per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_stop)
    pub async fn stop(self) -> Result<WakuNodeHandle<Initialized>> {
        management::waku_stop(&self.ctx).await?;

        let init_node = WakuNodeHandle {
            ctx: self.ctx,
            config: self.config,
            _state: PhantomData,
        };

        Ok(init_node)
    }

    /// Get the multiaddresses the Waku node is listening to
    /// as per [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_listen_addresses)
    pub async fn listen_addresses(&self) -> Result<Vec<Multiaddr>> {
        management::waku_listen_addresses(&self.ctx).await
    }

    /// Dial peer using a multiaddress
    /// If `timeout` as milliseconds doesn't fit into a `i32` it is clamped to [`i32::MAX`]
    /// If the function execution takes longer than `timeout` value, the execution will be canceled and an error returned.
    /// Use 0 for no timeout
    /// As per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_connect_peerchar-address-int-timeoutms)
    pub async fn connect(&self, address: &Multiaddr, timeout: Option<Duration>) -> Result<()> {
        peers::waku_connect(&self.ctx, address, timeout).await
    }

    /// Publish a message using Waku Relay.
    /// As per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_relay_publishchar-messagejson-char-pubsubtopic-int-timeoutms)
    /// The pubsub_topic parameter is optional and if not specified it will be derived from the contentTopic.
    pub async fn relay_publish_message(
        &self,
        message: &WakuMessage,
        pubsub_topic: impl Into<PubsubTopic>,
        timeout: Option<Duration>,
    ) -> Result<MessageHash> {
        if self.config.relay.is_none_or(|enabled| !enabled) {
            //TODO add error types
            return Err(
                "Relay is disabled. Restart the waku node with Relay enabled to use this function."
                    .to_string(),
            );
        }

        relay::waku_relay_publish_message(&self.ctx, message, pubsub_topic.into(), timeout).await
    }

    /// Subscribe to receive messages matching a pubsub topic.
    pub async fn relay_subscribe(&self, pubsub_topic: impl Into<PubsubTopic>) -> Result<()> {
        if self.config.relay.is_none_or(|enabled| !enabled) {
            //TODO add error types
            return Err(
                "Relay is disabled. Restart the waku node with Relay enabled to use this function."
                    .to_string(),
            );
        }

        relay::waku_relay_subscribe(&self.ctx, pubsub_topic.into()).await
    }

    /// Unsubscribe to stop receiving messages matching a pubsub topic.
    pub async fn relay_unsubscribe(&self, pubsub_topic: impl Into<PubsubTopic>) -> Result<()> {
        if self.config.relay.is_none_or(|enabled| !enabled) {
            //TODO add error types
            return Err(
                "Relay is disabled. Restart the waku node with Relay enabled to use this function."
                    .to_string(),
            );
        }

        relay::waku_relay_unsubscribe(&self.ctx, pubsub_topic.into()).await
    }

    pub async fn filter_subscribe(
        &self,
        pubsub_topic: PubsubTopic,
        content_topics: Vec<WakuContentTopic>,
    ) -> Result<()> {
        filter::waku_filter_subscribe(&self.ctx, pubsub_topic, content_topics).await
    }

    pub async fn filter_unsubscribe(
        &self,
        pubsub_topic: PubsubTopic,
        content_topics: Vec<WakuContentTopic>,
    ) -> Result<()> {
        filter::waku_filter_unsubscribe(&self.ctx, pubsub_topic, content_topics).await
    }

    pub async fn filter_unsubscribe_all(&self) -> Result<()> {
        filter::waku_filter_unsubscribe_all(&self.ctx).await
    }

    pub async fn lightpush_publish_message(
        &self,
        message: &WakuMessage,
        pubsub_topic: PubsubTopic,
    ) -> Result<MessageHash> {
        lightpush::waku_lightpush_publish_message(&self.ctx, message, pubsub_topic).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn store_query(
        &self,
        pubsub_topic: Option<PubsubTopic>,
        content_topics: Vec<WakuContentTopic>,
        peer_addr: &str,
        include_data: bool, // is true, resp contains payload, etc. Only msg_hashes otherwise
        time_start: Option<u64>, // unix time nanoseconds
        time_end: Option<u64>, // unix time nanoseconds
        timeout_millis: Option<Duration>,
    ) -> Result<Vec<StoreWakuMessageResponse>> {
        let mut cursor: Option<MessageHash> = None;

        let mut messages: Vec<StoreWakuMessageResponse> = Vec::new();

        loop {
            let query = StoreQueryRequest::new()
                .with_pubsub_topic(pubsub_topic.clone())
                .with_content_topics(content_topics.clone())
                .with_include_data(include_data)
                .with_time_start(time_start)
                .with_time_end(time_end)
                .with_pagination_cursor(cursor)
                .with_pagination_forward(true);

            let response =
                store::waku_store_query(&self.ctx, query, peer_addr, timeout_millis).await?;

            messages.extend(response.messages);

            if response.pagination_cursor.is_none() {
                break;
            }
            cursor = response.pagination_cursor;
        }

        messages.reverse();

        Ok(messages)
    }
}
