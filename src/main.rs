use minimal_generator::Co;

async fn fib_generator(mut co: Co<u64>) {
    let mut a = 1u64;
    let mut b = 1u64;
    loop {
        co.yield_(a).await;
        (a, b) = (b, a + b);
    }
}

fn main() {
    // fibは、フィボナッチ数列を返すGeneratorとして定義したい
    // let fib = fib_generator(??);
    // 先頭10個のfibの中身を取り出したい
    // let mut gen = fib()
    // for _ in 0..10 { ... }
}
