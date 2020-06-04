# mybench

[![Crates.io](https://img.shields.io/crates/l/toolbox.svg)](https://github.com/dvshapkin/mybench/LICENSE)

Simple (and very primitive) benchmarking macro.

### Use cases:

- `bench!(wrapper, "Prompt")`
    calculates average execution time for code inside `wrapper` function for 10,000 times.

- `bench!(wrapper, number_of_repetitions, "Prompt")`
    calculates average execution time for code inside `wrapper` function for `number_of_repetitions` times.

Result is displayed as `filename:row:col 'Prompt' xxx.yy ms`

### Examples

<pre><code>
#[macro_use]
extern crate mybench;

#[test]
fn bench_ok() {
    bench!(wrapper, "Prompt 1");
    bench!(wrapper, 100_000, "Prompt 2");
}

fn wrapper() {
    for i in 0..1000 {
        let _ = i*i;
    }
}
</code></pre>

Output:

<pre><code>
running 1 test
tests\mybench.rs:6:5 'Prompt 1' 29.581µs
tests\mybench.rs:7:5 'Prompt 2' 29.693µs
test bench_ok ... ok
</code></pre>

If you don't see stdout, try this: `cargo test -- --show-output`

If you see:
<pre><code>
test bench_ok ... test bench_ok has been running for over 60 seconds
</code></pre>
Don't worry, this is an intermediate result (the test is not yet completed). Wait a little longer and let the test end.
