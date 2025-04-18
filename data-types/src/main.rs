fn main() {
    let v1: u32 = "42".parse().expect("Not a number");
    let v2: i64 = 12;
    let v3: u32 = 1_000_000;
    let v4: u32 = 1_000u32;
    let v5: u32 = 1_000_000_u32;
    let v6: u8 = 1u8;
    let v7: u8 = 0u8;
    let v8: u8 = 123u8;
    let v9: u8 = b'Z';

    println!("Val: {}", v9);
}
