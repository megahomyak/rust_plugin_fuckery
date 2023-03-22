pub trait Plugin: Sync + Send {
    fn print(&self, message: &str);
}

pluginator::plugin_trait!(Plugin);
