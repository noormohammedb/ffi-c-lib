#[link(name = "mymath")]
extern "C" {
	fn my_add(a: i32, b: i32) -> i32;
	fn my_sub(a: i32, b: i32) -> i32;
}

fn main() {
	println!("Hello, world!");
	unsafe {
		println!("my_add(1, 2) = {}", my_add(1, 2));
		println!("my_sub(1, 2) = {}", my_sub(1, 2));
	}
}
