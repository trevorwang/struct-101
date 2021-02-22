fn main() {
    mutate_variable();
    tuple_test();

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array is {}", a[0]);
    let r = plus_one(5);
    let y = r;
    println!("{} plus one is {}", 5, r)
}

fn mutate_variable() {
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);
}

fn _parse() {
    let guess: u64 = "422d".parse().expect("Not a number");
    println!("The number you guess is {}", guess)
}

fn tuple_test() {
    let tup = (1, 3, 4);
    println!("value is {}", tup.0)
}

fn plus_one(x: i32) -> i32 {
    // return x + 1;
    x + 1
}
