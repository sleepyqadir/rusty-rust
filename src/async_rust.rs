use std::fmt::Display;
use tokio::task::JoinHandle;

pub async fn async_rust() {
    let simple_msg = get_string();

    println!("{}", simple_msg);

    let async_msg = async_get_string();

    println!("future {}", async_msg.await);
    async_closure().await;

    let mark_twain = "Samuel Clemens".to_owned();

    async_print(mark_twain).await;
}
fn get_string() -> String {
    "simple string".to_owned()
}

async fn async_get_string() -> String {
    "async simple string".to_owned()
}

async fn async_closure() {
    let message = "==========================> First";

    let closure = || async {
        println!("{}", message);
    };

    let first = closure();
    println!("==========================> Second");
    println!("==========================> Third");
    first.await
}

fn async_print<T: Display + Send + 'static>(msg: T) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        println!("{}", msg);
    })
}
