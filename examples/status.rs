extern crate pixy_access;

fn main() {
  let pixo = pixy_access::init_device();
  if pixo.is_ok() {
    println!("found a pixy");
  }
}