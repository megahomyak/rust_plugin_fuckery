use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    }, io::Write,
};

use serde::Deserialize;

#[derive(Deserialize)]
struct PluginInfo {
    path: PathBuf,
    id: u64,
}

type PluginId = u64;

fn load_plugins() -> HashMap<PluginId, pluginator::LoadedPlugin<dyn app::Plugin>> {
    let plugins_info: Vec<PluginInfo> = serde_json::from_str(
        &std::fs::read_to_string("plugins/plugins.json")
            .expect("plugins list file cannot be read!"),
    )
    .expect("plugins list file is ill-formed!");
    let mut plugins = HashMap::new();
    for plugin_info in plugins_info {
        let plugin = unsafe { app::load_plugin(Path::new("plugins/").join(&plugin_info.path)) }
            .unwrap_or_else(|_err| panic!("Plugin file {:?} cannot be read!", plugin_info.path));
        plugins.insert(plugin_info.id, plugin);
    }
    plugins
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_owned();
    input
}

fn main() {
    let do_reload = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGHUP, do_reload.clone())
        .expect("error setting signal hook");
    let mut plugins = load_plugins();
    loop {
        if do_reload.load(Ordering::Relaxed) {
            plugins = load_plugins();
        }
        println!(
            "Loaded plugin IDs: {}",
            plugins
                .keys()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        );
        let input = input("Enter the ID of a plugin to activate: ");
        let plugin_id = input.parse::<u64>().expect("input is not a number");
        let Some(plugin) = plugins.get(&plugin_id) else {
            println!("No plugin with ID {}!", plugin_id);
            continue;
        };
        plugin.print(&crate::input("Enter what to say: "));
    }
}
