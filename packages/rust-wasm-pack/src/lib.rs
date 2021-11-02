pub fn main() {}

#[no_mangle]
pub extern fn square (x: u32) -> u32 {
  x * x
}