fn main() {
    let mut number: u8 = 0b0101_0101;

    println!("Number is {number} binary is {number:08b}");

    let number_left4 = number << 4;
    println!("Number is {number_left4} \t binary is {number_left4:08b} when shifted left 4");

    let number_right4 = number >> 4;
    println!("Number is {number_right4} \t Binary is {number_right4:08b} when shifted right 4");
}

fn type_of<T>(_: &T) -> &str {
        return std::any::type_name::<T>()
}