#![allow(improper_ctypes_definitions)]

#[no_mangle]
pub extern "C" fn handler() {
	println!("First plugin handler!");
}

#[no_mangle]
pub extern "C" fn name() -> String {
	format!("first_plugin")
}

#[no_mangle]
pub extern "C" fn description() -> String {
	format!("first plugin description")
}
