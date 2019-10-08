extern crate rand;

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

fn bubble_sort(array: &mut [u32]) {
    let n = array.len();

    for _ in 0..n {
        for j in 1..n {
            if array[j - 1] > array[j] {
                array.swap(j, j - 1);
            }
        }
    }
}

fn main() {
    let rng = thread_rng();
    let num_range = Uniform::new_inclusive(0, 1000);
    let mut rand_vec: Vec<u32> = rng.sample_iter(&num_range).take(10).collect();

    println!("Before sorting: {:?}", rand_vec);
    bubble_sort(&mut rand_vec);
    println!("After sorting: {:?}", rand_vec);
}
