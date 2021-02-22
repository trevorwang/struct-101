mod front_of_house;
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    front_of_house::hosting::add_to_waitlist();
}
