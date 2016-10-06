use z3_sys::*;
use Sort;
use Ast;
use EnumSort;
use Z3_MUTEX;
use std::ptr;

use ast;

impl<'ctx> EnumSort<'ctx> {
    pub fn sort(&self) -> Sort<'ctx> {
        Sort {
            ctx: self.ctx,
            z3_sort: self.z3_sort
        }
    }

    pub fn value(&self, name: &str) -> Ast<'ctx> {
        let (n, _) = self.value_names.iter().enumerate().filter(|&(n, nm)| nm.eq(name)).next().unwrap();
        Ast::new(self.ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            ast::check_ast(self.ctx, Z3_mk_app(self.ctx.z3_ctx, self.consts[n], 0, ptr::null()))
        })
    }
}
