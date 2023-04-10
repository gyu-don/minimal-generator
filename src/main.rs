fn main() {
    // yieldは、作り方が分からないので、panicする関数として作っておく
    let yield_ = |_| { unimplemented!() };
    // fibは、フィボナッチ数列を返すGeneratorとして定義したい
    let fib = || {
        let mut a = 1;
        let mut b = 1;
        loop {
            yield_(a);
            (a, b) = (b, a + b);
        }
    };
    // 先頭10個のfibの中身を取り出したい
    // let mut gen = fib()
    // for _ in 0..10 { ... }
}
