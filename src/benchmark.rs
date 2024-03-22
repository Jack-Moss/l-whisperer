use criterion::{black_box, criterion_group, criterion_main,Criterion};
use mycrate::test_populate_circular_buffer;
//https://bheisler.github.io/criterion.rs/book/getting_started.html
pub fn criterion_benchmark(c: &mut Criterion){
    c.bench_function("test_databuffer", |b| b.iter(||test_populate_circular_buffer()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);