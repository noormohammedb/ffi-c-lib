fn main() {
	println!("cargo:rustc-link-lib=dylib=mymath");
	println!("cargo:rustc-link-search=native=.");
}
