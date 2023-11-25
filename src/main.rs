use futures::join;
use tokio::{self, task};

#[tokio::main]
async fn main() {
    do_three_things().await;
}

async fn do_three_things() {
    join!(add(4, 5), subtract(15, 12));
}

async fn add(x: i32, y: i32) {
    for _ in 0..1000 {
        println!("{}", x + y);
        task::yield_now().await;
    }
}

async fn subtract(x: i32, y: i32) {
    for _ in 0..1000 {
        println!("{}", x - y);
        task::yield_now().await;
    }
}
