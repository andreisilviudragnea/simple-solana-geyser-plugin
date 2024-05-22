#![deny(warnings)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaBlockInfoVersions, ReplicaEntryInfoVersions,
    ReplicaTransactionInfoVersions, SlotStatus,
};
use solana_geyser_plugin_interface::geyser_plugin_interface::{ReplicaAccountInfoVersions, Result};
use solana_sdk::clock::Slot;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::SanitizedTransaction;
use std::fmt::Debug;

#[derive(Debug)]
struct GeyserPluginImpl;

impl GeyserPlugin for GeyserPluginImpl {
    fn name(&self) -> &'static str {
        "GeyserPluginImpl"
    }

    fn on_load(&mut self, config_file: &str) -> Result<()> {
        solana_logger::setup_with_default("info");
        info!("on_load(config_file={:?})", config_file);

        Ok(())
    }

    fn on_unload(&mut self) {
        info!("on_unload()");
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: Slot,
        is_startup: bool,
    ) -> Result<()> {
        let replica_account_info_v3 = match account {
            ReplicaAccountInfoVersions::V0_0_1(_) | ReplicaAccountInfoVersions::V0_0_2(_) => {
                unreachable!()
            }
            ReplicaAccountInfoVersions::V0_0_3(replica_account_info_v3) => replica_account_info_v3,
        };
        info!(
            "update_account(slot={slot}, pubkey={}, owner={}, executable={}, write_version={}, tx_sig={:?}, is_startup={is_startup})",
            Pubkey::try_from(replica_account_info_v3.pubkey).unwrap(),
            Pubkey::try_from(replica_account_info_v3.owner).unwrap(),
            replica_account_info_v3.executable,
            replica_account_info_v3.write_version,
            replica_account_info_v3.txn.map(SanitizedTransaction::signature),
        );
        Ok(())
    }

    fn notify_end_of_startup(&self) -> Result<()> {
        info!("notify_end_of_startup()");
        Ok(())
    }

    fn update_slot_status(
        &self,
        slot: Slot,
        parent: Option<u64>,
        status: SlotStatus,
    ) -> Result<()> {
        info!("update_slot_status(slot={slot}, parent={parent:?}, status={status:?})");
        Ok(())
    }

    fn notify_transaction(
        &self,
        transaction: ReplicaTransactionInfoVersions,
        slot: Slot,
    ) -> Result<()> {
        let replica_transaction_info_v2 = match transaction {
            ReplicaTransactionInfoVersions::V0_0_1(_) => unreachable!(),
            ReplicaTransactionInfoVersions::V0_0_2(replica_transaction_info_v2) => {
                replica_transaction_info_v2
            }
        };
        info!(
            "notify_transaction(slot={slot}, sig={})",
            replica_transaction_info_v2.signature
        );
        Ok(())
    }

    fn notify_entry(&self, entry: ReplicaEntryInfoVersions) -> Result<()> {
        let ReplicaEntryInfoVersions::V0_0_1(replica_entry_info) = entry;
        info!(
            "notify_entry(slot={}, index={}, num_hashes={:?}, hash={}, executed_tx_count={})",
            replica_entry_info.slot,
            replica_entry_info.index,
            replica_entry_info.num_hashes,
            Hash::new(replica_entry_info.hash),
            replica_entry_info.executed_transaction_count
        );
        Ok(())
    }

    fn notify_block_metadata(&self, blockinfo: ReplicaBlockInfoVersions) -> Result<()> {
        info!(
            "notify_block_metadata(blockinfo={:?})",
            match blockinfo {
                ReplicaBlockInfoVersions::V0_0_1(_) | ReplicaBlockInfoVersions::V0_0_2(_) =>
                    unreachable!(),
                ReplicaBlockInfoVersions::V0_0_3(replica_block_info_v3) => replica_block_info_v3,
            }
        );
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
    }

    fn entry_notifications_enabled(&self) -> bool {
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
/// Return `GeyserPlugin`
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    Box::into_raw(Box::new(GeyserPluginImpl))
}
