use z3_sys::*;
use Solver;
use Optimize;
use Model;
use Ast;
use Z3_MUTEX;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};
use std;

use context;

impl<'ctx> Model<'ctx> {
    pub fn of_solver(slv: &Solver<'ctx>) -> Model<'ctx> {
        Model {
            ctx: slv.ctx,
            z3_mdl: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                let m = Z3_solver_get_model(slv.ctx.z3_ctx, slv.z3_slv);
                if m.is_null() { context::check_error(slv.ctx) };
                Z3_model_inc_ref(slv.ctx.z3_ctx, m);
                m
            }
        }
    }

    pub fn of_optimize(opt: &Optimize<'ctx>) -> Model<'ctx> {
        Model {
            ctx: opt.ctx,
            z3_mdl: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                let m = Z3_optimize_get_model(opt.ctx.z3_ctx, opt.z3_opt);
                if m.is_null() { context::check_error(opt.ctx) };
                Z3_model_inc_ref(opt.ctx.z3_ctx, m);
                m
            }
        }
    }

    pub fn eval(&self, ast: &Ast<'ctx>) -> Option<Ast<'ctx>> {
        unsafe {
            let mut tmp : Z3_ast = ast.z3_ast;
            let res;
            {
                let guard = Z3_MUTEX.lock().unwrap();
                res = Z3_model_eval(self.ctx.z3_ctx,
                                    self.z3_mdl,
                                    ast.z3_ast,
                                    Z3_TRUE,
                                    &mut tmp)
            }
            if res == Z3_TRUE {
                Some(Ast::new(self.ctx, tmp))
            } else {
                context::check_error(self.ctx);
                None
            }
        }
    }
}

impl<'ctx> Display for Model<'ctx> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            let res = Z3_model_to_string(self.ctx.z3_ctx, self.z3_mdl);
            if res.is_null() { context::check_error(self.ctx) };
            formatter.write_str(CStr::from_ptr(res).to_string_lossy().as_ref())
        }
    }
}

impl<'ctx> Drop for Model<'ctx> {
    fn drop(&mut self) {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            Z3_model_dec_ref(self.ctx.z3_ctx, self.z3_mdl);
        }
    }
}
