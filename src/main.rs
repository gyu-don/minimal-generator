use minimal_generator::{Co, Gen};

async fn fib_producer(mut co: Co<u64>) {
    let mut a = 1u64;
    let mut b = 1u64;
    loop {
        co.yield_(a).await;
        (a, b) = (b, a + b);
    }
}

fn main() {
    // 先頭10個のfibの中身を取り出したい
    for n in Gen::new(fib_producer).into_iter().take(10) {
        println!("{}", n);
    }
}
