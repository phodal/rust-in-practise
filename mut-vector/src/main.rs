pub mod rule;

use std::{cell::RefCell, collections::HashMap};
use crate::rule::{EmptyRule, BeginRule, BasicRule};
use std::rc::Rc;

fn main() {
    let mut map: HashMap<i32, Rc<RefCell<dyn BasicRule>>> = HashMap::new();

    let first_rule = EmptyRule {};
    let second_rule = BeginRule::new(1);

    map.insert(1, Rc::new(RefCell::new(first_rule)));
    map.insert(2, Rc::new(RefCell::new(second_rule)));
    {
        let a = map.get(&1).unwrap();
        let x = &*a.borrow();
        println!("a: {}", x.id());


        let b = map.get(&2).unwrap();
        let y = &*b.borrow_mut();
        println!("b: {}", y.id());
    }

    // println!("Results: {:?}", map);
}
