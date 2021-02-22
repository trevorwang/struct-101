fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut y = vec![1, 3, 2];
    y.push(100);
    println!("Hello, world! {:?}, {:?}", v, y);

    let third = &y[2];
    println!("the third value is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There's no third element!"),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // println!("the third value is {:?}", does_not_exist);

    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    let first = &v[0];
    println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    //map

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores);
}
