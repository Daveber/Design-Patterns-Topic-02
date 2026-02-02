fn main() {
    let number: u8 = 0b0101_0101;
    println!("Number is {number} and the type is {}", type_of(&number));
    println!("Number as Binary {number:08b}");
}

fn type_of<T>(_: &T) -> &str {
        return std::any::type_name::<T>()
}