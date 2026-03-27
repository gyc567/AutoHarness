use autoharness::engine::synthesis::{
    CachedEvaluator, CodeSynthesisEngine, ParallelEvaluator, SimpleEvaluator, SynthesisConfig,
};
use autoharness::engine::Evaluator;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_evaluator(c: &mut Criterion) {
    let evaluator = SimpleEvaluator::new();
    let code = "fn test() { let x = 1; x + 1 }";

    c.bench_function("simple_evaluate", |b| {
        b.iter(|| evaluator.evaluate(code).unwrap());
    });
}

fn bench_cached_evaluator(c: &mut Criterion) {
    let inner = SimpleEvaluator::new();
    let cached = CachedEvaluator::new(inner, 1000);
    let code = "fn test() { let x = 1; x + 1 }";

    c.bench_function("cached_evaluate_first", |b| {
        b.iter(|| cached.evaluate(code).unwrap());
    });

    c.bench_function("cached_evaluate_cached", |b| {
        let _ = cached.evaluate(code).unwrap();
        b.iter(|| cached.evaluate(code).unwrap());
    });
}

fn bench_parallel_evaluator(c: &mut Criterion) {
    let inner = SimpleEvaluator::new();
    let parallel = ParallelEvaluator::new(inner, 4);
    let codes: Vec<String> = (0..10)
        .map(|i| format!("fn test_{}() {{ {} }}", i, i))
        .collect();

    c.bench_function("parallel_evaluate_batch_10", |b| {
        b.iter(|| parallel.evaluate_batch(&codes));
    });
}

fn bench_synthesis(c: &mut Criterion) {
    let mut group = c.benchmark_group("synthesis");

    for iterations in [5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(iterations),
            iterations,
            |b, &iters| {
                let config = SynthesisConfig::new()
                    .with_max_iterations(iters)
                    .with_convergence_threshold(0.8);

                b.iter(|| {
                    let mut engine = CodeSynthesisEngine::new(config.clone());
                    let evaluator = SimpleEvaluator::new();
                    let _ = engine.synthesize("fn test() {}", &evaluator);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_evaluator,
    bench_cached_evaluator,
    bench_parallel_evaluator,
    bench_synthesis
);
criterion_main!(benches);
