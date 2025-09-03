//! Node lifcycle [mangement](https://rfc.vac.dev/spec/36/#node-management) related methods

// std
use std::ffi::CString;
// crates
use libc::c_void;
use multiaddr::Multiaddr;
use std::sync::Arc;
use tokio::sync::Notify;
// internal
use super::config::WakuNodeConfig;
use crate::general::libwaku_response::{handle_no_response, handle_response, LibwakuResponse};
use crate::general::Result;
use crate::handle_ffi_call;
use crate::macros::get_trampoline;
use crate::node::context::WakuNodeContext;

/// Instantiates a Waku node
/// as per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_newchar-jsonconfig)
pub(crate) async fn waku_new(config: &WakuNodeConfig) -> Result<WakuNodeContext> {
    let config = CString::new(
        serde_json::to_string(config)
            .expect("Serialization from properly built NodeConfig should never fail"),
    )
    .expect("CString should build properly from the config");
    let config_ptr = config.as_ptr();

    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();
    let mut result = LibwakuResponse::default();
    let result_cb = |r: LibwakuResponse| {
        result = r;
        notify_clone.notify_one(); // Notify that the value has been updated
    };
    let mut closure = result_cb;
    let obj_ptr = unsafe {
        let cb = get_trampoline(&closure);
        waku_sys::waku_new(config_ptr, cb, &mut closure as *mut _ as *mut c_void)
    };

    notify.notified().await; // Wait until a result is received

    match result {
        LibwakuResponse::MissingCallback => panic!("callback is required"),
        LibwakuResponse::Failure(v) => Err(v),
        _ => Ok(WakuNodeContext::new(obj_ptr)),
    }
}

pub(crate) async fn waku_destroy(ctx: &WakuNodeContext) -> Result<()> {
    handle_ffi_call!(waku_sys::waku_destroy, handle_no_response, ctx.get_ptr())
}

/// Start a Waku node mounting all the protocols that were enabled during the Waku node instantiation.
/// as per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_start)
pub(crate) async fn waku_start(ctx: &WakuNodeContext) -> Result<()> {
    handle_ffi_call!(waku_sys::waku_start, handle_no_response, ctx.get_ptr())
}

/// Stops a Waku node
/// as per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_stop)
pub(crate) async fn waku_stop(ctx: &WakuNodeContext) -> Result<()> {
    handle_ffi_call!(waku_sys::waku_stop, handle_no_response, ctx.get_ptr())
}

/// nwaku version
pub(crate) async fn waku_version(ctx: &WakuNodeContext) -> Result<String> {
    handle_ffi_call!(waku_sys::waku_version, handle_response, ctx.get_ptr())
}

/// Get the multiaddresses the Waku node is listening to
/// as per [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_listen_addresses)
pub(crate) async fn waku_listen_addresses(ctx: &WakuNodeContext) -> Result<Vec<Multiaddr>> {
    handle_ffi_call!(
        waku_sys::waku_listen_addresses,
        handle_response,
        ctx.get_ptr()
    )
}

#[cfg(test)]
mod test {
    use crate::WakuNodeHandle;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn waku_flow() {
        let node = WakuNodeHandle::new(None).await.unwrap();
        let node = node.start().await.unwrap();

        // test addresses
        let addresses = node.listen_addresses().await.unwrap();
        dbg!(&addresses);
        assert!(!addresses.is_empty());

        let node = node.stop().await.unwrap();
        node.waku_destroy().await.unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn nwaku_version() {
        let node = WakuNodeHandle::new(None).await.unwrap();

        let version = node.version().await.expect("should return the version");

        print!("Current version: {}", version);

        assert!(!version.is_empty());
        node.waku_destroy().await.unwrap();
    }
}
