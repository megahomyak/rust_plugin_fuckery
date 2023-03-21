use std::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

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

pub trait Plugin: Sync + Send {
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

impl Deref for LoadedPlugin {
    type Target = dyn Plugin;

    fn deref(&self) -> &Self::Target {
        self.plugin.as_ref()
    }
}

impl DerefMut for LoadedPlugin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.plugin.as_mut()
    }
}

pub mod plugin {
    use std::{mem::ManuallyDrop, path::Path};

    use super::{LoadedPlugin, Plugin};

    use libloading::{Library, Symbol};

    #[derive(Debug)]
    pub enum LoadingError {
        OpeningError(libloading::Error),
        InterfaceGettingError(libloading::Error),
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<LoadedPlugin, LoadingError> {
        let library =
            unsafe { Library::new(path.as_ref()) }.map_err(|e| LoadingError::OpeningError(e))?;
        let get_interface: Symbol<fn() -> *mut dyn Plugin> =
            unsafe { library.get(b"get_interface") }
                .map_err(|e| LoadingError::InterfaceGettingError(e))?;
        let plugin = unsafe { Box::from_raw(get_interface()) };
        Ok(LoadedPlugin {
            plugin: ManuallyDrop::new(plugin),
            library: ManuallyDrop::new(library),
        })
    }
}
