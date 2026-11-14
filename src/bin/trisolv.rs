use polybench_rs::linear_algebra::solvers::trisolv::bench;

fn bench_and_print<const N: usize>() {
    let dims = format!("{:?}", (N));
    let elapsed = bench::<N>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "trisolv", dims, elapsed);
}

fn main() {
    bench_and_print::<4000>();
}
