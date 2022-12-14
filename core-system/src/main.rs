fn main() {
	println!("Hello, world!");
	
	#[cfg(target_os = "linux")]
		let plugins = vec!["./libfirst_plugin.so", "./libsecond_plugin.so"];
	
	#[cfg(target_os = "windows")]
		let plugins = vec!["./first_plugin.dll", "./second_plugin.dll"];
	
	for plugin in plugins {
		if let Err(e) = call_dynamic(plugin) {
			eprintln!("Error when running the plugin: {}", e);
			std::process::exit(1);
		}
	}
}

fn call_dynamic(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	unsafe {
		let path = std::path::Path::new(path);
		let library = libloading::Library::new(path.to_str().unwrap())?;
		let name: libloading::Symbol<unsafe extern fn() -> String> = library.get(b"name")?;
		let description: libloading::Symbol<unsafe extern fn() -> String> = library.get(b"description")?;
		let handler: libloading::Symbol<unsafe extern fn()> = library.get(b"handler")?;
		println!("name: {}", name());
		println!("description: {}", description());
		handler();
		println!("-------------------");
		Ok(())
	}
}