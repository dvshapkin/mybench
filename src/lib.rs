use std::time::{Instant, Duration};

#[macro_export]
macro_rules! bench {
    ($func:expr, $description:expr) => {
        bench!($func, 10_000, $description);
    };
    ($func:expr, $count:expr, $description:expr) => {
        let n: u32 = $count;
        let start = Instant::now();
        for _ in 0..n {
            $func();
        }
        let stop = Instant::now();
        log($description, stop.saturating_duration_since(start) / n);
    }
}

fn log(description: &str, time_spent: Duration) {
    println!("{}: {:?}", description, time_spent)
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
