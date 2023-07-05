use criterion::{black_box, criterion_group, criterion_main, Criterion};
use target::parse::parse_target;

macro_rules! bench {
    ($c:expr, $a:expr) => {
        $c.bench_function(&format!("parse_target({:?})", $a), |b| {
            b.iter(|| parse_target(black_box($a)))
        });
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // Valid targets
    bench!(c, "...");
    bench!(c, "foo/bar/baz:hello");
    bench!(c, "foo/bar/baz:...");

    // Valid, long targets
    bench!(
        c,
        "foo/bar/baz/qux/quux/corge/grault/garply:where-is-waldo-i-really-need-to-find-waldo-now"
    );
    bench!(
        c,
        "foo/bar/baz/qux/quux/corge/grault/garply/where-is-waldo-i-really-need-to-find-waldo-now:..."
    );

    // Invalid targets
    bench!(c, "foo/bar/baz:hello/world");
    bench!(c, "foo/bar:baz/qux");

    // really long invalid targets
    bench!(
        c,
        "foo/bar/baz/qux/quux/corge/grault/garp ly:where-is-waldo-i-really-need-to-find-waldo-now"
    );
    bench!(
        c,
        "foo/bar/baz/qux/quux/corge/grault/garp ly/where-is-waldo-i-really-need-to-find-waldo-now:..."
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
