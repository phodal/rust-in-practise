use dyn_clone::{clone_trait_object, DynClone};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub trait BasicRule: DynClone {
    fn id(&self) -> i32;
    fn collect_patterns_recursive(&mut self, container: &RefCell<HashMap<i32, Rc<dyn BasicRule>>>);
    fn update_by(&mut self) where Self: Sized;
}

clone_trait_object!(BasicRule);

#[derive(Debug, Clone)]
pub struct Rule { id: i32 }

#[derive(Debug, Clone)]
pub struct BeginRule { rule: Rule }

impl BeginRule {
    pub fn new(id: i32) -> Self {
        BeginRule { rule: Rule { id } }
    }
}

impl BasicRule for BeginRule {
    fn id(&self) -> i32 { self.rule.id }

    fn collect_patterns_recursive(&mut self, _map: &RefCell<HashMap<i32, Rc<dyn BasicRule>>>) {
        // let rule_ref = map.get(&1).unwrap().clone();
        // let x = &mut *rule_ref.borrow();
        // x.collect_patterns_recursive(map);
    }

    fn update_by(&mut self) where Self: Sized {}
}

#[derive(Debug, Clone)]
pub struct EmptyRule {}

impl BasicRule for EmptyRule {
    fn id(&self) -> i32 { 0 }
    fn collect_patterns_recursive(&mut self, _container: &RefCell<HashMap<i32, Rc<dyn BasicRule>>>) {}
    fn update_by(&mut self) where Self: Sized {}
}
