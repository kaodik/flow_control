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
    // can apply the break to leave the loop
    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    //nested loops applying loop labels with single qoute '
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //while loop
    // the following way to loop through is error prone
    //if it does reach end and call for index that is out of bounds
    // it will panic.
    // it also adds the runtime code to check if the index is within bounds
    let mut num2 = 3;
    while num2 != 0 {
        println!("{num2}!");
        num2 -= 1;
    }
    println!("LIFTOFF!!!");

    //loop through array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    //use a for loop instead
    for element in a {
        println!("the value is: for {element}");
    }
}
