pub fn main() {}

#[no_mangle]
pub extern fn add(a: i32, b: i32) -> i32 {
  return a + b
}