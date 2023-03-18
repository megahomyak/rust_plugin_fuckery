extern crate app;

struct Plugin1;

impl app::Plugin for Plugin1 {
    fn handle_command(&self, command: &str) {
        if command.starts_with("say ") {
            println!("{}", command.chars().skip(4).as_str());
        }
    }
}

#[no_mangle]
pub fn get_plugin() -> *mut dyn app::Plugin {
    Box::into_raw(Box::new(Plugin1))
}
