fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number_2 = 3;

    if number_2 != 0 {
        println!("number was something other than zero");
    }

    let number_3 = 6;

    if number_3 % 4 == 0 {
        println!("number_3 is divisible by 4");
    } else if number_3 % 3 == 0 {
        println!("number_3 is divisible by 3");
    } else if number_3 % 2 == 0 {
        println!("number_3 is divisible by 2");
    } else {
        println!("number_3 is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number_4 = if condition { 5 } else { 6 };

    println!("The value of number_4 is: {number_4}");

    // let number = if condition { 5 } else { "six" } is error

    // Repetition with loops
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop labels
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

    // Conditional loops with while
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // for version (more safe and faster)
    for element in a {
        println!("the value is: {element}");
    }

    // for loop and range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
