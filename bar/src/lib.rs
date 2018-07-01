pub fn bar(u: u8) -> u8 {
    println!("u={}", u);
    let x = u & 0x56;
    let y = !x;

    y
}