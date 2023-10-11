use crate::driver::{AuditDir, DirType, FileSummary};
use git2::Repository;
use gix::bstr::BStr;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use syn::{parse_file, Item, ItemFn};
use walkdir::WalkDir;

pub fn is_package(path: &Path) -> bool {
    path.join("Cargo.toml").exists()
}

// Checks if the directory is in the ignored list
// TODO: add a list of ignored paths
pub fn check_dir(dir: &Path) -> bool {
    dir.to_str().unwrap().contains("/target/") || dir.to_str().unwrap().contains("test")
}

pub fn walk(p: &PathBuf) -> Vec<FileSummary> {
    let mut dir_files: Vec<FileSummary> = Vec::new();
    for entry in WalkDir::new(p) {
        let entry = entry.unwrap();
        if entry.path().extension().is_some()
            && entry.path().extension().unwrap() == "rs"
            && !check_dir(entry.path())
        {
            dir_files.push(FileSummary::new(entry.path().to_path_buf()))
        }
    }

    dir_files
}

pub fn get_dir_type(s: String) -> DirType {
    let dir_type: DirType;
    if s.contains("/contracts/") {
        dir_type = DirType::Contract
    } else if s.contains("/packages/") {
        dir_type = DirType::Package
    } else {
        dir_type = DirType::Other
    }
    dir_type
}
pub fn is_top_level_dir(dir_type: DirType) -> bool {
    match dir_type {
        DirType::Package => false,
        DirType::Contract => false,
        DirType::Other => true, // TODO: update
    }
}

/// Takes a path to a directory and returns a vector of all the .rs files in that directory and
/// all subdirectories of that directory.
pub fn walk_dir(p: &PathBuf) -> Vec<AuditDir> {
    let mut audit_dirs: Vec<AuditDir> = Vec::new();

    for entry in WalkDir::new(p) {
        let entry = entry.unwrap();
        if entry.path().is_dir()
            && !check_dir(entry.path())
            && is_package(entry.path())
            && !audit_dirs.iter().any(|p| p.cmp(entry.path().to_path_buf()))
        {
            let dir_type: DirType = get_dir_type(entry.path().to_str().unwrap().to_string());
            let is_tld = is_top_level_dir(dir_type.clone());
            if !is_tld {
                let new_dir = AuditDir::new(
                    entry.path().to_path_buf(),
                    walk(&entry.path().to_path_buf()),
                    dir_type,
                );
                audit_dirs.push(new_dir);
            }
        }
    }
    // println!("{:#?}", audit_dirs);
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

pub fn get_file_lines(f: PathBuf) -> (u32, u32) {
    let file = std::fs::File::open(f).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut loc: u32 = 0;
    let mut audit_lines: u32 = 0;

    for l in lines {
        if let Ok(l) = l {
            loc += 1;
            // starting out with a very basic check
            if !l.trim().is_empty() && !l.trim().starts_with("//") {
                if !l.trim().starts_with("#[test]") {
                    audit_lines += 1;
                } else {
                    break; // TODO: need to update to use AST to find test
                }
            }
        }
    }
    return (loc, audit_lines);
}

pub fn validate_github_url(url: &String) -> bool {
    url.starts_with("https://github.com/") || url.starts_with("git@github.com:")
}
pub fn process_remote_repo(url: &String) -> PathBuf {
    let repo_name = url.split("/").last().unwrap();

    let repo_path = PathBuf::from(format!("/tmp/{}", repo_name));

    let repo = Repository::clone(url, repo_path.clone());

    if let Ok(repo) = repo {
        println!("Cloned {} to {:?}", url, repo.path());
    } else {
        panic!("Error cloning {}, -- {}", url, repo.err().unwrap());
    }

    repo_path
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
