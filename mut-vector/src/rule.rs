use dyn_clone::{clone_trait_object, DynClone};

pub trait BasicRule: DynClone {
    fn id(&self) -> i32;
    fn collect_patterns_recursive(&mut self, container: &mut Vec<Box<dyn BasicRule>>);
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

    fn collect_patterns_recursive(&mut self, container: &mut Vec<Box<dyn BasicRule>>) {
        let other_rule_id = 0;
        let mut rule = container[other_rule_id].clone();
        rule.collect_patterns_recursive(container);
    }

    fn update_by(&mut self) where Self: Sized {}
}

#[derive(Debug, Clone)]
pub struct EmptyRule {}

impl BasicRule for EmptyRule {
    fn id(&self) -> i32 { 0 }
    fn collect_patterns_recursive(&mut self, _container: &mut Vec<Box<dyn BasicRule>>) {}
    fn update_by(&mut self) where Self: Sized {}
}
