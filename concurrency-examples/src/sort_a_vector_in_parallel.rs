use rand::RngExt;
use rayon::prelude::*;
use std::println;

fn main() {
    let mut vec = vec![0; 1_000_000];
    rand::rng().fill(&mut vec[..]);

    vec.par_sort_unstable();

    let first = vec.first().unwrap();
    let last = vec.last().unwrap();
    println!("first: {first}, last: {last}")
}
