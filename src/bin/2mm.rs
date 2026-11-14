use polybench_rs::linear_algebra::kernels::_2mm::bench;

fn bench_and_print<const NI: usize, const NJ: usize, const NK: usize, const NL: usize>() {
    let dims = format!("{:?}", (NI, NJ, NK, NL));
    let elapsed = bench::<NI, NJ, NK, NL>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "2mm", dims, elapsed);
}

fn main() {
    bench_and_print::<1600, 1800, 2200, 2400>();
}
