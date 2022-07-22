use criterion::{criterion_group, criterion_main, Criterion, Fun};
use std::env;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itconfig;

fn setup_env_var(key: &'static str, initial: String) {
    env::set_var(key, initial);
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
    setup_env_var("TEST", "1".to_string());

    let source = Fun::new("source", |b, _| {
        b.iter(move || assert_eq!(source_get_env(), 1))
    });
    let lazy = Fun::new("lazy", |b, _| {
        b.iter(move || {
            assert_eq!(lazy_get_env(), 1);
        })
    });

    c.bench_functions("get_env", vec![source, lazy], 0);
}

fn source_macro_vs_lazy_macro(c: &mut Criterion) {
    config! {
        TEST: &'static str,
        TEST_WITH_DEFAULT: &'static str => "default",

        static LAZY_TEST: &'static str,
        static LAZY_TEST_WITH_DEFAULT: &'static str => "default",
    }

    setup_env_var("TEST", "test".to_string());
    setup_env_var("LAZY_TEST", "test".to_string());

    let source = Fun::new("source", |b, _| {
        b.iter(move || {
            assert_eq!(config::TEST(), "test");
        })
    });
    let lazy = Fun::new("lazy", |b, _| {
        b.iter(move || {
            assert_eq!(config::LAZY_TEST(), "test");
        })
    });
    let source_with_default = Fun::new("source_with_default", |b, _| {
        b.iter(move || {
            assert_eq!(config::TEST_WITH_DEFAULT(), "default");
        })
    });
    let lazy_with_default = Fun::new("lazy_with_default", |b, _| {
        b.iter(move || {
            assert_eq!(config::LAZY_TEST_WITH_DEFAULT(), "default");
        })
    });

    let funcs = vec![source, lazy, source_with_default, lazy_with_default];

    c.bench_functions("macro", funcs, 0);
}

criterion_group! {
    benches,
    source_vs_lazy,
    source_macro_vs_lazy_macro,
}

criterion_main!(benches);
