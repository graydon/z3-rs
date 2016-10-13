use z3_sys::*;
use Ast;
use Sort;
use ListSort;
use Z3_MUTEX;
use std::fmt::{Formatter,Display};
use std;

use ast;
use sort;

impl<'ctx> ListSort<'ctx> {
    pub fn sort(&'ctx self) -> &'ctx Sort<'ctx> { &self.sort }

    fn zop(&self, decl: Z3_func_decl) -> Ast<'ctx> {
        Ast::new(self.sort.ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            ast::check_ast(self.sort.ctx, Z3_mk_app(self.sort.ctx.z3_ctx, decl, 0, std::ptr::null()))
        })
    }

    fn unop(&self, decl: Z3_func_decl, list: &Ast<'ctx>) -> Ast<'ctx> {
        Ast::new(self.sort.ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            ast::check_ast(self.sort.ctx, Z3_mk_app(self.sort.ctx.z3_ctx, decl, 1, &list.z3_ast as *const Z3_ast))
        })
    }

    fn binop(&self, decl: Z3_func_decl, x1: &Ast<'ctx>, x2: &Ast<'ctx>) -> Ast<'ctx> {
        Ast::new(self.sort.ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            let args: [Z3_ast; 2] = [x1.z3_ast, x2.z3_ast];
            ast::check_ast(self.sort.ctx, Z3_mk_app(self.sort.ctx.z3_ctx, decl, 2, &args as *const Z3_ast))
        })
    }

    pub fn nil(&self) -> Ast<'ctx> {
        self.zop(self.nil_decl)
    }

    pub fn cons(&self, head: &Ast<'ctx>, tail: &Ast<'ctx>) -> Ast<'ctx> {
        self.binop(self.cons_decl, head, tail)
    }

    pub fn head(&self, list: &Ast<'ctx>) -> Ast<'ctx> {
        self.unop(self.head_decl, list)
    }

    pub fn tail(&self, list: &Ast<'ctx>) -> Ast<'ctx> {
        self.unop(self.tail_decl, list)
    }

    pub fn is_nil(&self, list: &Ast<'ctx>) -> Ast<'ctx> {
        self.unop(self.is_nil_decl, list)
    }

    pub fn is_cons(&self, list: &Ast<'ctx>) -> Ast<'ctx> {
        self.unop(self.is_cons_decl, list)
    }
}

impl<'ctx> Display for ListSort<'ctx> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        self.sort().fmt(formatter)
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

