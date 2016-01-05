#[macro_use]
extern crate log;
extern crate env_logger;

extern crate z3;
use z3::*;

#[test]
fn test_config() {
    let _ = env_logger::init();
    let mut c = Config::new();
    c.set_proof_generation(true);
}

#[test]
fn test_context() {
    let _ = env_logger::init();
    let mut cfg = Config::new();
    cfg.set_proof_generation(true);
    let _ = Context::new(&cfg);
}

#[test]
fn test_sorts_and_symbols() {
    let _ = env_logger::init();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let _ = ctx.named_int_const("x");
    let _ = ctx.named_int_const("y");
}

#[test]
fn test_solving() {
    let _ = env_logger::init();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = ctx.named_int_const("x");
    let y = ctx.named_int_const("y");

    let solver = Solver::new(&ctx);
    solver.assert(&x.gt(&y));
    assert!(solver.check());
}

#[test]
fn test_solving_for_model() {
    let _ = env_logger::init();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = ctx.named_int_const("x");
    let y = ctx.named_int_const("y");
    let zero = ctx.from_i64(0);
    let two = ctx.from_i64(2);
    let seven = ctx.from_i64(7);

    let solver = Solver::new(&ctx);
    solver.assert(&x.gt(&y));
    solver.assert(&y.gt(&zero));
    solver.assert(&y.rem(&seven)._eq(&two));
    solver.assert(&x.add(&[&two]).gt(&seven));
    assert!(solver.check());

    let model = solver.get_model();
    let xv = model.eval(&x).unwrap().as_i64().unwrap();
    let yv = model.eval(&y).unwrap().as_i64().unwrap();
    info!("x: {}", xv);
    info!("y: {}", yv);
    assert!(xv > yv);
    assert!(yv % 7 == 2);
    assert!(xv + 2 > 7);
}

#[test]
fn test_bitvector_from_str() {
    let _ = env_logger::init();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = ctx.named_bitvector_const("x", 32);
    let zero = ctx.from_str("0", &ctx.bitvector_sort(32));
    let one = ctx.from_str("1", &ctx.bitvector_sort(32));
    let two = ctx.from_str("2", &ctx.bitvector_sort(32));
    let four = ctx.from_str("4", &ctx.bitvector_sort(32));

    let solver = Solver::new(&ctx);
    solver.assert(&x.bvugt(&zero));
    solver.assert(&x._eq(&two));
    solver.assert(&x.bvult(&four));
    solver.assert(&x.bvshl(&one)._eq(&four));
    assert!(solver.check());

    let model = solver.get_model();
    let xv = model.eval(&x).unwrap().as_i64().unwrap();
    assert!(xv == 2);
}

#[test]
fn test_bitvector_u64() {
    let _ = env_logger::init();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = ctx.named_bitvector_const("x", 8);
    let zero = ctx.bitvector_from_u64(0, 8);
    let one = ctx.bitvector_from_u64(1, 8);

    let solver = Solver::new(&ctx);
    solver.assert(&x._eq(&zero.bvadd(&one)));
    assert!(solver.check());

    let model = solver.get_model();
    let xv = model.eval(&x).unwrap().as_i64().unwrap();
    assert!(xv == 1);


}
