fn main() {
    if cfg!(target_os = "macos") && cfg!(feature = "use_sdl2_mac_framework") {
	println!("cargo:rustc-link-search=framework=/Library/Frameworks");
    }
}
