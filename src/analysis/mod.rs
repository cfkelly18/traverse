use std::collections::{HashMap, HashSet};
use std::env::args;
use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};
use syn::{parse_file, Arm, Expr, ExprCall, ExprPath, File, ItemFn, Meta, Pat, Receiver, Token};
mod disallow_println;
pub use disallow_println::DisallowPrintlnVisitor;
// TODO: add Result struct

struct FunctionVisitor;

impl<'ast> Visit<'ast> for FunctionVisitor {
    fn visit_item_fn(&mut self, i: &syn::ItemFn) {
        println!("Function: {}", i.sig.ident);
    }
}
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ExecuteEntryPoint {
    name: String,
    children: Vec<Function>,
}

struct EntryPointVisitor {
    entrypoints: HashSet<String>,
    functions: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
    name: String,
    is_entry_point: bool,
    is_test: bool,
}
impl Function {
    fn new(name: String, is_entry_point: bool, is_test: bool) -> Self {
        Function {
            name,
            is_entry_point,
            is_test,
        }
    }
}

impl<'ast> Visit<'ast> for EntryPointVisitor {
    fn visit_item_fn(&mut self, i: &ItemFn) {
        let new_function = Function::new(i.sig.ident.to_string(), false, false);
        self.functions.push(new_function.clone());

        if i.sig.ident.to_string() == "execute" {
            for stmt in i.block.stmts.iter() {
                //println!("\nStatement:{:#?}", stmt);
                if let syn::Stmt::Expr(expr, ..) = stmt {
                    if let syn::Expr::Match(syn::ExprMatch { ref arms, .. }) = *expr {
                        for a in arms.iter() {
                            self.visit_arm(a);
                        }
                    }
                }
            }
        }
    }
    // Currently just naively picks up on the path segment
    fn visit_arm(&mut self, arm: &Arm) {
        match &arm.pat {
            Pat::Struct(pat) => {
                if pat.path.segments.first().unwrap().ident.to_string() == "ExecuteMsg" {
                    self.entrypoints
                        .insert(pat.path.segments.last().unwrap().ident.to_string());
                }
            }

            _ => {}
        }
    }
}
struct CallGraphVisitor {
    entrypoints: HashSet<String>,
}

impl<'ast> Visit<'ast> for CallGraphVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        // visit function body to find function calls

        if self.entrypoints.contains(&node.sig.ident.to_string()) {
            // syn::visit::visit_block(&mut function_calls, &*node.block);
            // println!("Function calls: {:?}", function_calls.function_calls);
            //print!("Matched an Entrypoint fn: {:?}", node.sig.ident);
        }
        // syn::visit::visit_block(&mut self.function_calls, &*node.block);

        // // continue with traversal
        // syn::visit::visit_item_fn(self, node);
    }

    // fn visit_expr(&mut self, node: &'ast Expr) {
    //     if let Expr::Call(call) = node {
    //         // add function call to list
    //         if let Expr::Path(path) = &*call.func {
    //             let ident = &path.path.segments.last().unwrap().ident;
    //             self.function_calls.push(ident.to_string());
    //         }
    //     }

    //     // continue with traversal
    //     syn::visit::visit_expr(self, node);
    // }
}

pub struct Analyzer {
    results: HashMap<String, usize>,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        Analyzer {
            results: HashMap::new(),
        }
    }
    pub fn analyze(&mut self, ast: syn::File) {
        let mut visitor = FunctionVisitor;
        visitor.visit_file(&ast);
    }
    pub fn get_entrypoints(&mut self, ast: syn::File) -> HashSet<String> {
        let mut visitor = EntryPointVisitor {
            entrypoints: HashSet::new(),
            functions: Vec::new(),
        };
        visitor.visit_file(&ast);

        return visitor.entrypoints;
    }

    pub fn get_call_graph(&mut self, ast: syn::File) {
        let entry_points = self.get_entrypoints(ast.clone());
        let mut visitor = CallGraphVisitor {
            entrypoints: entry_points.clone(),
        };
        println!("Call Graph: {:#?}", entry_points);
        visitor.visit_file(&ast);
    }
    pub fn run_static_analysis(&mut self, ast: syn::File) {
        let mut visitors = vec![DisallowPrintlnVisitor { issues: vec![] }];
    }
}
