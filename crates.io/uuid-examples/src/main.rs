use uuid::{uuid, ContextV7, Timestamp, Uuid};

pub fn random_uuid() {
    let uuid = Uuid::new_v4();

    assert_eq!(Some(uuid::Version::Random), uuid.get_version());

    println!("{}", uuid);
}

pub fn sortable_uuid() {
    // If you enable the `v7` feature you can generate sortable UUIDs.
    let uuid = Uuid::now_v7();

    assert_eq!(Some(uuid::Version::SortRand), uuid.get_version());

    println!("{}", uuid);
}

pub fn sortable_uuid_bulk() {
    let ctxt = ContextV7::new();
    for _ in 0..10 {
        println!("{}", Uuid::new_v7(Timestamp::now(&ctxt)));
    }
}

pub fn uuid_macro() {
    let uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
    assert_eq!(Some(uuid::Version::Random), uuid.get_version());
    println!("{uuid}");
}

fn main() {
    random_uuid();
    sortable_uuid();
    sortable_uuid_bulk();
    uuid_macro();
}
