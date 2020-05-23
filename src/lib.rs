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
        let stop = std::time::Instant::now();
        println!("{}: {:?}", $description, stop.saturating_duration_since(start) / n)
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
