# mybench

[![Crates.io](https://img.shields.io/crates/l/toolbox.svg)](https://github.com/dvshapkin/mybench/LICENSE)

Simple (and very primitive) benchmarking macro.

### Use cases:

- bench!(wrapper_func, "Prompt")
    Сalculates average execution time for code inside `wrapper_func` function for 10,000 repetitions.

- bench!(wrapper_func, number_of_repetitions, "Prompt")
    Сalculates average execution time for code inside `wrapper_func` function for `number_of_repetitions` repetitions.

Result is displayed as `filename:row:col 'Prompt' xxx.yy ms`

### Examples

<pre><code>
extern crate mybench;

use mybench::bench;

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
