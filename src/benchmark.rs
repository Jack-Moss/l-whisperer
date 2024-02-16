use criterion::{black_box, criterion_group, criterion_main,Criterion};
use mycrate::main;
//https://bheisler.github.io/criterion.rs/book/getting_started.html
pub fn criterion_benchmark(c: &mut Criterion){
    c.bench_function("test_databuffer", |b| b.iter(||buffer(run_test_buffer(100))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);