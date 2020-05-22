# mybench

[![Crates.io](https://img.shields.io/crates/l/toolbox.svg)](https://github.com/dvshapkin/mybench/LICENSE)

Simple (and very primitive) benchmarking macro.

### Use cases:

- bench!(wrapper_func, "prompt")
    Сalculates average execution time for code inside `wrapper_func` function for 10,000 repetitions.
    The result is displayed as `prompt: xxx.yy ms`

- bench!(wrapper_func, number_of_repetitions, "prompt")
    Сalculates average execution time for code inside `wrapper_func` function for `number_of_repetitions` repetitions.
    The result is displayed as `prompt: xxx.yy ms`

### Examples

<pre><code>
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench_ok() {
        bench!(wrapper, "Var1");
        bench!(wrapper, 100_000, "Var2");
    }

    fn wrapper() {
        for i in 0..1000 {
            let _ = i*i;
        }
    }
}
</code></pre>
