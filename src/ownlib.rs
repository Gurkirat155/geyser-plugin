use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfoVersions, Result as PluginResult,
};


#[derive(Debug, Default)]
pub struct StreamPlugin;

impl GeyserPlugin for StreamPlugin {
    fn name(&self) -> &'static str {
        "geyser-stream-plugin"
    }

    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> PluginResult<()> {
        eprintln!("[geyser-stream-plugin] loaded");
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        if is_startup { 
            return Ok(()); // skip the snapshot flood for now (handled in Week 5)
        }
        // If the compiler complains here, your pinned version has fewer/more            
        // variants — adjust the arms to match.
        let pubkey = match account {
            ReplicaAccountInfoVersions::V0_0_3(a) => a.pubkey,
            ReplicaAccountInfoVersions::V0_0_2(a) => a.pubkey,
            ReplicaAccountInfoVersions::V0_0_1(a) => a.pubkey,
        };
        eprintln!(
            "[geyser-stream-plugin] account {} @ slot {}",
            bs58::encode(pubkey).into_string(),
            slot
        );
        Ok(())
    }
}

/// The validator loads the .dylib/.so and calls this to build the plugin.
// #[no_mangle]
// #[allow(improper_ctypes_definitions)]
// pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
//     let plugin: Box<dyn GeyserPlugin> = Box::new(StreamPlugin::default());
//     Box::into_raw(plugin)
// }
