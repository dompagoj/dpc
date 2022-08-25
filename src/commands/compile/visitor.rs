use swc_common::util::take::Take;
use swc_ecma_visit::VisitMutWith;
use swc_ecma_visit::{as_folder, Fold, VisitMut};
use swc_ecmascript::ast::VarDeclarator;

pub struct MyVisitor;

pub fn get_my_visitor() -> impl Fold {
    as_folder(MyVisitor)
}

impl VisitMut for MyVisitor {
    fn visit_mut_var_declarator(&mut self, v: &mut VarDeclarator) {
        // This is not required in this example, but you typically need this.
        v.visit_mut_children_with(self);

        v.take();
    }

    fn visit_mut_var_declarators(&mut self, vars: &mut Vec<VarDeclarator>) {
        vars.visit_mut_children_with(self);

        vars.retain(|node| {
            // We want to remove the node, so we should return false.
            if node.name.is_invalid() {
                return true;
            }

            // Return true if we want to keep the node.
            true
        });
    }

    // fn visit_mut_exprs(&mut self, exprs: &mut Vec<Expr>) {}
}
