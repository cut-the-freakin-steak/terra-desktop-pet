fn main() {
	println!("cargo:rerun-if-changed=assets/jAkEtop.ico");

	if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
		let mut resource = winresource::WindowsResource::new();
		resource.set_icon("assets/jAkEtop.ico");
		resource
			.compile()
			.expect("failed to compile Windows resources");
	}
}
