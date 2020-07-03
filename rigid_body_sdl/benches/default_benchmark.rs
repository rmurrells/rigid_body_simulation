use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use rigid_body_sdl::{
    config,
    RigidBodySimulationSDL,
    RigidBodySimulationTrait,
};
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rigid_body = RigidBodySimulationSDL::new((800, 600)).unwrap();
    rigid_body.set_fps(0);
    config::default(8, &mut rigid_body).unwrap();
    
    let mut group = c.benchmark_group("bgroup");
    group.measurement_time(Duration::from_secs(30));
    //group.sample_size(10);
    group.bench_function("default", |b| b.iter(|| {
	rigid_body.tick().unwrap();
    }));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
