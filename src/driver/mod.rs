use std::path::PathBuf;

#[path = "../utils/mod.rs"]
mod utils;

#[path = "../analysis/mod.rs"]
mod analysis;
// Represents a specific directory / package
#[derive(Debug, PartialEq, PartialOrd)]
pub struct AuditDir {
    dir: PathBuf,
    dir_files: Vec<PathBuf>,
    //ast: syn::File,
}
impl AuditDir {
    pub fn new(dir: PathBuf, dir_files: Vec<PathBuf>) -> AuditDir {
        AuditDir {
            dir,
            dir_files,
           // ast,
        }
    }
    pub fn cmp(&self, other: PathBuf) -> bool {
        self.dir == other
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

        for dir in &self.auditDirs {
            println!("{:#?}", dir);
        
        let merged_ast = utils::get_merged_ast(&dir.dir_files);

        // IF analysis flag is passed
        let mut analyzer = analysis::Analyzer::new();
        // analyzer.analyze(merged_ast);
        analyzer.get_call_graph(merged_ast)
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_driver() {
        let mut driver = Driver::new();
        driver.scope = PathBuf::from("/Users/cfkelly18/DEV/cosmwasm/cw-plus");
        driver.run();
    }
}
