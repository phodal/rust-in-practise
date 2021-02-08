use my_interface::SayHelloService;
use libloading::Library;

#[cfg(target_os = "linux")]
fn load_library() -> Library {
    unsafe {
        return libloading::Library::new("target/debug/libmy_plugin.dylib")
            .expect("load library");
    }
}

#[cfg(target_os = "macos")]
fn load_library() -> Library {
    unsafe {
        return libloading::Library::new("target/debug/libmy_plugin.dylib")
            .expect("load library");
    }
}

#[cfg(target_os = "windows")]
fn load_library() -> Library {
    unsafe {
        return libloading::Library::new("target/debug/libmy_plugin.dll")
            .expect("load library");
    }
}

fn main() {
    let lib = load_library();

    let new_service: libloading::Symbol<extern "Rust" fn() -> Box<dyn SayHelloService>> = unsafe { lib.get(b"new_service") }
        .expect("load symbol");

    let service = new_service();
    service.say_hello();
    let service = new_service();
    service.say_hello();
}
