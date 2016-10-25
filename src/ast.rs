use z3_sys::*;
use Context;
use Sort;
use Symbol;
use Ast;
use Z3_MUTEX;
use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};
use std::ffi::{CStr, CString};
use std::fmt::{Display, Formatter};
use std;
use libc::c_uint;
use num::FromPrimitive;

use context;

macro_rules! unop {
    ( $f:ident, $z3fn:ident ) => {
        pub fn $f(&self) -> Ast<'ctx> {
            Ast::new(self.ctx, unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                check_ast(self.ctx, $z3fn(self.ctx.z3_ctx, self.z3_ast))
            })
    }
    };
}

macro_rules! binop {
    ( $f:ident, $z3fn:ident ) => {
        pub fn $f(&self, other: &Ast<'ctx>) -> Ast<'ctx> {
            Ast::new(self.ctx, unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                check_ast(self.ctx, $z3fn(self.ctx.z3_ctx, self.z3_ast, other.z3_ast))
            })
    }
    };
}

macro_rules! trinop {
    ( $f:ident, $z3fn:ident ) => {
        pub fn $f(&self, a: &Ast<'ctx>, b: &Ast<'ctx>) -> Ast<'ctx> {
            Ast::new(self.ctx, unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                check_ast(self.ctx, $z3fn(self.ctx.z3_ctx, self.z3_ast, a.z3_ast, b.z3_ast))
            })
    }
    };
}

macro_rules! varop {
    ( $f:ident, $z3fn:ident ) => {
        pub fn $f(&self, other: &[&Ast<'ctx>]) -> Ast<'ctx> {
            Ast::new(self.ctx, unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                let mut tmp = vec![self.z3_ast];
                for a in other {
                    tmp.push(a.z3_ast)
                }
                assert!(tmp.len() <= 0xffffffff);
                check_ast(self.ctx, $z3fn(self.ctx.z3_ctx, tmp.len() as u32, tmp.as_ptr()))
            })
    }
    };
}

pub unsafe fn check_ast(ctx: &Context, ast: Z3_ast) -> Z3_ast {
    if ast.is_null() {
        context::check_error(ctx)
    };
    ast
}

impl<'ctx> Ast<'ctx> {
    pub fn new(ctx: &Context, ast: Z3_ast) -> Ast {
        assert!(!ast.is_null());
        Ast {
            ctx: ctx,
            z3_ast: unsafe {
                debug!("new ast {:p}", ast);
                let guard = Z3_MUTEX.lock().unwrap();
                Z3_inc_ref(ctx.z3_ctx, ast);
                ast
            }
        }
    }

    pub fn new_const(sym: &Symbol<'ctx>,
                     sort: &Sort<'ctx>) -> Ast<'ctx> {
        Ast::new(sym.ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            check_ast(sym.ctx, Z3_mk_const(sym.ctx.z3_ctx, sym.z3_sym, sort.z3_sort))
        })
    }

    pub fn fresh_const(ctx: &'ctx Context,
                       prefix: &str,
                       sort: &Sort<'ctx>) -> Ast<'ctx> {
        Ast::new(ctx, unsafe {
            let pp = CString::new(prefix).unwrap();
            let p = pp.as_ptr();
            let guard = Z3_MUTEX.lock().unwrap();
            check_ast(ctx, Z3_mk_fresh_const(ctx.z3_ctx, p, sort.z3_sort))
        })
    }

    pub fn bound(index: c_uint, sort: &'ctx Sort) -> Ast<'ctx> {
        Ast::new(sort.ctx, unsafe {
            check_ast(sort.ctx, Z3_mk_bound(sort.ctx.z3_ctx, index, sort.z3_sort))
        })
    }

    pub fn forall_bound(bound: &[(&Symbol<'ctx>, &Ast<'ctx>)], body: &Ast<'ctx>) -> Ast<'ctx> {
        let len32 = FromPrimitive::from_usize(bound.len()).unwrap();

        Ast::new(body.ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            let sorts = bound.iter().map(|&(_, ref ast)| {
                let s = Z3_get_sort(body.ctx.z3_ctx, ast.z3_ast);
                if s.is_null() { context::check_error(body.ctx) }
                s
            }).collect::<Vec<Z3_sort>>();
            let decl_names = bound.iter().map(|&(ref sym, _)| sym.z3_sym).collect::<Vec<Z3_symbol>>();
            check_ast(body.ctx, Z3_mk_forall(body.ctx.z3_ctx, 0, 0, std::ptr::null(), len32, sorts.as_ptr(), decl_names.as_ptr(), body.z3_ast))
        })
    }

    pub fn forall_const(bound: &[&Ast<'ctx>], body: &Ast<'ctx>) -> Ast<'ctx> {
        let len32 = FromPrimitive::from_usize(bound.len()).unwrap();

        Ast::new(body.ctx, unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            let symbols = bound.iter().map(|&ast| {
                let a = Z3_to_app(body.ctx.z3_ctx, ast.z3_ast);
                if a.is_null() { context::check_error(body.ctx) }
                a
            }).collect::<Vec<Z3_app>>();
            check_ast(body.ctx, Z3_mk_forall_const(body.ctx.z3_ctx, 1, len32, symbols.as_ptr(), 0, std::ptr::null(), body.z3_ast))
        })
    }

    pub fn from_bool(ctx: &'ctx Context, b: bool) -> Ast<'ctx> {
            Ast::new(ctx, unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                check_ast(ctx, if b { Z3_mk_true(ctx.z3_ctx) } else { Z3_mk_false(ctx.z3_ctx) })
            })
    }

    pub fn from_i64(ctx: &'ctx Context, i: i64) -> Ast<'ctx> {
            Ast::new(ctx, unsafe {
                let sort = ctx.int_sort();
                let guard = Z3_MUTEX.lock().unwrap();
                check_ast(ctx, Z3_mk_int64(ctx.z3_ctx, i, sort.z3_sort))
            })
    }

    pub fn from_u64(ctx: &'ctx Context, u: u64) -> Ast<'ctx> {
            Ast::new(ctx, unsafe {
                let sort = ctx.int_sort();
                let guard = Z3_MUTEX.lock().unwrap();
                check_ast(ctx, Z3_mk_unsigned_int64(ctx.z3_ctx, u, sort.z3_sort))
            })
    }

    pub fn from_real(ctx: &'ctx Context, num: i32, den: i32) -> Ast<'ctx> {
            Ast::new(ctx, unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                check_ast(ctx, Z3_mk_real(ctx.z3_ctx,
                                   num as ::libc::c_int,
                                   den as ::libc::c_int))
            })
    }

    pub fn as_bool(&self) -> Option<bool> {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            match Z3_get_bool_value(self.ctx.z3_ctx, self.z3_ast) {
                Z3_L_TRUE => Some(true),
                Z3_L_FALSE => Some(false),
                Z3_L_UNDEF => { context::check_error(self.ctx); None },
                _ => None
            }
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            let mut tmp : ::libc::c_longlong = 0;
            if Z3_TRUE == Z3_get_numeral_int64(self.ctx.z3_ctx,
                                               self.z3_ast, &mut tmp) {
                Some(tmp)
            } else {
                context::check_error(self.ctx);
                None
            }
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            let mut tmp : ::libc::c_ulonglong = 0;
            if Z3_TRUE == Z3_get_numeral_uint64(self.ctx.z3_ctx,
                                                self.z3_ast, &mut tmp) {
                Some(tmp)
            } else {
                context::check_error(self.ctx);
                None
            }
        }
    }

    pub fn as_real(&self) -> Option<(i64,i64)> {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            let mut num : i64 = 0;
            let mut den : i64 = 0;
            if Z3_TRUE == Z3_get_numeral_small(self.ctx.z3_ctx,
                                               self.z3_ast,
                                               &mut num, &mut den) {
                Some((num,den))
            } else {
                context::check_error(self.ctx);
                None
            }
        }
    }

    pub fn as_app(&self) -> Option<(Ast<'ctx>, Vec<Ast<'ctx>>)> {
        unsafe {
            let is_app = Z3_is_app(self.ctx.z3_ctx, self.z3_ast);
            if is_app == 0 {
                context::check_error(self.ctx);
                None
            } else {
                let app = Z3_to_app(self.ctx.z3_ctx, self.z3_ast);
                if app.is_null() { context::check_error(self.ctx) };

                let decl = Z3_get_app_decl(self.ctx.z3_ctx, app);
                if decl.is_null() { context::check_error(self.ctx) };

                let decl_ast = Z3_func_decl_to_ast(self.ctx.z3_ctx, decl);
                if decl_ast.is_null() { context::check_error(self.ctx) };

                let nargs = Z3_get_app_num_args(self.ctx.z3_ctx, app);
                if nargs <= 0 { context::check_error(self.ctx) };

                let mut arg_asts = Vec::with_capacity(FromPrimitive::from_u32(nargs).unwrap());
                for i in 0..nargs {
                    let arg = Z3_get_app_arg(self.ctx.z3_ctx, app, i);
                    if arg.is_null() { context::check_error(self.ctx) };
                    arg_asts.push(Ast::new(self.ctx, arg))
                }

                Some((Ast::new(self.ctx, decl_ast), arg_asts))
            }
        }
    }

    varop!(distinct, Z3_mk_distinct);

    // Boolean ops
    trinop!(ite, Z3_mk_ite);
    binop!(iff, Z3_mk_iff);
    binop!(implies, Z3_mk_implies);
    binop!(xor, Z3_mk_xor);
    varop!(and, Z3_mk_and);
    varop!(or, Z3_mk_or);
    varop!(add, Z3_mk_add);
    varop!(sub, Z3_mk_sub);
    varop!(mul, Z3_mk_mul);
    unop!(not, Z3_mk_not);

    // Numeric ops
    binop!(div, Z3_mk_div);
    binop!(rem, Z3_mk_rem);
    binop!(modulo, Z3_mk_mod);
    binop!(power, Z3_mk_power);
    unop!(minus, Z3_mk_unary_minus);
    binop!(lt, Z3_mk_lt);
    binop!(le, Z3_mk_le);
    binop!(_eq, Z3_mk_eq);
    binop!(ge, Z3_mk_ge);
    binop!(gt, Z3_mk_gt);
    unop!(int2real, Z3_mk_int2real);
    unop!(real2int, Z3_mk_real2int);
    unop!(is_int, Z3_mk_is_int);

    // Bitvector ops
    unop!(bvnot, Z3_mk_bvnot);
    unop!(bvneg, Z3_mk_bvneg);
    unop!(bvredand, Z3_mk_bvredand);
    unop!(bvredor, Z3_mk_bvredor);
    binop!(bvand, Z3_mk_bvand);
    binop!(bvor, Z3_mk_bvor);
    binop!(bvxor, Z3_mk_bvxor);
    binop!(bvnand, Z3_mk_bvnand);
    binop!(bvnor, Z3_mk_bvnor);
    binop!(bvxnor, Z3_mk_bvxnor);
    binop!(bvadd, Z3_mk_bvadd);
    binop!(bvsub, Z3_mk_bvsub);
    binop!(bvmul, Z3_mk_bvmul);
    binop!(bvudiv, Z3_mk_bvudiv);
    binop!(bvsdiv, Z3_mk_bvsdiv);
    binop!(bvurem, Z3_mk_bvurem);
    binop!(bvsrem, Z3_mk_bvsrem);
    binop!(bvsmod, Z3_mk_bvsmod);
    binop!(bvult, Z3_mk_bvult);
    binop!(bvslt, Z3_mk_bvslt);
    binop!(bvule, Z3_mk_bvule);
    binop!(bvsle, Z3_mk_bvsle);
    binop!(bvuge, Z3_mk_bvuge);
    binop!(bvsge, Z3_mk_bvsge);
    binop!(bvugt, Z3_mk_bvugt);
    binop!(bvsgt, Z3_mk_bvsgt);
    binop!(concat, Z3_mk_concat);
    binop!(bvshl, Z3_mk_bvshl);
    binop!(bvlshr, Z3_mk_bvlshr);
    binop!(bvashr, Z3_mk_bvashr);

    // Array ops
    binop!(select, Z3_mk_select);
    trinop!(store, Z3_mk_store);

    // Set ops
    binop!(set_add, Z3_mk_set_add);
    binop!(set_del, Z3_mk_set_del);
    varop!(set_union, Z3_mk_set_union);
    varop!(set_intersect, Z3_mk_set_intersect);
    binop!(set_member, Z3_mk_set_member);
    binop!(set_subset, Z3_mk_set_subset);
    unop!(set_complement, Z3_mk_set_complement);
}

impl<'ctx> Display for Ast<'ctx> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            let res = Z3_ast_to_string(self.ctx.z3_ctx, self.z3_ast);
            if res.is_null() { context::check_error(self.ctx) };
            formatter.write_str(CStr::from_ptr(res).to_string_lossy().as_ref())
        }
    }
}

impl<'ctx> Drop for Ast<'ctx> {
    fn drop(&mut self) {
        unsafe {
            debug!("drop ast {:p}", self.z3_ast);
            let guard = Z3_MUTEX.lock().unwrap();
            Z3_dec_ref(self.ctx.z3_ctx, self.z3_ast);
        }
    }
}

impl<'ctx> Hash for Ast<'ctx> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            let u = Z3_get_ast_hash(self.ctx.z3_ctx, self.z3_ast);
            context::check_error(self.ctx);
            u.hash(state);
        }
    }
}

impl<'ctx> PartialEq<Ast<'ctx>> for Ast<'ctx> {
    fn eq(&self, other: &Ast<'ctx>) -> bool {
        unsafe {
            let res = Z3_is_eq_ast(self.ctx.z3_ctx,
                                   self.z3_ast,
                                   other.z3_ast);
            context::check_error(self.ctx);
            Z3_TRUE == res
        }
    }
}

impl<'ctx> Eq for Ast<'ctx> { }
