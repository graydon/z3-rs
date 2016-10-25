use z3_sys::*;
use Context;
use Symbol;
use Sort;
use ListSort;
use EnumSort;
use Z3_MUTEX;
use std::ptr;
use std::ffi::{CStr,CString};
use std::fmt::{Formatter,Display};
use std;

use context;

unsafe fn check_sort(ctx: &Context, sort: Z3_sort) -> Z3_sort {
    if sort.is_null() { context::check_error(ctx) };
    sort
}

unsafe fn func_decl_inc_ref(ctx: &Context, decl: Z3_func_decl) {
    let ast = Z3_func_decl_to_ast(ctx.z3_ctx, decl);
    if ast.is_null() { context::check_error(ctx) };
    Z3_inc_ref(ctx.z3_ctx, ast);
}

pub unsafe fn func_decl_dec_ref(ctx: &Context, decl: Z3_func_decl) {
    let ast = Z3_func_decl_to_ast(ctx.z3_ctx, decl);
    // Shouldn't fail, but can't panic if it does fail since we are being called from Drop
    if !ast.is_null() {
        Z3_dec_ref(ctx.z3_ctx, ast)
    }
}

impl<'ctx> Sort<'ctx> {
    pub fn new(ctx: &Context, sort: Z3_sort) -> Sort {
        assert!(!sort.is_null());
        Sort {
            ctx: ctx,
            z3_sort: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                let ast = Z3_sort_to_ast(ctx.z3_ctx, sort);
                if ast.is_null() { context::check_error(ctx) };
                Z3_inc_ref(ctx.z3_ctx, ast);
                sort
            }
        }
    }

    pub fn uninterpretd(ctx: &'ctx Context, sym: &Symbol<'ctx>) -> Sort<'ctx> {
        Sort::new(ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            check_sort(ctx, Z3_mk_uninterpreted_sort(ctx.z3_ctx, sym.z3_sym))
        })
    }

    pub fn bool(ctx: &Context) -> Sort {
        Sort::new(ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            check_sort(ctx, Z3_mk_bool_sort(ctx.z3_ctx))
        })
    }

    pub fn int(ctx: &Context) -> Sort {
        Sort::new(ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            check_sort(ctx, Z3_mk_int_sort(ctx.z3_ctx))
        })
    }

    pub fn real(ctx: &Context) -> Sort {
        Sort::new(ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            check_sort(ctx, Z3_mk_real_sort(ctx.z3_ctx))
        })
    }

    pub fn bitvector(ctx: &Context, sz: u32) -> Sort {
        Sort::new(ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            check_sort(ctx, Z3_mk_bv_sort(ctx.z3_ctx, sz as ::libc::c_uint))
        })
    }

    pub fn array(ctx: &'ctx Context,
                 domain: &Sort<'ctx>,
                 range: &Sort<'ctx>) -> Sort<'ctx> {
        Sort::new(ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            check_sort(ctx, Z3_mk_array_sort(ctx.z3_ctx, domain.z3_sort, range.z3_sort))
        })
    }

    pub fn set(ctx: &'ctx Context, elt: &Sort<'ctx>) -> Sort<'ctx> {
        Sort::new(ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            check_sort(ctx, Z3_mk_set_sort(ctx.z3_ctx, elt.z3_sort))
        })
    }

    unsafe fn mk_string_symbol(ctx: &'ctx Context, s: &str) -> Z3_symbol {
        let ss = CString::new(s).unwrap();
        let p = ss.as_ptr();
        let res = Z3_mk_string_symbol(ctx.z3_ctx, p);
        if res.is_null() { context::check_error(ctx) };
        res
    }

    pub fn list(ctx: &'ctx Context, name: &str, elt: &Sort<'ctx>) -> ListSort<'ctx> {
        unsafe {
            let mut nil_decl: Z3_func_decl = ptr::null_mut();
            let mut is_nil_decl: Z3_func_decl = ptr::null_mut();
            
            let mut cons_decl: Z3_func_decl = ptr::null_mut();
            let mut is_cons_decl: Z3_func_decl = ptr::null_mut();
            
            let mut head_decl: Z3_func_decl = ptr::null_mut();
            let mut tail_decl: Z3_func_decl = ptr::null_mut();
            
            let z3_sort = {
                let guard = Z3_MUTEX.lock().unwrap();

                let name_symbol = Sort::mk_string_symbol(ctx, name);
                check_sort(ctx, Z3_mk_list_sort(
                    ctx.z3_ctx, name_symbol, elt.z3_sort,
                    &mut nil_decl    as *mut Z3_func_decl,
                    &mut is_nil_decl as *mut Z3_func_decl,
                    &mut cons_decl    as *mut Z3_func_decl,
                    &mut is_cons_decl as *mut Z3_func_decl,
                    &mut head_decl as *mut Z3_func_decl,
                    &mut tail_decl as *mut Z3_func_decl
                ))
            };

            func_decl_inc_ref(ctx, nil_decl);
            func_decl_inc_ref(ctx, is_nil_decl);
            func_decl_inc_ref(ctx, cons_decl);
            func_decl_inc_ref(ctx, is_cons_decl);
            func_decl_inc_ref(ctx, head_decl);
            func_decl_inc_ref(ctx, tail_decl);

            ListSort {
                sort: Sort::new(ctx, z3_sort),
                nil_decl: nil_decl, is_nil_decl: is_nil_decl,
                cons_decl: cons_decl, is_cons_decl: is_cons_decl,
                head_decl: head_decl, tail_decl: tail_decl
            }
        }
    }

    pub fn enumeration(ctx: &'ctx Context, name: &str, value_names: &Vec<&str>) -> EnumSort<'ctx> {
        unsafe {
            let value_names_symbols : Vec<Z3_symbol> = value_names.iter().map(|s| Sort::mk_string_symbol(ctx, s)).collect();
            let owned_value_names : Vec<String> = value_names.iter().map(|s| s.to_string()).collect();

            let mut enum_consts:  Vec<Z3_func_decl> = Vec::with_capacity(value_names.len());
            let mut enum_testers: Vec<Z3_func_decl> = Vec::with_capacity(value_names.len());
            
            let z3_sort = {
                let guard = Z3_MUTEX.lock().unwrap();

                let name_symbol = Sort::mk_string_symbol(ctx, name);
                check_sort(ctx, Z3_mk_enumeration_sort(
                    ctx.z3_ctx, name_symbol, value_names_symbols.len() as u32,
                    value_names_symbols.as_slice().as_ptr(),
                    enum_consts .as_mut_slice().as_mut_ptr(),
                    enum_testers.as_mut_slice().as_mut_ptr()
                ))
            };

            enum_consts .set_len(value_names.len());
            enum_testers.set_len(value_names.len());

            {
                for decl in (&enum_consts).into_iter().chain((&enum_testers).into_iter()) {
                    func_decl_inc_ref(ctx, *decl);
                }
            }

            EnumSort {
                sort: Sort::new(ctx, z3_sort),
                value_names: owned_value_names.into_boxed_slice(),
                consts: enum_consts.into_boxed_slice(),
                testers: enum_testers.into_boxed_slice()
            }
        }
    }
}

impl<'ctx> Display for Sort<'ctx> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            let res = Z3_sort_to_string(self.ctx.z3_ctx, self.z3_sort);
            if res.is_null() { context::check_error(self.ctx) };
            formatter.write_str(CStr::from_ptr(res).to_string_lossy().as_ref())
        }
    }
}

impl<'ctx> Drop for Sort<'ctx> {
    fn drop(&mut self) {
        unsafe {
            debug!("drop sort {:p}", self.ctx.z3_ctx);
            let guard = Z3_MUTEX.lock().unwrap();

            let ast = Z3_sort_to_ast(self.ctx.z3_ctx, self.z3_sort);
            // Z3_sort_to_ast is a cast and can't fail right now, but
            // we don't want to panic in a Drop impl so swallow the
            // possibility anyway
            if !ast.is_null() {
                Z3_dec_ref(self.ctx.z3_ctx, ast);
            }
        }
    }
}
