use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug};
use std::path::PathBuf;

use std::fs;

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
impl fmt::Display for DirType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DirType::Package => write!(f, "Package"),
            DirType::Contract => write!(f, "Contract"),
            DirType::Other => write!(f, "Other"),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct FileSummary {
    path: PathBuf,
    name: String,
    loc: u32,
    audit_lines: u32,
}
// Represents a specific file object, initialized with empty lines
impl FileSummary {
    pub fn new(path: PathBuf) -> FileSummary {
        FileSummary {
            path: path.clone(),
            name: path.file_name().unwrap().to_string_lossy().into_owned(),
            loc: 0,
            audit_lines: 0,
        }
    }
}
impl fmt::Display for FileSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\t[{}]\t  | LOC: ({})  | SLOC: ({})",
            self.name, self.loc, self.audit_lines
        )
    }
}
// Represents a specific directory / package
#[derive(Debug, PartialEq, Clone)]
pub struct AuditDir {
    dir: PathBuf,
    dir_files: Vec<FileSummary>,
    entrypoints: HashSet<String>,
    dir_type: DirType,
    dir_loc: u32,
    dir_audit_lines: u32,
}
impl AuditDir {
    pub fn new(dir: PathBuf, dir_files: Vec<FileSummary>, dir_type: DirType) -> AuditDir {
        AuditDir {
            dir,
            dir_files,
            entrypoints: HashSet::new(),
            dir_type,
            dir_audit_lines: 0,
            dir_loc: 0,
        }
    }
    pub fn cmp(&self, other: PathBuf) -> bool {
        self.dir == other
    }
    pub fn set_entrypoints(&mut self, entrypoints: HashSet<String>) {
        self.entrypoints = entrypoints;
    }
    pub fn fmt(&self) {
        let output_str: String;
        if !self.entrypoints.is_empty() {
            output_str = format!(
                "\n> {} [{}] | SLOC: [{}] | Entrypoints: [{}]",
                self.dir.to_string_lossy(),
                self.dir_type.clone(),
                self.dir_loc,
                self.entrypoints.len()
            );
        } else {
            output_str = format!(
                "\n> {} [{}] | SLOC: [{}]",
                self.dir.to_string_lossy(),
                self.dir_type.clone(),
                self.dir_loc
            );
        }

        println!("{}", output_str);
        for f in self.dir_files.clone() {
            println!("{}", f);
        }
    }
    fn get_paths(&self) -> Vec<PathBuf> {
        let paths = self
            .dir_files
            .iter()
            .map(|f| f.path.clone())
            .collect::<Vec<PathBuf>>();
        paths
    }
    fn set_file_lines(&mut self) {
        for f in &mut self.dir_files {
            let (loc, audit_lines) = utils::get_file_lines(f.path.clone());
            f.loc = loc;
            f.audit_lines = audit_lines;
            self.dir_loc += loc;
            self.dir_audit_lines += audit_lines;
        }
    }
}

pub struct Driver {
    scope: PathBuf,
    auditDirs: Vec<AuditDir>,
    cleanup: bool,
    analysis: bool,
    //args
}

impl Driver {
    pub fn new(cleanup:bool, analysis:bool) -> Driver {
        Driver {
            scope: PathBuf::new(),
            auditDirs: Vec::new(),
            cleanup: cleanup,
            analysis: analysis,
        }
    }

    pub fn run(&mut self) {
        self.auditDirs = utils::walk_dir(&self.scope);

        for dir in &mut self.auditDirs {
            let merged_ast = utils::get_merged_ast(&dir.get_paths());

            let mut analyzer = analysis::Analyzer::new();
            dir.set_entrypoints(analyzer.get_entrypoints(merged_ast.clone()));

            dir.set_file_lines(); // TODO - If in scoping mode

            // I removed analysis function for now.. just a scoper until I get time to work on it
            if self.analysis {
                analyzer.run_static_analysis(merged_ast)
                //analyzer.get_call_graph(merged_ast);
            }
            // 
        }
        self.summarize();

        // Only cleanup for remote repos
        if self.cleanup {
            self.cleanup();
        }
        
    }
    pub fn set_scope(&mut self, scope: PathBuf) {
        self.scope = scope;
    }
    // very very basic cleanup for now
    fn cleanup(&mut self) {
        fs::remove_dir_all(&self.scope).unwrap();
        println!("REMOVING: {:#?}", self.scope);
    }
    fn summarize(&self) {
        let mut total_lines = 0;

        for d in self.auditDirs.clone() {
            d.fmt();
            total_lines += d.dir_audit_lines;
        }
        let lines_per_hour: f32 = 200.0;
        let hours_per_aw: f32 = 20.0;

        let time_est: f32 = total_lines as f32 / lines_per_hour;
        let aw = time_est / hours_per_aw;
        print!(
            "\nTotal Lines: {}\nTime Est: {} Hours / AW Est {}\n ",
            total_lines, time_est, aw
        )
    }
}
// need to add actual unit tests later

mod test {
    use super::*;

    #[test]
    fn test_driver() {
        let mut driver = Driver::new(false, false);
        driver.scope = PathBuf::from("/Users/cfkelly18/DEV/cosmwasm/cw-plus/");
        driver.run();
    }
    #[test]
    fn test_remote_repo() {
        let url = String::from("https://github.com/CosmWasm/cw-plus.git");
        let mut driver = Driver::new(true, false);
        driver.set_scope(utils::process_remote_repo(&url));
        driver.run();

    }
        
}
