use libloading::{Library, Symbol};

type AddFunc = unsafe fn(usize, usize) -> usize;

fn main() {
    let plugin = unsafe { Library::new("plugins/plugin1.so") }.unwrap();
    let func: Symbol<AddFunc> = unsafe { plugin.get(b"add") }.unwrap();
    println!("{}", unsafe { func(1, 2) });
}
