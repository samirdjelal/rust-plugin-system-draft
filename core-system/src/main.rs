fn main() {
	println!("Hello, world!");
	let plugins = vec!["first_plugin.dll", "second_plugin.dll"];
	for plugin in plugins {
		if let Err(e) = call_dynamic(plugin) {
			eprintln!("Error when running the plugin: {}", e);
			std::process::exit(1);
		}
	}
}

fn call_dynamic(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	unsafe {
		let library = libloading::Library::new(path)?;
		let func: libloading::Symbol<unsafe extern fn()> = library.get(b"handler")?;
		Ok(func())
	}
}