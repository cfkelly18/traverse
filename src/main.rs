mod analysis;
mod driver;
mod utils;

use std::env;
use std::path::PathBuf;

struct ScopeSource {
    url: Option<String>,
    path: Option<PathBuf>,
}

fn main() {
    let mut args = env::args();

    // Skip first argument which is program name
    args.next();

    let mut scope_source = ScopeSource {
        url: None,
        path: None,
    };
    let mut analysis = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--scope" => {
                scope_source.path = args.next().map(PathBuf::from);
            }
            "--url" => {
                scope_source.url = args.next().and_then(|s| Some(s.to_string()));
            }
            "--analysis" => {
                analysis = true;
                println!("Analysis mode enabled");
            }
            _ => println!("Unknown argument {}", arg),
        }
    }

    if let Some(url) = &scope_source.url {
        if utils::validate_github_url(&url) {
            println!("Analyzing {}", url);
            let tmp_path = utils::process_remote_repo(&url);
            print!("tmp_path: {}", tmp_path.display());
            let mut driver = driver::Driver::new(true, analysis);
            driver.set_scope(tmp_path);

            // Run analysis...
            driver.run();
        } else {
            println!("Error: {} is not a valid GitHub URL", url);
        }
    } else if let Some(path) = scope_source.path {
        if !path.is_dir() {
            println!("Error: {} is not a valid directory", path.display());
            return;
        }

        println!("Analyzing {}", path.display());

        let mut driver = driver::Driver::new(false, analysis);
        driver.set_scope(path);

        // Run analysis...
        driver.run();
    } else {
        println!("Error: --scope argument required");
    }
   
}
// cargo run -- --scope /Users/cfkelly18/DEV/cosmwasm/cw-plus/
