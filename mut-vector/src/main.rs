pub mod rule;

use std::{cell::RefCell, collections::HashMap};
use crate::rule::{EmptyRule, BeginRule, BasicRule};
use std::rc::Rc;

thread_local! {
    pub static RULES: RefCell<HashMap<i32, Rc<dyn BasicRule>>> = RefCell::new(HashMap::new());
}


fn main() {
    let first_rule = EmptyRule {};
    let second_rule = BeginRule::new(1);

    register_rule(Rc::new(first_rule));
    register_rule(Rc::new(second_rule));

    RULES.with( | rules| {
        let rule = rules.borrow_mut().get_mut(&1).unwrap();

        match rules.borrow_mut().get(&1) {
            None => {}
            Some(rule) => {
                println!("{:?}", rule.id());
            }
        }
    });
}

pub fn register_rule(rule: Rc<dyn BasicRule>) {
    RULES.with(|w| {
        w.borrow_mut().insert(rule.id(), rule);
    });
}

// fn refcell() {
// // https://github.com/servo/servo/blob/master/ports/winit/app.rs
//     let first_rule = EmptyRule {};
//     let second_rule = BeginRule::new(1);
//
//     let mut map: HashMap<i32, Rc<RefCell<dyn BasicRule>>> = HashMap::new();
//
//     map.insert(1, Rc::new(RefCell::new(second_rule)));
//     map.insert(2, Rc::new(RefCell::new(first_rule)));
//     {
//         let a = map.get(&1).unwrap().clone();
//         let x = &mut *a.borrow_mut();
//         x.collect_patterns_recursive(map);
//     }
// }
