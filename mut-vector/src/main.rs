pub mod rule;

use std::{cell::RefCell, collections::HashMap};
use crate::rule::{EmptyRule, BeginRule, BasicRule};
use std::rc::Rc;

fn main() {
    // https://github.com/servo/servo/blob/master/ports/winit/app.rs
    let first_rule = EmptyRule {};
    let second_rule = BeginRule::new(1);

    let mut map: HashMap<i32, Rc<RefCell<dyn BasicRule>>> = HashMap::new();

    map.insert(1, Rc::new(RefCell::new(second_rule)));
    map.insert(2, Rc::new(RefCell::new(first_rule)));
    {
        let a = map.get(&1).unwrap().clone();
        let x = &mut *a.borrow_mut();
        x.collect_patterns_recursive(map);
    }
}
