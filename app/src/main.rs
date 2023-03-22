fn main() {
    let plugin = app::load_plugin("plugins/libplugin1.so").unwrap();
    plugin.print("hello");
}
