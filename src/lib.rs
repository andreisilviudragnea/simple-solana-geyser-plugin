use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;
use std::fmt::Debug;

#[derive(Debug)]
struct GeyserPluginImpl;

impl GeyserPlugin for GeyserPluginImpl {
    fn name(&self) -> &'static str {
        "GeyserPluginImpl"
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// Return GeyserPlugin
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    Box::into_raw(Box::new(GeyserPluginImpl))
}
