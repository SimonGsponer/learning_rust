
fn square(input: u16) -> u8{
    return (input * input).try_into().unwrap()
}


fn main() {
    println!("Hello, world!");

    let y = 5;

    let mut x = square(y);
    println!("{x}");
    x = 4;
    println!("{x}");
}
