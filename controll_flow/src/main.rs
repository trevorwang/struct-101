fn main() {
    println!("Hello, world!");

    if_flow();
    loop_with_result();
    while_flow();
    for_flow();
    for_flow2();
}

fn if_flow() {
    let x = 10;
    if x % 4 == 0 {
        println!(" x is dividable by 4")
    } else if x % 3 == 0 {
        println!("number is divisible by 3");
    } else if x % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if with let
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn _loop_flow() {
    loop {
        println!("Again")
    }
}

fn loop_with_result() {
    let mut counter = 5;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result of loop is {}", result)
}

fn while_flow() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_flow() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn for_flow2() {
    for i in 1..=4 {
        println!("{}", i)
    }

    for i in (1..=4).rev() {
        println!("{}", i)
    }
}
