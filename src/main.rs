mod analysis;
mod driver;
mod utils;

use std::env;
use std::path::PathBuf;

fn main() {
    

    let mut driver = driver::Driver::new();
    driver.set_scope(PathBuf::from("/Users/cfkelly18/DEV/cosmwasm/cw-plus/"));
    driver.run();
}
