fn main() {
    let mut x = 0;
    let mut y = 1;
    println!("0"); // hardcoded for omptimization. n0 = 0
    for _i in 0..10 {
        let z = x + y;
        println!("{}", z);
        y = x;
        x = z;
    }
}