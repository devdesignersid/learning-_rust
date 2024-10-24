fn main() {
    let number = 24;

    if number % 4 == 0 {
        println!("The number {} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("The number {} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("The number {} is divisible by 2", number);
    } else {
        println!("The number {} is not divisible by 4, 3, 2", number);
    };

    let condition = false;
    // When using if condition with let statement value of each arm must be of same type.
    // In this case value of both arms must be of i32.
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {}", number);

    // An infinite loop.
    //loop {
    //    println!("Printing from the loop!")
    //}

    let mut counter = 0;

    // Returning value from loop using 'break'.
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        };
    };

    println!("The value of the counter is {}", result);

    let mut count = 0;

    // Labeled loop
    // Can use the label to break & continues inside the nested loops.
    'counting_loop: loop {
        println!("The value of count is {count}");
        let mut remaining = 10;

        loop {
            println!("The value of remaining is {remaining}");
            if remaining == 9 {
                break;
            };
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        };

        count += 1;
    };

    println!("The final value of count is {count}");

    let mut number = 3;

    while number > 0 {
        println!("{number}");
        number -= 1;
    };

    println!("LIFTOFF!!!");

    let arr: [i32; 5] = [32, 21, 4, 16, 28];

    let mut index = 0;

    // Using the while loop iterate over array elements
    // less efficient due to overhead for the compiler to check the condition
    // wether the index is out of bounds on every iteration.
    while index < 5 {
        println!("{index} : {}", arr[index]);
        index += 1;
    };

    for   item in arr {
        println!("item: {item}");
    };

    // Countdown using for loop
    for item in (1..4).rev() {
        println!("{item}");
    }

    println!("LIFTOFF!!!")
}
