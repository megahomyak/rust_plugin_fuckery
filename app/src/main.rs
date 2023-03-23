fn main() {
    let plugin = unsafe { app::load_plugin("plugins/libplugin1.so") }.unwrap();
    plugin.print("hello");
}
