//! Waku [peer handling and connection](https://rfc.vac.dev/spec/36/#connecting-to-peers) methods

// std
use std::ffi::CString;
use std::time::Duration;
// crates
use multiaddr::Multiaddr;
// internal
use crate::general::libwaku_response::{handle_no_response, LibwakuResponse};
use crate::general::Result;
use crate::handle_ffi_call;
use crate::node::context::WakuNodeContext;

/// Dial peer using a multiaddress
/// If `timeout` as milliseconds doesn't fit into a `i32` it is clamped to [`i32::MAX`]
/// If the function execution takes longer than `timeout` value, the execution will be canceled and an error returned.
/// Use 0 for no timeout
/// As per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_connect_peerchar-address-int-timeoutms)
pub(crate) async fn waku_connect(
    ctx: &WakuNodeContext,
    address: &Multiaddr,
    timeout: Option<Duration>,
) -> Result<()> {
    let address =
        CString::new(address.to_string()).expect("CString should build properly from multiaddress");

    handle_ffi_call!(
        waku_sys::waku_connect,
        handle_no_response,
        ctx.get_ptr(),
        address.as_ptr(),
        timeout
            .map(|duration| duration.as_millis().try_into().unwrap_or(u32::MAX))
            .unwrap_or(0)
    )
}
