use z3_sys::*;
use Ast;
use Sort;
use ListSort;
use Z3_MUTEX;

use ast;

impl<'ctx> ListSort<'ctx> {
    pub fn sort(&self) -> Sort<'ctx> {
        Sort {
            ctx: self.ctx,
            z3_sort: self.z3_sort
        }
    }

    pub fn head(&self, list: &Ast<'ctx>) -> Ast<'ctx> {
        Ast::new(self.ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            ast::check_ast(self.ctx, Z3_mk_app(self.ctx.z3_ctx, self.head_decl, 1, &list.z3_ast as *const Z3_ast))
        })
    }
}
