use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use syn::{parse_file, Item, ItemFn};
use walkdir::WalkDir;
use crate::driver::AuditDir;

pub fn is_package(path: &Path) -> bool {
    path.join("Cargo.toml").exists()
}

// Checks if the directory is in the ignored list
// TODO: add a list of ignored paths
pub fn check_dir(dir: &Path) -> bool {
    dir.to_str().unwrap().contains("/target/")
}

pub fn walk(p: &PathBuf) -> Vec<PathBuf> {
    let mut dir_files: Vec<PathBuf> = Vec::new(); 
    for entry in WalkDir::new(p) {
        let entry = entry.unwrap(); 
        if entry.path().extension().is_some() && entry.path().extension().unwrap() == "rs" && !check_dir(entry.path()) {
            dir_files.push(entry.path().to_path_buf())
        }
    }
    dir_files
    
}

/// Takes a path to a directory and returns a vector of all the .rs files in that directory and
/// all subdirectories of that directory.
pub fn walk_dir(p: &PathBuf) -> Vec<AuditDir> {
    let mut scope_files: Vec<PathBuf> = vec![];
    let mut audit_dirs: Vec<AuditDir> = Vec::new(); 
    
    for entry in WalkDir::new(p) {
        let entry = entry.unwrap();
        if entry.path().is_dir() && !check_dir(entry.path()) && is_package(entry.path()) && !audit_dirs.iter().any(|p| p.cmp(entry.path().to_path_buf())){

            let new_dir = AuditDir::new(entry.path().to_path_buf(), walk(&entry.path().to_path_buf()));
            audit_dirs.push(new_dir);
        } 
    }
    println!("{:#?}", audit_dirs);
    audit_dirs
}

pub fn get_merged_ast(files: &Vec<PathBuf>) -> syn::File {
    let mut ast_vec = Vec::new();

    for file in files {
        let file = read_to_string(&file).unwrap();
        let ast = parse_file(&file).unwrap();

        // filtered_ast.visit_file_mut(&mut ast);

        ast_vec.extend(ast.items);
    }

    let merged: syn::File = syn::File {
        attrs: vec![],
        shebang: None,
        items: ast_vec,
    };
    merged
}

mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_process_files() {
        let path = PathBuf::from("/Users/cfkelly18/DEV/cosmwasm/cw-plus");
        let rs_files = walk_dir(&path);
        assert_ne!(rs_files.len(), 0); //TODO: need a better check
    }

    // #[test]
    // fn test_get_merged_ast() {
    //     let path = PathBuf::from("/Users/cfkelly18/DEV/cosmwasm/cw-plus");
    //     let rs_files = walk_dir(&path);
    //     let merged_ast = get_merged_ast(&rs_files);
    //     //random check to see if it's working
    //     for item in merged_ast.items {
    //         match item {
    //             Item::Fn(item_fn) => {
    //                 println!("Found function: {}", item_fn.sig.ident);
    //             }
    //             _ => {}
    //         }
    //     } //TODO: need a better check
    // }
}
