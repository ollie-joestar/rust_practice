fn  greater() {
    println!("Sometimes I wonder whether I should retire. I would have guessed higher.")
}

fn  less() {
    println!("This student might not be as smart as I was told. This answer is obviously too weak.")
}

fn  equal(n: i32) {
    println!("That is right! The secret was indeed the number {n}, which you have brilliantly discovered!");
}

fn  start() {
    println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.")
}

fn main() {
    start();
    let number = ftkit::random_number(-2147483648..2147483647);
    let mut user_number:i32 = ftkit::read_number();
    while user_number != number {
        match user_number.cmp(&number) {
            std::cmp::Ordering::Greater => greater(),
            std::cmp::Ordering::Less => less(),
            std::cmp::Ordering::Equal => break,
        }
        user_number = ftkit::read_number();
    }
    equal(number);
}
