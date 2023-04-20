use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use syn::{parse_file, Item, ItemFn};
use walkdir::WalkDir;

// Checks if the directory is in the ignored list
// TODO: add a list of ignored paths
pub fn check_dir(dir: &Path) -> bool {
    dir.to_str().unwrap().contains("/target/")
}

/// Takes a path to a directory and returns a vector of all the .rs files in that directory and
/// all subdirectories of that directory.
pub fn walk_dir(p: &PathBuf) -> Vec<PathBuf> {
    let mut scope_files: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(p) {
        let entry = entry.unwrap();

        if entry.path().extension().is_some()
            && entry.path().extension().unwrap() == "rs"
            && !check_dir(entry.path())
        {
            // println!("{:#?}", entry.path());
            scope_files.push(entry.path().to_path_buf())
        };
    }

    scope_files.sort();
    scope_files
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

    #[test]
    fn test_get_merged_ast() {
        let path = PathBuf::from("/Users/cfkelly18/DEV/cosmwasm/cw-plus");
        let rs_files = walk_dir(&path);
        let merged_ast = get_merged_ast(&rs_files);
        //random check to see if it's working
        for item in merged_ast.items {
            match item {
                Item::Fn(item_fn) => {
                    println!("Found function: {}", item_fn.sig.ident);
                }
                _ => {}
            }
        } //TODO: need a better check
    }
}
