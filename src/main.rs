pub mod rmap;

use crate::rmap::RMap;

fn main() {
    let mut m1 = RMap::new(5, String::from("foo"));
    m1.add(7, String::from("bar"));
    m1.add(3, String::from("baz"));

    println!("{:#?}", m1);

    let mut m2 = RMap::new(3, String::from("ay"));
    m2.add(5, String::from("all"));
    m2.add(1, String::from("bell"));

    println!("{:#?}", m2);

    let res = m2.get_value(1);
    println!("{:#?}", res);
}
