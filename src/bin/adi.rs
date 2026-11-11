use polybench_rs::stencils::adi::bench;

fn bench_and_print<const TSTEPS: usize, const N: usize>() {
    let dims = format!("{:?}", (TSTEPS, N));
    let elapsed = bench::<N, TSTEPS>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "adi", dims, elapsed);
}

fn main() {
    bench_and_print::<500, 1000>();
}
