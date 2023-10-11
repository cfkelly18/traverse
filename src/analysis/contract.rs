use std::collections::HashSet;

use git2::Error;
use gix::config::Path;
use syn::{visit::Visit, ItemFn, Expr, parse_str};

pub fn get_entrypoints(node: &ItemFn) {
    
    if node.sig.ident == "execute" {
        
        for stmt in node.block.stmts.clone() {
            if let syn::Stmt::Expr(expr, ..) = stmt {
                if let syn::Expr::Match(syn::ExprMatch { ref arms, .. }) = expr {
                    for a in arms.iter() {
                        get_arm_expression(a)
                    }
                }
            }
        }
    }
        
   
}
// Base expression for pulling entrypoint from execute match 
pub fn get_arm_expression(arm: &syn::Arm)  {
    if let syn::Pat::TupleStruct( PatTupleStruct) = arm.pat.clone() {
        let res = get_expr_fn_name(*arm.body.clone());
        print!("{:?} -> {:?}\n\n",PatTupleStruct.path.segments.last().unwrap().ident, res);
        
            
    }
}
// Take an Expr and return the function name
// Get righthand function name form a match branch
// Return a Result with string or error   
pub fn get_expr_fn_name(expr: Expr) -> Result<String, &'static str> {

    if let Expr::Call(syn::ExprCall { func, .. }) = expr.clone() {
      if let Expr::Path(syn::ExprPath { path, .. }) = &*func {
        if let Some(ident) = path.get_ident() {
          return Ok(ident.to_string());
        }
      }
    }
    println!("{:#?}", expr.clone() );
    Err("Not a function call")
  }





mod test {
    #[test]
    fn test_get_expr_fn_name() {
        let s = r#"Expr::Call { attrs: [], func: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(execute), arguments: PathArguments::None }, PathSep, PathSegment { ident: Ident(job), arguments: PathArguments::None }, PathSep, PathSegment { ident: Ident(execute_job), arguments: PathArguments::None }] } }, paren_token: Paren, args: [Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(deps), arguments: PathArguments::None }] } }, Comma, Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(env), arguments: PathArguments::None }] } }, Comma, Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(info), arguments: PathArguments::None }] } }, Comma, Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(data), arguments: PathArguments::None }] } }] }"#;
        let expr = syn::parse_str::<syn::Expr>(s);

        if let Ok(expr) = expr {
            let res = super::get_expr_fn_name(expr);
            print!("{:?}", res);
            assert_eq!(res, Ok("execute_job".to_string()));
        }
    }
}
