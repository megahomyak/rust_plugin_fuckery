#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    run: extern "C" fn(),
}

pub fn run() {
    println!("Starting App");

    let plugin_api_wrapper: Container<PluginApi> =
        unsafe { Container::load("../plugin1/target/debug/libplugin1.so") }.unwrap();
    plugin_api_wrapper.run();
}

pub fn test_app_func(message: &str) {
    println!("test_app_func(\"{}\")", message);
}
