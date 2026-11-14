use polybench_rs::linear_algebra::kernels::_3mm::bench;

fn bench_and_print<
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
    const E: usize,
>() {
    let dims = format!("{:?}", (A, B, C, D, E));
    let elapsed = bench::<A, B, C, D, E>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "3mm", dims, elapsed);
}

fn main() {
    bench_and_print::<1600, 1800, 2000, 2200, 2400>();
}
