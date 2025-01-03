use criterion::{black_box, criterion_group, criterion_main, Criterion};
use triple_buffer::TripleBuffer;

pub fn benchmark(c: &mut Criterion) {
    let (mut input, output) = TripleBuffer::<u8>::default().split();

    {
        let mut uncontended = c.benchmark_group("uncontended");
        uncontended.bench_function("read output", |b| b.iter(|| *output.output_buffer()));
        uncontended.bench_function("clean update", |b| {
            b.iter(|| {
                output.update();
            })
        });
        uncontended.bench_function("clean receive", |b| b.iter(|| *output.read()));
        uncontended.bench_function("write input", |b| {
            b.iter(|| {
                *input.input_buffer() = black_box(0);
            })
        });
        uncontended.bench_function("send", |b| b.iter(|| *input.input_buffer() = black_box(0)));
        uncontended.bench_function("transmit", |b| {
            b.iter(|| {
                *input.input_buffer() = black_box(0);
                *output.read()
            })
        });
    }

    {
        let (mut input, output) = TripleBuffer::<u8>::default().split();
        let mut read_contended = c.benchmark_group("read contention");
        testbench::run_under_contention(
            move || black_box(*output.read()),
            || {
                read_contended.bench_function("write input", |b| {
                    b.iter(|| {
                        *input.input_buffer() = black_box(0);
                    })
                });
                read_contended
                    .bench_function("send", |b| b.iter(|| *input.input_buffer() = black_box(0)));
            },
        );
    }

    {
        let (mut input, output) = TripleBuffer::<u8>::default().split();
        let mut write_contended = c.benchmark_group("write contention");
        testbench::run_under_contention(
            || *input.input_buffer() = black_box(0),
            || {
                write_contended
                    .bench_function("read output", |b| b.iter(|| *output.output_buffer()));
                write_contended.bench_function("update", |b| {
                    b.iter(|| {
                        output.update();
                    })
                });
                write_contended.bench_function("receive", |b| b.iter(|| *output.read()));
            },
        );
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
