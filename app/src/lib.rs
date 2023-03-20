use std::{any::Any, mem::ManuallyDrop};

use libloading::Library;

#[macro_export]
macro_rules! export_plugin {
    ($initializer:expr) => {
        #[no_mangle]
        pub extern "C" fn get_interface() -> *mut dyn $crate::Plugin {
            Box::into_raw(Box::new($initializer))
        }
    };
}

pub trait Plugin: Any + Sync + Send {
    fn print(&self, message: &str);
}

pub struct LoadedPlugin {
    library: ManuallyDrop<Library>,
    plugin: ManuallyDrop<Box<dyn Plugin>>,
}

impl Drop for LoadedPlugin {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.plugin);
            ManuallyDrop::drop(&mut self.library);
        }
    }
}

impl Plugin for LoadedPlugin {
    fn print(&self, message: &str) {
        self.plugin.print(message)
    }
}

pub mod plugin {
    use std::{mem::ManuallyDrop, path::Path};

    use super::{LoadedPlugin, Plugin};

    use libloading::{Library, Symbol};

    #[derive(Debug)]
    pub enum LoadingError {
        LibraryNotFound,
        InterfaceGetterNotFound,
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<LoadedPlugin, LoadingError> {
        let library =
            unsafe { Library::new(path.as_ref()) }.map_err(|_| LoadingError::LibraryNotFound)?;
        let get_interface: Symbol<fn() -> *mut dyn Plugin> =
            unsafe { library.get(b"get_interface") }
                .map_err(|_| LoadingError::InterfaceGetterNotFound)?;
        let plugin = unsafe { Box::from_raw(get_interface()) };
        Ok(LoadedPlugin {
            plugin: ManuallyDrop::new(plugin),
            library: ManuallyDrop::new(library),
        })
    }
}
