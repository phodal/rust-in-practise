pub mod rule;

use std::{cell::RefCell, collections::HashMap};
use crate::rule::{EmptyRule, BeginRule, BasicRule};
use std::rc::Rc;

fn main() {
    let mut rules: Vec<Box<dyn BasicRule>> = vec![];

    let first_rule = EmptyRule {};
    let second_rule = BeginRule::new(1);

    let mut hm: HashMap<i32, Box<dyn BasicRule>> = HashMap::new();
    hm.insert(1, Box::new(first_rule.clone()));
    hm.insert(2, Box::new(second_rule.clone()));

    let map2: Rc<RefCell<HashMap<i32, Box<dyn BasicRule>>>> = Rc::new(RefCell::new(hm));
    let mut ref_mut = map2.borrow_mut();
    let mut rule = ref_mut.get_mut(&1).unwrap();
    rule.collect_patterns_recursive(&mut rules);

    // let rule = ref_mut.get_mut(&1).unwrap();
    // rule.id();

    // let mut map: HashMap<i32, Rc<RefCell<dyn BasicRule>>> = HashMap::new();
    //
    // map.insert(1, Rc::new(RefCell::new(first_rule)));
    // map.insert(2, Rc::new(RefCell::new(second_rule)));
    // {
    //     let a = map.get(&1).unwrap();
    //     let x = &*a.borrow();
    //     println!("a: {}", x.id());
    //
    //
    //     let b = map.get(&2).unwrap();
    //     let y = &*b.borrow_mut();
    //     println!("b: {}", y.id());
    // }

    // println!("Results: {:?}", map);
}
