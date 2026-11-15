use rand::{Rng, RngExt, distr::Alphanumeric};
use rand_distr::{Distribution, LogNormal, Normal, Uniform};
use std::println;

pub fn generate_random_values() {
    let mut rng = rand::rng();
    let random_number: u8 = rng.random();
    println!("Random value: {random_number}");
}

pub fn generate_random_numbers_within_a_range() {
    let mut rng = rand::rng();
    println!("Integer: {}", rng.random_range(0..10));
    println!("Float: {}", rng.random_range(0.0..10.0));
}

pub fn generate_random_number_within_a_range_uniform() {
    let mut rng = rand::rng();
    let die =
        Uniform::new_inclusive(1, 6).expect("Failed to create uniform distribution: invalid range");

    let float_die = Uniform::new_inclusive(0.0, 6.0)
        .expect("Failed to create uniform distribution: invalid range");

    let throw = die.sample(&mut rng);
    println!("Uniform Integer: {}", throw);

    let float_random = float_die.sample(&mut rng);
    println!("Unform Float: {}", float_random);

    // loop {
    //     let throw = die.sample(&mut rng);
    //     println!("Roll the die: {}", throw);
    //     if throw == 0 {
    //         break;
    //     }
    // }
}

pub fn generate_random_numbers_with_given_distribution() {
    let mut rng = rand::rng();
    let normal = Normal::new(2.0, 3.0).expect("Failed to create normal distribtion");
    let log_normal = LogNormal::new(1.0, 0.5).expect("Failed to create log-normal distributin");

    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
    let v = log_normal.sample(&mut rng);
    println!("{} is from an ln N(1, 0.25) distribtion", v);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn random<R: Rng>(rng: &mut R) -> Self {
        Point {
            x: rng.random(),
            y: rng.random(),
        }
    }
}

pub fn generate_random_values_of_a_custom_type() {
    let mut rng = rand::rng();

    let rand_tuple = rng.random::<(i32, bool, f64)>();
    let rand_point = Point::random(&mut rng);
    println!("Random  tuple: {:?}", rand_tuple);
    // println!("Random tuple: {:#?}", rand_tuple);

    println!("Random Point: {:?}", rand_point);
    println!("Random Point: {:#?}", rand_point);
}

pub fn create_random_password_from_a_set_of_alphanumeric_characters() {
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    println!("Random password is: {}", password);
}

pub fn create_random_password_from_a_set_of_user_defined_characters() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;

    let mut rng = rand::rng();
    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            char::from(CHARSET[idx])
        })
        .collect();
    println!("Random password by a set of user-defined: {password}");
}
