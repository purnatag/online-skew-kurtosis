fn compute_stats(i: usize, arr: & [f64], stats: &mut [f64;4]) -> [f64;4]{
    let s = arr.len();
    let n = (i as f64) + 1.0;
    let delta = arr[s-i-1] - stats[0];
    let delta_n = delta/n ;
    let delta_n2 = delta_n * delta_n ;
    let term1 = delta * delta_n * (i as f64);
    stats[0] += delta_n;
    stats[3] += term1*delta_n2*(3.0 + n*n - 3.0*n) + 6.0*delta_n2*stats[1] - 4.0*delta_n*stats[2];
    stats[2] += term1*delta_n*(n - 2.0) - 3.0*delta_n*stats[1];
    stats[1] += term1;
    println!("Curr variance: {}\n", stats[1]/n);

    return  *stats;
}

fn get_mean(stats:&mut [f64]) -> f64{
    return stats[0];
}

fn get_var(n: f64, stats:&mut [f64]) -> f64{
    return stats[1]/(n - 1.0);
}

fn get_sk(n: f64, stats:&mut [f64]) -> f64{
    return n.sqrt() * stats[2]/(stats[1].powf(1.5));
}

fn get_kurt(n: f64, stats:&mut [f64]) -> f64{
    return n * stats[3]/(stats[1]*stats[1]);
}

fn main() {
    let mut arr:[f64;5] = [0.0;5];
    let n = arr.len();
    let val:[f64;5] = [101.5, 33.25, 56.75, 61.5, 10.0];
    let mut stats: [f64;4] = [0.0;4];

    for i in 0..n{
        arr[n - i - 1] = val[n - i - 1];
        println!("index: {} val: {}", n-i-1, arr[n-i-1]);
        stats = compute_stats(i, &arr, &mut stats);
    }

    println!("Current stream stats:");
    println!("Mean:{}", get_mean(&mut stats));
    println!("Variance:{}", get_var(n as f64, &mut stats));
    println!("Skewness:{}", get_sk(n as f64, &mut stats));
    println!("Kurtosis:{}", get_kurt(n as f64, &mut stats));

    println!("stats1:{} stats2:{} stats3:{}", stats[1], stats[2], stats[3]);
}

