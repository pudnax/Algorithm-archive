extern crate rand;

use std::f64::consts::PI;

fn in_circle(x: f64, y: f64, radius: f64) -> bool {
    x * x + y * y < radius * radius
}

fn monte_carlo(n: i64) -> f64 {
    let mut count = 0;

    for _ in 0..n {
        let x = rand::random();
        let y = rand::random();
        if in_circle(x, y, 1.0) {
            count += 1;
        }
    }

    // return our pi estimate
    f64::from(4 * count) / n as f64
}

fn main() {
    let pi_estimate = monte_carlo(100_000_000);

    println!(
        "Percent error is {:.3}%",
        (100.0 * (pi_estimate - PI).abs() / PI)
    );
}
