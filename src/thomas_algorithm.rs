fn thomas(a: &[f64], b: &[f64], c: &[f64], x: &mut [f64]) {
    let size: usize = a.len();

    let mut y = vec![0.; size];

    y[0] = c[0] / b[0];
    x[0] /= b[0];

    for i in 1..size {
        let scale = 1. / (b[i] - a[i] * y[i - 1]);
        y[i] = c[i] * scale;
        x[i] = (x[i] - a[i] * x[i - 1]) * scale;
    }

    for i in (0..(size - 1)).rev() {
        x[i] -= y[i] * x[i + 1]
    }
}

fn main() {
    let a = vec![0.0, 2.0, 3.0];
    let b = vec![1.0, 3.0, 6.0];
    let c = vec![4.0, 5.0, 0.0];
    let mut x = vec![7.0, 5.0, 3.0];

    println!(
        "The system
[1.0  4.0  0.0][x] = [7.0]
[2.0  3.0  5.0][y] = [5.0]
[0.0  3.0  6.0][z] = [3.0]
has the solution"
    );

    thomas(&a, &b, &c, &mut x);

    println!("{:?}", x);
}
