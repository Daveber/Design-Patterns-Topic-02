fn main() {
    let mut number: u8 = 0b0101_0101;

    println!("Number is {number} binary is {number:08b}");

    number = number | 0b0000_1111;
    println!("bit 7 Number is {number} binary is {number:08b}");
}

fn type_of<T>(_: &T) -> &str {
        return std::any::type_name::<T>()
}