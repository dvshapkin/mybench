/// Measures the average code execution time in N repetitions.
/// 
/// `bench!(wrapper_func, N, "Prompt")` execute `wrapper_func` N repetitions.
/// 
/// `bench!(wrapper_func, "Prompt")` execute `wrapper_func` 10,000 repetitions.
/// 
/// The result prints to the standard output as `Prompt: xxx.yy ms`.
#[macro_export]
macro_rules! bench {
    ($func:expr, $description:expr) => {
        bench!($func, 10_000, $description);
    };
    ($func:expr, $count:expr, $description:expr) => {
        let n: u32 = $count;
        let start = std::time::Instant::now();
        for _ in 0..n {
            $func();
        }
        let elapsed = start.elapsed() / n;
        if elapsed.as_secs() > 60 {
            println!("{}: {} s", $description, elapsed.as_secs_f32())
        } else {
            println!("{}: {:?}", $description, elapsed)
        }
        
    }
}

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
