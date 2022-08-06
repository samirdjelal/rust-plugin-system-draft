#[no_mangle]
pub extern "Rust" fn handler() {
	println!("Second plugin handler!");
}

#[no_mangle]
pub extern "Rust" fn name() -> String {
	format!("second_plugin")
}

#[no_mangle]
pub extern "Rust" fn description() -> String {
	format!("second plugin description")
}
