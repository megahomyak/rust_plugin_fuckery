#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    get_plugin: extern "C" fn() -> *mut dyn Plugin,
}

pub trait Plugin {
    fn handle_command(&self, command: &str);
}

fn main() {
    println!("Starting App");

    let plugin_api_wrapper: Container<PluginApi> =
        unsafe { Container::load("../plugin1/target/debug/libplugin1.so") }.unwrap();
    let plugin = unsafe { Box::from_raw(plugin_api_wrapper.get_plugin()) };

    loop {
        // Prompt
        println!("Enter command:");

        // Read input
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();

        // Trim newline
        command = command.trim().into();

        // Check command
        if command == "exit" {
            break;
        }
        plugin.handle_command(&command);
    }
}
