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