#[macro_use]
extern crate janus_plugin;

use jansson_sys::json_t;
use janus_plugin::{
    JanssonDecodingFlags, JanssonValue, LibraryMetadata, Plugin, PluginCallbacks, PluginDataPacket,
    PluginResult, PluginRtcpPacket, PluginRtpPacket, PluginSession, RawJanssonValue,
    RawPluginResult,
};
use std::os::raw::{c_char, c_int};

macro_rules! c_str {
    ($lit:expr) => {
        unsafe { std::ffi::CStr::from_ptr(concat!($lit, "\0").as_ptr() as *const c_char) }
    };
}

extern "C" fn init(_callbacks: *mut PluginCallbacks, _config_path: *const c_char) -> c_int {
    janus_info!("Plugin loaded!");
    0
}

extern "C" fn destroy() {
    janus_info!("Plugin destroyed!");
}

extern "C" fn create_session(_handle: *mut PluginSession, _error: *mut c_int) {
    janus_info!("Create session!");
}
extern "C" fn handle_message(
    _handle: *mut PluginSession,
    _transaction: *mut c_char,
    _message: *mut json_t,
    _jsep: *mut json_t,
) -> *mut RawPluginResult {
    //    -> *mut janus_plugin_result,
    janus_info!("Handle message!");
    let result = PluginResult::ok_wait(Some(c_str!("Rust string")));
    result.into_raw()
}
extern "C" fn handle_admin_message(_message: *mut json_t) -> *mut RawJanssonValue {
    // *mut json_t,
    janus_info!("Handle admin message!");
    JanssonValue::from_str("[]", JanssonDecodingFlags::empty())
        .unwrap()
        .into_raw()
}
extern "C" fn setup_media(_handle: *mut PluginSession) {
    janus_info!("Setup media!");
}
extern "C" fn incoming_rtp(_handle: *mut PluginSession, _packet: *mut PluginRtpPacket) {
    janus_info!("Incoming rtp!");
}
extern "C" fn incoming_rtcp(_handle: *mut PluginSession, _packet: *mut PluginRtcpPacket) {
    janus_info!("Incoming rtcp!");
}
extern "C" fn incoming_data(_handle: *mut PluginSession, _packet: *mut PluginDataPacket) {
    janus_info!("Incoming data!");
}
extern "C" fn data_ready(_handle: *mut PluginSession) {
    janus_info!("Data ready!");
}
extern "C" fn slow_link(_handle: *mut PluginSession, _uplink: c_int, _video: c_int) {
    janus_info!("Slow link!");
}
extern "C" fn hangup_media(_handle: *mut PluginSession) {
    janus_info!("Hangup Media!");
}
extern "C" fn destroy_session(_handle: *mut PluginSession, _error: *mut c_int) {
    janus_info!("Destroy session!");
}
extern "C" fn query_session(_handle: *mut PluginSession) -> *mut RawJanssonValue {
    //  *mut json_t,
    janus_info!("Query session!");
    JanssonValue::from_str("[]", JanssonDecodingFlags::empty())
        .unwrap()
        .into_raw()
}

// ...other handlers omitted: see
// https://janus.conf.meetecho.com/docs/plugin_8h.html#details
const PLUGIN: Plugin = build_plugin!(
    LibraryMetadata {
        // The Janus plugin API version. The version compiled into the plugin
        // must be identical to the version in the Janus which loads the plugin.
        api_version: 15,
        // Incrementing plugin version number for your own use.
        version: 1,
        // Human-readable metadata which Janus can query.
        name: c_str!("My plugin name"),
        package: c_str!("My plugin package name"),
        version_str: c_str!(env!("CARGO_PKG_VERSION")),
        description: c_str!(env!("CARGO_PKG_DESCRIPTION")),
        author: c_str!(env!("CARGO_PKG_AUTHORS")),
    },
    init,
    destroy,
    create_session,
    handle_message,
    handle_admin_message,
    setup_media,
    incoming_rtp,
    incoming_rtcp,
    incoming_data,
    data_ready,
    slow_link,
    hangup_media,
    destroy_session,
    query_session
);

export_plugin!(&PLUGIN);

