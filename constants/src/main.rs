const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn someVal() -> u32 {
    2
}

fn main() {
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    const val: u32 = someVal() + 2;
}
