use std::{assert_eq, println};

pub fn sort_vector_integers() {
    let mut vec = vec![1, 5, 10, 2, 15];
    println!("{:?}", vec);

    vec.sort();
    println!("{:?}", vec);
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}

pub fn sort_vector_float() {
    let mut vec = vec![1.1_f64, 1.15, 5.5, 1.123, 2.0];
    println!("float vec: {:?}", vec);

    vec.sort_by(|a, b| a.total_cmp(b));
    println!("float vec: {:?}", vec);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

pub fn sort_vecrot_struct() {
    let mut people = vec![
        Person::new("zoe", 25),
        Person::new("al", 60),
        Person::new("john", 1),
    ];
    println!("before sort people: {:?}", people);

    // sort people by derived natural order (Name and age)
    people.sort();

    println!("after sort people: {:?}", people);
}
