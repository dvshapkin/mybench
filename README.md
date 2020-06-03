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
