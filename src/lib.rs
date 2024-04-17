use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::Result;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaTransactionInfoVersions,
};
use solana_sdk::clock::Slot;
use solana_sdk::signature::Signature;
use std::fmt::Debug;

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
        if transaction.signature().to_string() == "4QdDG3fjk4vLLHEpxrFYUMux49Eg4vVaynaiKA9fJR64ZSoEcBA4xPpSYAfnSxoB1p2GQAruh8fPoXsUgX5YdZsj" {
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
        }
        Ok(())
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
    }
}

trait ReplicaTransactionInfoVersionsExt {
    fn signature(&self) -> &Signature;
}

impl ReplicaTransactionInfoVersionsExt for ReplicaTransactionInfoVersions<'_> {
    fn signature(&self) -> &Signature {
        match self {
            ReplicaTransactionInfoVersions::V0_0_1(replica_transaction_info) => {
                replica_transaction_info.signature
            }
            ReplicaTransactionInfoVersions::V0_0_2(replica_transaction_info_v2) => {
                replica_transaction_info_v2.signature
            }
        }
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
