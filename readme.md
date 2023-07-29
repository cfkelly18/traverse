# Traverse

Traverse is a Rust tool to analyze CosmWasm code and extract useful information.

## Notice
Please audit and test before using, this is a work in progress and hacked togeter quickly to get some initial results. Lacking proper error handing, tests, and edge case handling.
## Features

- Scope local or remote cosmwasm repository for audit
- Parses Rust code using the syn crate
- Finds all functions in a crate [in progress]
- Identifies entry point functions like `execute` in CosmWasm contracts
- Builds a call graph to show relationships between functions [in progress]
- Runs custom static analysis checks [in progress]

## Usage

Traverse can be used as a library or a CLI. 

The --scope argument takes a local filesystem path to analyze.


## Running for a remote repo:
`cargo run -- --url https://github.com/CosmWasm/cw-plus.git`
- Clones repo to a temp directory
- Runs analysis on the temp directory
- Cleans up temp directory

## Running for a local repo:
`cargo run -- --scope /path/to/repo`
- Runs analysis on the local directory
