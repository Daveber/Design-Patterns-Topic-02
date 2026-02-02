fn main() {
    //println!("The number is : {}" , is_even(501));
    //let (result, long_string)  = calculate(1 as f64, 1 as f64, "*");
    //println!("result: {result} {long_string}");
    //fizzBuzz();
    println!("is prime: {}", is_prime(29));
}

//Exercise 1: check if number is even or odd
fn is_even(number: i32) -> bool {
    if (number % 2) == 0 {
        true
    } else {
        false
    }
}


//Exercise 2: simple calculator
fn calculate(a: f64, b: f64, operator: &str) -> (f64, String) {
    let mut result = 0.0;

    if operator == "+" {
        result = a + b;
        (result, "Success".to_string())
    } else if operator == "-" {
        result = a - b;
        (result, "Success".to_string())
    } else if operator == "*" {
        result = a * b;
        (result, "Success".to_string())
    } else if operator == "/" {
        result = a / b;
        (result, "Success".to_string())
    } else {
        (result, "Failure".to_string())
    }
}

//Exercise 3: Fizz Buzz
fn fizzBuzz() {
    let mut counter = 1;

    while counter <= 100 {
        if (counter % 3) == 0 && (counter % 5) == 0 {
            println!("FizzBuzz");

        } else if (counter % 3) == 0 {
            println!("Fizz");

        } else if (counter % 5) == 0 {
            println!("Buzz");

        } else {
            println!("{counter}");
        } 
        counter +=1;
    }
}

//Exercise 4: Prime Checker
fn is_prime(number: u32) -> bool {
    
    let mut division_counter = 0;

    if number <= 1 {
        return false;

    } else {
        for i in 1..=number {
            if (number % i) == 0 {
                division_counter+=1;                
            }
        }
    }

    if division_counter > 2 {
        return false;
    } else {
        return true;
    }
}
