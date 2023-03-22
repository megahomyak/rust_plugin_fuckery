struct Plugin;

impl app::Plugin for Plugin {
    fn print(&self, message: &str) {
        println!("{}", message);
    }
}

pluginator::plugin_implementation!(app::Plugin, Plugin);
