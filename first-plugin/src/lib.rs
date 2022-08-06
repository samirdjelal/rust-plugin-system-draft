#[no_mangle]
pub extern "Rust" fn handler() {
	println!("First plugin handler!");
}

#[no_mangle]
pub extern "Rust" fn name() -> String {
	format!("first_plugin")
}

#[no_mangle]
pub extern "Rust" fn description() -> String {
	format!("first plugin description")
}
