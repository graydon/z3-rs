#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate lazy_static;

extern crate z3_sys;
extern crate libc;

extern crate num;

use std::sync::Mutex;
use std::ffi::CString;
use z3_sys::*;

mod sort;
mod config;
mod context;
mod symbol;
mod ast;
mod solver;
mod optimize;
mod model;
mod enumsort;
mod listsort;
mod funcdecl;

// Z3 appears to be only mostly-threadsafe, a few initializers
// and such race; so we mutex-guard all access to the library.
lazy_static! {
    static ref Z3_MUTEX: Mutex<()> = Mutex::new(());
}

pub struct Config {
    kvs: Vec<(CString,CString)>,
    z3_cfg: Z3_config
}

pub struct Context {
    z3_ctx: Z3_context
}

pub struct Symbol<'ctx>
{
    ctx: &'ctx Context,
    cst: Option<CString>,
    z3_sym: Z3_symbol
}

pub struct Sort<'ctx>
{
    ctx: &'ctx Context,
    z3_sort: Z3_sort
}

pub struct ListSort<'ctx>
{
    sort: Sort<'ctx>,
    nil_decl: Z3_func_decl,
    is_nil_decl: Z3_func_decl,
    cons_decl: Z3_func_decl,
    is_cons_decl: Z3_func_decl,
    head_decl: Z3_func_decl,
    tail_decl: Z3_func_decl
}

pub struct EnumSort<'ctx>
{
    sort: Sort<'ctx>,
    value_names: Box<[String]>,
    consts: Box<[Z3_func_decl]>,
    testers: Box<[Z3_func_decl]>
}

pub struct Ast<'ctx>
{
    ctx: &'ctx Context,
    z3_ast: Z3_ast
}

pub struct FuncDecl<'ctx>
{
    ctx: &'ctx Context,
    z3_func_decl: Z3_func_decl
}

pub struct Solver<'ctx>
{
    ctx: &'ctx Context,
    z3_slv: Z3_solver
}

pub struct Model<'ctx>
{
    ctx: &'ctx Context,
    z3_mdl: Z3_model
}

pub struct Optimize<'ctx>
{
    ctx: &'ctx Context,
    z3_opt: Z3_optimize
}

