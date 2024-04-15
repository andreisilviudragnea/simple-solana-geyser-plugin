use solana_geyser_plugin_interface::geyser_plugin_interface::Result;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaTransactionInfoVersions,
};
use solana_sdk::clock::Slot;
use std::fmt::Debug;
use tracing::info;

#[derive(Debug)]
struct GeyserPluginImpl;

impl GeyserPlugin for GeyserPluginImpl {
    fn name(&self) -> &'static str {
        "GeyserPluginImpl"
    }

    fn on_load(&mut self, config_file: &str) -> Result<()> {
        solana_logger::setup_with_default("info");
        info!("on_load(config_file={:#?})", config_file);

        Ok(())
    }

    fn notify_transaction(
        &self,
        transaction: ReplicaTransactionInfoVersions,
        slot: Slot,
    ) -> Result<()> {
        info!(
            "notify_transaction(slot={slot}, transaction={})",
            match transaction {
                ReplicaTransactionInfoVersions::V0_0_1(replica_transaction_info) => {
                    format!("{:#?}", replica_transaction_info)
                }
                ReplicaTransactionInfoVersions::V0_0_2(replica_transaction_info_v2) => {
                    format!("{:#?}", replica_transaction_info_v2)
                }
            }
        );
        Ok(())
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
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
