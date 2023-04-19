mod analysis;
mod driver;
mod utils;

use std::env;
use std::path::PathBuf;

fn main() {
    // Get the command-line arguments passed to the program
    // let args: Vec<String> = env::args().collect();
    //TODO - get the path to the project
    let dir = PathBuf::from("/Users/cfkelly18/DEV/cosmwasm/cw-plus");

    let analyzer = analysis::Analyzer::new();
    // Call your analysis function from the analysis module and pass the arguments to it
    //analyzer.analyze(&args);
}
