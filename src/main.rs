fn main() {
    let number = 7;
    
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // The following will cause an error
    // if number {
    //     println!("number was 7");
    // }
    //Rust will not automatically try to convert non-Boolean types to a boolean.
    // you must be explicit.

    if number != 0 {
        println!("number was something other than zero");
    }

    let number1 = 6;
     
    if number1 % 4 == 0{
        println!("number is divisible by 4");
    } else if number1 % 3 == 0 {
        println!("number is divisable by 3");
    } else if number1 % 2 == 0 {
        println!("number is divisable by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    //Once the condtion is met it skips the rest of the conditions
    let condtion = true;
    let number2 = if condtion {5} else {6};
    println!("The value of number is: {number2}");

    //The following will cause an error
    // let number2 = if condtion {5} else {"six"};
    // println!("The value of number is: {number2}");

    //The following will loop forever until ctr + C
    // loop {
    //     println!("again!")
    // }
}
