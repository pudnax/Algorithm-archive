extern crate num;
extern crate num_complex;
extern crate num_traits;
extern crate rand;

use rand::Rng;

use num_complex::Complex;

// mod scalar;
// use scalar::Scalar;

use std::f64::consts::PI;

fn dft(array: &[f64]) -> Vec<Complex<f64>> {
    let size = array.len();
    let fsize = size as f64;
    let mut res = (0..size).map(|_x| Complex::from(0.)).collect::<Vec<_>>();
    // let array = array.iter().map(|x| Complex::from(x)).collect::<Vec<_>>();

    for i in 0..size {
        for j in 0..size {
            res[i] += array[j] * Complex::new(0., -2. * PI * ((i * j) as f64) / fsize).exp();
        }
    }
    res
}

// fn dft2<T: Scalar>(array: &[T]) -> Vec<Complex<f64>>
// where
//     std::vec::Vec<num::Complex<f64>>: std::iter::FromIterator<num::Complex<T>>,
// {
//     let size = array.len();
//     let fsize = size as f64;
//     let mut res = (0..size).map(|x| Complex::from(0.)).collect::<Vec<_>>();
//     let array = array.iter().map(Complex::from).collect::<Vec<_>>();

//     for i in 0..size {
//         for j in 0..size {
//             res[i] += array[j] * Complex::new(0., -2. * PI * ((i * j) as f64) / fsize).exp();
//         }
//     }
//     res
// }

fn sort_by_bit_reverse<T>(array: &mut [T]) {
    let size = array.len();

    for i in 0..size {
        let mut b = i as u32;
        b = ((b & 0xaaaa_aaaa) >> 1) | ((b & 0x5555_5555) << 1);
        b = ((b & 0xcccc_cccc) >> 2) | ((b & 0x3333_3333) << 2);
        b = ((b & 0xf0f0_f0f0) >> 4) | ((b & 0x0f0f_0f0f) << 4);
        b = ((b & 0xff00_ff00) >> 8) | ((b & 0x00ff_00ff) << 8);
        b = ((b >> 16) | (b << 16)) >> (32 - (size as f32).log2() as u32);
        if b as usize > i {
            array.swap(b as usize, i);
        }
    }
}

fn cooley_turkey(array: &[f64]) -> Vec<Complex<f64>> {
    let mut tmp = Vec::new();
    for i in 0..array.len() {
        tmp.push(Complex::from(array[i]));
    }

    fn rec_ct(array: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        let size = array.len();
        let fsize = size as f64;

        if size <= 1 {
            return array;
        }

        let even = rec_ct((0..size).step_by(2).map(|i| array[i]).collect::<Vec<_>>());
        let odd = rec_ct((1..size).step_by(2).map(|i| array[i]).collect::<Vec<_>>());

        let mut temp = (0..size).map(|_x| Complex::from(0.)).collect::<Vec<_>>();

        for i in 0..size / 2 {
            temp[i] = even[i] + Complex::new(0., -2. * PI * i as f64 / fsize).exp() * odd[i];
            temp[i + size / 2] =
                even[i] - Complex::new(0., -2. * PI * i as f64 / fsize).exp() * odd[i];
        }
        temp
    }

    rec_ct(tmp)
}

fn iter_cooley_turkey(tmp: &[f64]) -> Vec<Complex<f64>> {
    let mut array = Vec::new();
    for i in 0..tmp.len() {
        array.push(Complex::from(tmp[i]));
    }

    let size = array.len();
    let fsize = size as f64;
    sort_by_bit_reverse(&mut array);

    let mut stride = 2;
    while stride <= size {
        let w = Complex::new(0., -2. * PI / stride as f64).exp();

        for j in (0..size).step_by(stride) {
            let mut v = Complex::from(1.);
            for k in 0..stride / 2 {
                array[k + j + stride / 2] = array[k + j] - v * array[k + j + stride / 2];
                array[k + j] -= array[k + j + stride / 2] - array[k + j];
                v *= w;
            }
        }
        stride *= 2;
    }
    array
}

fn main() {
    let mut rng = rand::thread_rng();

    let initial = (0..64).map(|_x| rng.gen::<f64>()).collect::<Vec<_>>();

    let recursive = cooley_turkey(&initial);
    let iterative = iter_cooley_turkey(&initial);

    println!("\tidx\trec\tit\tsubstracted");
    for i in 0..initial.len() {
        println!(
            "\t{}\t{:.6e}\t{:.6e}\t{:.6e}",
            i,
            recursive[i].norm_sqr(),
            iterative[i].norm_sqr(),
            (recursive[i].norm_sqr() - iterative[i].norm_sqr())
        )
    }

    // let some: Complex<f64> = 10. + Complex::i();

    // let veci32: Vec<i32> = (0..10).map(|x| x as i32).collect();
    // let vecf32: Vec<f32> = (0..10).map(|x| x as f32).collect();
    // let vecc32: Vec<Complex<usize>> = (0..10).map(Complex::from).collect();

    // println!("{:?}", some.exp());
    // println!("{:?}", veci32);
    // println!("{:?}", vecf32);
    // println!("{:?}", vecc32);

    // println!("{:?}", send(&vecf32));
    // println!("{:?}", 5. * Complex::new(0., 2.));
    // println!("{:?}", dft2(&[6., 7.]));
}

fn send<T: num::Num>(some: &[T]) -> &T {
    &some[0]
}

