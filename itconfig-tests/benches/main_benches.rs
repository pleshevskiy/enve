use criterion::{Criterion, criterion_main, criterion_group, Fun};
use std::env;

#[macro_use]
extern crate lazy_static;



fn setup_env_var(initial: &String) {
    env::set_var("TEST", initial);
}


fn source_get_env() -> u32 {
    itconfig::get_env::<u32>("TEST").unwrap()
}


fn lazy_get_env() -> u32 {
    lazy_static! {
        static ref RES: u32 = source_get_env();
    };

    return *RES;
}


fn source_vs_lazy(c: &mut Criterion) {
    let source = Fun::new("source", |b, initial: &String| {
        setup_env_var(initial);
        let int: u32 = initial.parse().unwrap();

        b.iter(move || {
            assert_eq!(source_get_env(), int)
        })
    });
    let lazy = Fun::new("lazy", |b, initial: &String| {
        setup_env_var(initial);
        let int: u32 = initial.parse().unwrap();

        b.iter(move || {
            assert_eq!(lazy_get_env(), int);
        })
    });

    let funcs = vec![source, lazy];

    c.bench_functions("get_env", funcs, "1".to_string());
}



criterion_group! {
    benches,
    source_vs_lazy,
}

criterion_main!(benches);
