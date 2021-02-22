extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}
