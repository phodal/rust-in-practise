pub mod rule;

use std::{cell::RefCell, collections::HashMap};

fn main() {
    let mut map = HashMap::new();

    map.insert("1", RefCell::new(1));
    map.insert("2", RefCell::new(2));

    {
        let a = map.get("1").unwrap();
        println!("a: {}", a.borrow());

        let b = map.get("2").unwrap();
        println!("b: {}", b.borrow());
        *b.borrow_mut() = 5;
    }

    println!("Results: {:?}", map);
}
