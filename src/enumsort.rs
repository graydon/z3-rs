use Sort;
use Ast;
use EnumSort;
use FuncDecl;
use Z3_MUTEX;
use std::fmt::{Formatter,Display};
use std;

use sort;

impl<'ctx> EnumSort<'ctx> {
    pub fn sort(&'ctx self) -> &'ctx Sort<'ctx> { &self.sort }

    pub fn value_decl(&self, name: &str) -> FuncDecl<'ctx> {
        let (n, _) = self.value_names.iter().enumerate().filter(|&(n, nm)| nm.eq(name)).next().unwrap();
        FuncDecl::wrap(self.sort.ctx, self.consts[n])
    }

    pub fn value(&self, name: &str) -> Ast<'ctx> {
        self.value_decl(name).app(&[])
    }

    pub fn is_value_decl(&self, name: &str) -> FuncDecl<'ctx> {
        let (n, _) = self.value_names.iter().enumerate().filter(|&(n, nm)| nm.eq(name)).next().unwrap();
        FuncDecl::wrap(self.sort.ctx, self.testers[n])
    }

    pub fn is_value(&self, name: &str, what: &Ast<'ctx>) -> Ast<'ctx> {
        self.is_value_decl(name).app(&[what])
    }
}

impl<'ctx> Display for EnumSort<'ctx> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        self.sort().fmt(formatter)
    }
}

impl<'ctx> Drop for EnumSort<'ctx> {
    fn drop(&mut self) {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            for decl in (&self.consts).into_iter().chain((&self.testers).into_iter()) {
                sort::func_decl_dec_ref(self.sort.ctx, *decl);
            }
        }
    }
}
