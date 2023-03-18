#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    run: extern "C" fn(),
}

fn main() {
    println!("Starting App");

    let plugin_api_wrapper: Container<PluginApi> =
        unsafe { Container::load("../plugin1/target/debug/libplugin1.so") }.unwrap();
    plugin_api_wrapper.run();
}
