#![allow(improper_ctypes_definitions)]

#[no_mangle]
pub extern "C" fn handler() {
	println!("Second plugin handler!");
}

#[no_mangle]
pub extern "C" fn name() -> String {
	format!("second_plugin")
}

#[no_mangle]
pub extern "C" fn description() -> String {
	format!("second plugin description")
}
