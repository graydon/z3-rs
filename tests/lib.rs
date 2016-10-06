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
fn test_constructed_sorts() {
    println!("1");
    let _ = env_logger::init();
    println!("2");
    let cfg = Config::new();
    println!("3");
    let ctx = Context::new(&cfg);
    println!("4");
    let solver = Solver::new(&ctx);

    println!("5");
    let color = Sort::enumeration(&ctx, "Color", &vec!("Red", "Blue", "Green"));
    println!("6");
    let color_list = Sort::list(&ctx, "ColorList", &color.sort());
    println!("7");

    let color_list_sort = color_list.sort();
    println!("8");
    let alist : Ast = ctx.fresh_const("alist", &color_list_sort);
    println!("9");

    solver.assert(&color_list.head(&alist)._eq(&color.value("Red")));
    println!("10");
    assert!(solver.check());
    println!("11");

    let model = solver.get_model();
    model.eval(&color_list.head(&alist)).unwrap(); // FIXME
}
