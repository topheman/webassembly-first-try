// Invoke JavaScript from WebAssembly

// Declare functions to be used from importObject
extern "C" {
    fn consoleLogString(message: i32);
    fn appendStringToDOM(message: i32);
}

#[no_mangle]
pub extern "C" fn run() {
    unsafe {
        consoleLogString(43);
        appendStringToDOM(42);
    }
}
