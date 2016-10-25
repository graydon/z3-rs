use Ast;
use Sort;
use ListSort;
use FuncDecl;
use Z3_MUTEX;
use std::fmt::{Formatter,Display};
use std;

use sort;

impl<'ctx> ListSort<'ctx> {
    pub fn sort(&'ctx self) -> &'ctx Sort<'ctx> { &self.sort }

    pub fn nil_decl(&self) -> FuncDecl<'ctx> {
        FuncDecl::wrap(self.sort.ctx, self.nil_decl)
    }

    pub fn nil(&self) -> Ast<'ctx> {
        self.nil_decl().app(&[])
    }

    pub fn cons_decl(&self) -> FuncDecl<'ctx> {
        FuncDecl::wrap(self.sort.ctx, self.cons_decl)
    }

    pub fn cons(&self, head: &Ast<'ctx>, tail: &Ast<'ctx>) -> Ast<'ctx> {
        self.cons_decl().app(&[head, tail])
    }

    pub fn head_decl(&self) -> FuncDecl<'ctx> {
        FuncDecl::wrap(self.sort.ctx, self.head_decl)
    }

    pub fn head(&self, list: &Ast<'ctx>) -> Ast<'ctx> {
        self.head_decl().app(&[list])
    }

    pub fn tail_decl(&self) -> FuncDecl<'ctx> {
        FuncDecl::wrap(self.sort.ctx, self.tail_decl)
    }

    pub fn tail(&self, list: &Ast<'ctx>) -> Ast<'ctx> {
        self.tail_decl().app(&[list])
    }

    pub fn is_nil_decl(&self) -> FuncDecl<'ctx> {
        FuncDecl::wrap(self.sort.ctx, self.is_nil_decl)
    }

    pub fn is_nil(&self, list: &Ast<'ctx>) -> Ast<'ctx> {
        self.is_nil_decl().app(&[list])
    }

    pub fn is_cons_decl(&self) -> FuncDecl<'ctx> {
        FuncDecl::wrap(self.sort.ctx, self.is_cons_decl)
    }

    pub fn is_cons(&self, list: &Ast<'ctx>) -> Ast<'ctx> {
        self.is_cons_decl().app(&[list])
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

