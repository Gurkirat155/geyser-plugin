use agave_geyser_plugin_interface::geyser_plugin_interface::{GeyserPlugin,Result as PluginResult, ReplicaAccountInfoVersions, ReplicaTransactionInfoVersions};

use solana_transaction_status::{ TransactionStatusMeta};
use solana_transaction::{sanitized::SanitizedTransaction as SanitizedTransaction_Transaction, versioned::VersionedTransaction};
use {solana_clock::{Slot, BankId}, solana_signature::Signature};
#[derive(Debug)]
pub struct GeyserStreamPlugin;

impl GeyserPlugin for GeyserStreamPlugin {
    
    fn name(&self) -> &'static str {
        // println!("geyser-stream-plugin");
        "geyser-stream-plugin"
    }

    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> PluginResult<()>{
        println!("On load function called by the agave validator and this is the config file {} and is_reload {}", _config_file, _is_reload );
        Ok(())
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
    }

    fn update_account_for_bank(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: Slot,
        bank_id: BankId,
    ) -> PluginResult<()>
    {
        // println!("This is the data inside the slot {} and bankId {}", slot, bank_id);
        println!("UPDATE ACCOUNT FOR BANK");
        match account {
            ReplicaAccountInfoVersions::V0_0_1(val) => {
                // val
                // println!("ACCOUNT version 1 : lmaport {} , pubkey {:#?} , write_version {}", val.lamports, bs58::encode(val.pubkey).into_string(), val.write_version);
                println!(
                    "ACCOUNT VERSION 1 
                    slot: {}, bank_id: {}, pubkey: {}, lamports: {}, write_version: {}, data_len: {}",
                    slot,
                    bank_id,
                    bs58::encode(val.pubkey).into_string(),
                    val.lamports,
                    val.write_version,
                    val.data.len()
                );
            },
            ReplicaAccountInfoVersions::V0_0_2(val) => {
                // println!("ACCOUNT version 2 : lmaport {} , pubkey {:#?} , write_version {}", val.lamports, bs58::encode(val.pubkey).into_string(), val.write_version);
                println!(
                    "ACCOUNT VERSION 2 
                    slot: {}, bank_id: {}, pubkey: {}, lamports: {}, write_version: {}, data_len: {}",
                    slot,
                    bank_id,
                    bs58::encode(val.pubkey).into_string(),
                    val.lamports,
                    val.write_version,
                    val.data.len()
                );
            },
            ReplicaAccountInfoVersions::V0_0_3(val) => {
                // println!("ACCOUNT version 3 : lmaport {} , pubkey {:#?} , write_version {}", val.lamports, bs58::encode(val.pubkey).into_string(), val.write_version);
                println!("ACCOUNT VERSION 3");
                if val.txn.is_some() {
                    println!("This is the TXN INFO {:#?}" , val.txn);
                }
                // let txn_info = if let Some(txn)  = &val.txn {
                //     // println!(val1.)
                //     format!("This is the TXN INFO{:#?}", txn);
                // } else {
                //     "None".to_string();
                // };
                // println!("This is the txn info {:#?}", txn_info);
                println!(
                    "
                    slot: {}, bank_id: {}, pubkey: {}, lamports: {}, write_version: {}, data_len: {}", 
                    slot,
                    bank_id,
                    bs58::encode(val.pubkey).into_string(),
                    val.lamports,
                    val.write_version,
                    val.data.len(),
                    // val.txn.is_some(),
                    // val.txn,
                    // txn_info,

                );
            }
        }
        Ok(())
    }

    fn notify_transaction_for_bank(
        &self,
        transaction: ReplicaTransactionInfoVersions,
        slot: Slot,
        bank_id: BankId,
    ) -> PluginResult<()>
    {
        // We'll use if let and understand how it works 
        println!("NOTIFY TXN FOR BANK");
        let log_details = | version: &str, signature: &Signature , is_vote: bool, transaction:    Option<&SanitizedTransaction_Transaction>, transaction_v3 : Option<&VersionedTransaction> ,transaction_status_meta: &TransactionStatusMeta  , index: Option<usize>| {
            println!("TRANSACTION VERSION {version}, SIGNATURE : {signature}, IS VOTE : {is_vote}, TRANSACTION : {transaction:#?}, TRANSACTION_V3 : {transaction_v3:#?}, TRANSACTION STATUS META: {transaction_status_meta:#?}, INDEX : {index:?}");
        };

        match transaction {
            ReplicaTransactionInfoVersions::V0_0_1(val) => {
                log_details( "1", val.signature, val.is_vote, Some(val.transaction), None, val.transaction_status_meta,  None);
                // if let Some(val.)
            },
            ReplicaTransactionInfoVersions::V0_0_2(val) => { 
                log_details("2", val.signature, val.is_vote, Some(val.transaction), None, val.transaction_status_meta, Some(val.index));
            },
            ReplicaTransactionInfoVersions::V0_0_3(val) => { 
                log_details("3", val.signature, val.is_vote, None, Some(val.transaction), val.transaction_status_meta, Some(val.index));
            },

        }
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub unsafe fn _create_plugin() -> *mut dyn GeyserPlugin  {
    let plugin  = Box::new(GeyserStreamPlugin);

    let raw_plugin = Box::into_raw(plugin);
    raw_plugin
}
