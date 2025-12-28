use compiler_rs::{TransformOptions, transform};
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_vdom(b: &mut Criterion) {
  let source = format!(
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
  );

  b.bench_function("vdom", |b| {
    b.iter(|| {
      transform(
        &source,
        Some(TransformOptions {
          interop: true,
          ..Default::default()
        }),
      )
    })
  });
}

criterion_group!(benches, bench_vdom);
criterion_main!(benches);
