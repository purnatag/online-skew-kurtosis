/*use std::io;
fn push(x : i32, mut stream: &[i32], mut n : i32, mut stats: &[f64]) ->  &[i32]{
    let n1 = n - 1;
    let delta = (x as f64) - stats[0];
    let delta_n = delta/(n as f64) ;
    let delta_n2 = delta_n * delta_n ;
    let term1 = delta * delta_n * delta_n2;
    stats[0] += delta_n;
    stats[3] += term1*delta_n2*((n*n - 3*n + 3) as f64) + 6.0*delta_n2 *stats[1] - 4.0*delta_n*stats[2];
    stats[2] += term1*delta_n*((n - 2) as f64) - 3.0*delta_n*stats[1];
    stats[1] += term1;

    let mut newstream:[i32;n] = [0;n];
    for i in 0..n-1{
        newstream[i] = stream[i];
    }
    newstream[n-1] = x;

    return &mut newstream;
}
*/
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
    println!("Curr mean: {}", stats[0]);

    return  *stats;
}

fn get_mean(stats:&mut [f64]) -> f64{
    return stats[0];
}

fn get_var(n:i32, stats:&mut [f64]) -> f64{
    return stats[1]/((n as f64) - 1.0);
}

fn get_sk(n:i32, stats:&mut [f64]) -> f64{
    return (((n as f64).sqrt())*stats[2])/(stats[1].powf(1.5));
}

fn get_kurt(n:i32, stats:&mut [f64]) -> f64{
    return ((n as f64)*stats[3])/(stats[2]*stats[2] - 3.0);
}

fn main() {
    /*let mut cont = "Y";
    let mut n = 0;
    let mut stats: [f64; 4] = [0.0; 4];
    let mut stream:[i32;1] = [0];
    while cont == "Y" || cont == "y" {
        println!("Enter a number:");
        let mut input = String::new();
        n = n + 1;
        io::stdin()
            .read_line(&mut input)
            .expect("Error in reading user input!");
        let x = input.trim().parse();
        let mut newstream:[i32;n] = push(x, &mut stream, n,  &mut stats);
        stream = newstream;

        println!("Current stream stats:");
        println!("Mean:{}", getMean(n, &mut stats));
        println!("Variance:{}", getVar(n, &mut stats));
        println!("Skewness:{}", getSk(n, &mut stats));
        println!("Kurtosis:{}", getKurt(n, &mut stats));
        println!("Enter 1 if you want to continue:");
        io::stdin()
            .read_line(&mut cont)
            .expect("Error in reading user input!");    
    }*/


    let mut arr:[f64;4] = [0.0;4];
    let n = arr.len();
    //let val = n as f64;
    let mut stats: [f64;4] = [0.0;4];

    for i in 0..4{
        arr[n-i-1] = (n-i) as f64;
        stats = compute_stats(i,&arr, &mut stats);
        println!("index: {} val: {}", n-i-1, arr[n-i-1]);
    }

    println!("Current stream stats:");
    println!("Mean:{}", get_mean(&mut stats));
    println!("Variance:{}", get_var(n as i32, &mut stats));
    println!("Skewness:{}", get_sk(n as i32, &mut stats));
    println!("Kurtosis:{}", get_kurt(n as i32, &mut stats));
}

