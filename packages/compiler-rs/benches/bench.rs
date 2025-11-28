use compiler_rs::transform::transform;
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_compile(b: &mut Criterion) {
  b.bench_function("compile", |b| {
    b.iter(|| {
      transform(
        &format!(
          "<>{}</>",
          "<Comp
            foo={foo}
            ref={foo}
            onClick={()=> alert(1)}
            v-show={true}
            v-model={foo}
            v-once
            v-slot={foo}
          >
            <div
              v-if={foo}
              v-for={({item}, index) in list}
              key={key}
            >
              {item}
            </div>
            <span v-else-if={bar}>
              bar
            </span>
            <Foo v-else>
              default
              <template v-slot:bar={{ bar }}>
                {bar}
              </template>
            </Foo>
          </Comp>"
            .repeat(12)
        ),
        None,
      )
    })
  });
}

criterion_group!(benches, bench_compile);
criterion_main!(benches);
