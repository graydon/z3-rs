use z3_sys::*;
use Ast;
use Sort;
use ListSort;
use Z3_MUTEX;

use ast;
use sort;

impl<'ctx> ListSort<'ctx> {
    pub fn sort(&'ctx self) -> &'ctx Sort<'ctx> { &self.sort }

    pub fn head(&self, list: &Ast<'ctx>) -> Ast<'ctx> {
        Ast::new(self.sort.ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            ast::check_ast(self.sort.ctx, Z3_mk_app(self.sort.ctx.z3_ctx, self.head_decl, 1, &list.z3_ast as *const Z3_ast))
        })
    }
}

impl<'ctx> Drop for ListSort<'ctx> {
    fn drop(&mut self) {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            sort::func_decl_dec_ref(self.sort.ctx, self.nil_decl);
            sort::func_decl_dec_ref(self.sort.ctx, self.is_nil_decl);
            sort::func_decl_dec_ref(self.sort.ctx, self.cons_decl);
            sort::func_decl_dec_ref(self.sort.ctx, self.is_cons_decl);
            sort::func_decl_dec_ref(self.sort.ctx, self.head_decl);
            sort::func_decl_dec_ref(self.sort.ctx, self.tail_decl);
        }
    }
}

