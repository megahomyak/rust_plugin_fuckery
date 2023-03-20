struct Plugin;

impl app::Plugin for Plugin {
    fn print(&self, message: &str) {
        println!("{}", message);
    }
}

app::export_plugin!(Plugin);
