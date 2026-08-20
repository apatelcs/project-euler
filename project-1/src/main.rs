fn main() {
    let mut total: u64 = 0;
    const LIMIT: u64 = 1000;

    for i in 1..LIMIT {
        if (i % 3 == 0) || (i % 5 == 0) {
            total += i;
        }
    }

    println!("The sum of all multiples of 3 or 5 below {LIMIT} is {total}");
}
