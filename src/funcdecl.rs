use z3_sys::*;
use Sort;
use Ast;
use FuncDecl;
use Context;
use Symbol;
use Z3_MUTEX;
use std::ffi::{CStr,CString};
use std::fmt::{Formatter,Display};
use std;

use ast;
use context;
use num::FromPrimitive;

impl<'ctx> FuncDecl<'ctx> {
    pub fn wrap(ctx: &Context, func_decl: Z3_func_decl) -> FuncDecl {
        assert!(!func_decl.is_null());
        FuncDecl {
            ctx: ctx,
            z3_func_decl: unsafe {
                debug!("new func decl {:p}", func_decl);
                let guard = Z3_MUTEX.lock().unwrap();
                let ast = Z3_func_decl_to_ast(ctx.z3_ctx, func_decl);
                if ast.is_null() { context::check_error(ctx) };
                Z3_inc_ref(ctx.z3_ctx, ast);
                func_decl
            }
        }
    }

    pub fn new(sym: &Symbol<'ctx>,
               domain: &[&Sort<'ctx>], range: &Sort<'ctx>) -> FuncDecl<'ctx> {
        let ctx = sym.ctx;
        let domain_sorts = domain.iter().map(|sort| sort.z3_sort).collect::<Vec<Z3_sort>>();
        let len32 = FromPrimitive::from_usize(domain.len()).unwrap();

        FuncDecl {
            ctx: sym.ctx,
            z3_func_decl: unsafe {
                debug!("new func decl {:p}", sym);
                let guard = Z3_MUTEX.lock().unwrap();

                let func_decl = Z3_mk_func_decl(ctx.z3_ctx, sym.z3_sym, len32, domain_sorts.as_ptr(), range.z3_sort);
                if func_decl.is_null() { context::check_error(ctx) };

                let ast = Z3_func_decl_to_ast(ctx.z3_ctx, func_decl);
                if ast.is_null() { context::check_error(ctx) };
                Z3_inc_ref(ctx.z3_ctx, ast);

                func_decl
            }
        }
    }

    pub fn fresh(ctx: &'ctx Context, prefix: &str,
                 domain: &[&Sort<'ctx>], range: &Sort<'ctx>) -> FuncDecl<'ctx> {
        let pp = CString::new(prefix).unwrap();
        let domain_sorts = domain.iter().map(|sort| sort.z3_sort).collect::<Vec<Z3_sort>>();
        let len32 = FromPrimitive::from_usize(domain.len()).unwrap();

        FuncDecl {
            ctx: ctx,
            z3_func_decl: unsafe {
                debug!("new func decl {:p}", prefix);
                let guard = Z3_MUTEX.lock().unwrap();

                let func_decl = Z3_mk_fresh_func_decl(ctx.z3_ctx, pp.as_ptr(), len32, domain_sorts.as_ptr(), range.z3_sort);
                if func_decl.is_null() { context::check_error(ctx) };

                let ast = Z3_func_decl_to_ast(ctx.z3_ctx, func_decl);
                if ast.is_null() { context::check_error(ctx) };
                Z3_inc_ref(ctx.z3_ctx, ast);

                func_decl
            }
        }
    }

    pub fn app(&self, args: &[&Ast<'ctx>]) -> Ast<'ctx> {
        let args_asts = args.iter().map(|ast| ast.z3_ast).collect::<Vec<Z3_ast>>();
        let len32 = FromPrimitive::from_usize(args.len()).unwrap();

        Ast::new(self.ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            ast::check_ast(self.ctx, Z3_mk_app(self.ctx.z3_ctx, self.z3_func_decl, len32, args_asts.as_ptr()))
        })
    }
}

impl<'ctx> Display for FuncDecl<'ctx> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            let res = Z3_func_decl_to_string(self.ctx.z3_ctx, self.z3_func_decl);
            if res.is_null() { context::check_error(self.ctx) };
            formatter.write_str(CStr::from_ptr(res).to_string_lossy().as_ref())
        }
    }
}

impl<'ctx> Drop for FuncDecl<'ctx> {
    fn drop(&mut self) {
        unsafe {
            debug!("drop sort {:p}", self.ctx.z3_ctx);
            let guard = Z3_MUTEX.lock().unwrap();

            let ast = Z3_func_decl_to_ast(self.ctx.z3_ctx, self.z3_func_decl);
            // Z3_func_decl_to_ast is a cast and can't fail right now, but
            // we don't want to panic in a Drop impl so swallow the
            // possibility anyway
            if !ast.is_null() {
                Z3_dec_ref(self.ctx.z3_ctx, ast);
            }
        }
    }
}
