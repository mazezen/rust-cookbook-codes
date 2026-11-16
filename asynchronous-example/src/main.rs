use std::{assert_eq, println};

use tokio::task::JoinSet;

struct Job {
    id: usize,
    data: String,
}

impl Job {
    fn new(id: usize) -> Self {
        let data = "d".repeat(id);
        Self { id, data }
    }

    async fn process(&mut self) {
        // do some work, like fetching more data over the network
        self.data = self.data.replace("d", "!!");
        println!("Id {} -> {}", self.id, self.data);
    }
}

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    const JOB_COUNT: usize = 5;
    let jobs = (0..JOB_COUNT).map(Job::new).collect::<Vec<Job>>();

    for mut job in jobs {
        set.spawn(async move { job.process().await });
    }

    let mut complete = 0usize;
    while set.join_next().await.is_some() {
        // wait for each job to finish
        complete += 1;
    }

    assert_eq!(complete, JOB_COUNT);
}
