use std::path::PathBuf;

#[path = "../utils/mod.rs"]
mod utils;

#[path = "../analysis/mod.rs"]
mod analysis;

struct Driver {
    scope: PathBuf,
    scope_files: Vec<PathBuf>,
    //args
}

impl Driver {
    fn new() -> Driver {
        Driver {
            scope: PathBuf::new(),
            scope_files: vec![],
        }
    }

    fn run(&mut self) {
        self.scope_files = utils::walk_dir(&self.scope);

        let merged_ast = utils::get_merged_ast(&self.scope_files);

        // IF analysis flag is passed
        let mut analyzer = analysis::Analyzer::new();
        // analyzer.analyze(merged_ast);
        analyzer.get_call_graph(merged_ast)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_driver() {
        let mut driver = Driver::new();
        driver.scope = PathBuf::from("/Users/cfkelly18/DEV/cosmwasm/cw-plus/contracts/cw4-stake");
        driver.run();
    }
}
