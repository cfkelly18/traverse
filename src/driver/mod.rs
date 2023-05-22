use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[path = "../utils/mod.rs"]
mod utils;

#[path = "../analysis/mod.rs"]
mod analysis;
#[derive(Debug, PartialEq, Clone)]
pub enum DirType {
    Package,
    Contract,
    Other,
}
// Represents a specific directory / package
#[derive(Debug, PartialEq, Clone)]
pub struct AuditDir {
    dir: PathBuf,
    dir_files: Vec<PathBuf>,
    entrypoints: HashSet<String>,
    dir_type: DirType, // ast: syn::File,
}
impl AuditDir {
    pub fn new(dir: PathBuf, dir_files: Vec<PathBuf>, dir_type: DirType) -> AuditDir {
        AuditDir {
            dir,
            dir_files,
            entrypoints: HashSet::new(),
            dir_type, // ast: syn::File::default(),
        }
    }
    pub fn cmp(&self, other: PathBuf) -> bool {
        self.dir == other
    }
    pub fn set_entrypoints(&mut self, entrypoints: HashSet<String>) {
        self.entrypoints = entrypoints;
    }
    pub fn summarize(&self) {
        println!("Directory: {:#?}", self.dir);
        println!("Entrypoints: {:#?}", self.entrypoints);
    }
}

struct Driver {
    scope: PathBuf,
    auditDirs: Vec<AuditDir>,
    //args
}

impl Driver {
    fn new() -> Driver {
        Driver {
            scope: PathBuf::new(),
            auditDirs: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.auditDirs = utils::walk_dir(&self.scope);

        for mut dir in self.auditDirs.clone() {
            println!("{:#?}", dir);

            let merged_ast = utils::get_merged_ast(&dir.dir_files);

            // IF analysis flag is passed
            let mut analyzer = analysis::Analyzer::new();
            dir.set_entrypoints(analyzer.get_entrypoints(merged_ast.clone()));

            // analyzer.analyze(merged_ast);
            //analyzer.get_call_graph(merged_ast)
            dir.summarize();
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_driver() {
        let mut driver = Driver::new();
        driver.scope = PathBuf::from("/Users/cfkelly18/DEV/cosmwasm/cw-plus/");
        driver.run();
    }
}
