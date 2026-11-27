use agave_geyser_plugin_interface::geyser_plugin_interface::{GeyserPlugin,Result as PluginResult, ReplicaAccountInfoVersions};

use solana_clock::{Slot, BankId};
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

    fn update_account_for_bank(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: Slot,
        bank_id: BankId,
    ) -> PluginResult<()>
    {
        // println!("This is the data inside the slot {} and bankId {}", slot, bank_id);
        match account {
            ReplicaAccountInfoVersions::V0_0_1(val) => {
                // val
                // println!("ACCOUNT version 1 : lmaport {} , pubkey {:?} , write_version {}", val.lamports, bs58::encode(val.pubkey).into_string(), val.write_version);
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
                // println!("ACCOUNT version 2 : lmaport {} , pubkey {:?} , write_version {}", val.lamports, bs58::encode(val.pubkey).into_string(), val.write_version);
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
                // println!("ACCOUNT version 3 : lmaport {} , pubkey {:?} , write_version {}", val.lamports, bs58::encode(val.pubkey).into_string(), val.write_version);
                println!("ACCOUNT VERSION 3");
                if val.txn.is_some() {
                    println!("This is the TXN INFO {:?}" , val.txn);
                }
                // let txn_info = if let Some(txn)  = &val.txn {
                //     // println!(val1.)
                //     format!("This is the TXN INFO{:?}", txn);
                // } else {
                //     "None".to_string();
                // };
                // println!("This is the txn info {:?}", txn_info);
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
}

#[unsafe(no_mangle)]
pub unsafe fn _create_plugin() -> *mut dyn GeyserPlugin  {
    let plugin  = Box::new(GeyserStreamPlugin);

    let raw_plugin = Box::into_raw(plugin);
    raw_plugin
}
