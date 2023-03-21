fn main() {
    let plugin = app::plugin::load("plugins/libplugin1.so").unwrap();
    plugin.print("hello");
}
