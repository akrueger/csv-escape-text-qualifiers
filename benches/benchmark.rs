use criterion::{black_box, Criterion, criterion_group, criterion_main};
use csv_escape_text_qualifiers::process_csv;

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let input = "INDEX0001.csv";
    let output = "mark1.csv";

    let mut group = criterion.benchmark_group("csv_escape_text_qualifiers");
    group.sample_size(10);
    group.bench_function("process_csv", |b| b.iter(|| process_csv(black_box(input), black_box(output))));
}

criterion_group!(benchmark, criterion_benchmark);
criterion_main!(benchmark);