mod analysis;
mod driver;
mod utils;

use std::env;
use std::path::PathBuf;


fn main() {

  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Error: no scope argument provided");
    return;
  }

  let scope = &args[1];

  if !PathBuf::from(scope).is_dir() {
    println!("Error: {} is not a valid directory", scope);
    return;
  }

  println!("Analyzing {}", scope);
  
  let mut driver = driver::Driver::new();
  driver.set_scope(PathBuf::from(scope));
  driver.run();

}
// cargo run -- /Users/cfkelly18/DEV/cosmwasm/cw-plus/

