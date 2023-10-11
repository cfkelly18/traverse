use syn::{visit::Visit, ItemFn};

pub struct DisallowPrintlnVisitor {
    pub issues: Vec<String>,
}
// example -  unfinished
impl<'ast> Visit<'ast> for DisallowPrintlnVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast ItemFn) {
        // Check if the function contains a println! statement
        // and record an issue if it does.
        // ...

        // Traverse the rest of the function's AST.
        syn::visit::visit_item_fn(self, item_fn);
    }
}
