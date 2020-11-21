use std::borrow::BorrowMut;
use dyn_clone::{clone_trait_object, DynClone};

const RULE_SIZE: usize = 100;

trait BasicRule: DynClone {
    fn id(&self) -> i32;
    fn collect_patterns_recursive(&mut self, container: &mut RuleContainer,);
}

clone_trait_object!(BasicRule);

#[derive(Clone)]
pub struct Rule { id: i32, }

#[derive(Clone)]
pub struct BeginRule { rule: Rule }

impl BeginRule {
    pub fn new(id: i32) -> Self {
        BeginRule { rule: Rule { id } }
    }
}

impl BasicRule for BeginRule {
    fn id(&self) -> i32 { self.rule.id }

    fn collect_patterns_recursive(&mut self, container: &mut RuleContainer) {
        let mut rule = container.get_rule(0).clone();
        rule.collect_patterns_recursive(container);
    }
}

#[derive(Clone)]
pub struct EmptyRule { }

impl BasicRule for EmptyRule {
    fn id(&self) -> i32 { 0 }
    fn collect_patterns_recursive(&mut self, container: &mut RuleContainer) {

    }
}

pub struct RuleContainer {
    pub rules: Vec<Box<dyn BasicRule>>
}

impl RuleContainer {
    pub fn register_rule(&mut self, result: Box<dyn BasicRule>) -> i32 {
        let id = result.id();
        if id >= RULE_SIZE as i32 {
            self.rules
                .resize_with(id as usize + 1, || Box::new(EmptyRule {}));
        }
        self.rules[id as usize] = result;
        id
    }

    pub fn get_rule(&mut self, pattern_id: i32) -> &mut Box<dyn BasicRule> {
        return self.rules[pattern_id as usize].borrow_mut();
    }

    pub fn new() -> Self {
        let mut rules: Vec<Box<dyn BasicRule>> = vec![];
        rules.resize_with(RULE_SIZE, || { Box::new(EmptyRule {}) });
        RuleContainer {
            rules
        }
    }
}

fn main() {
    let mut container = RuleContainer::new();
    let id = 1;
    container.register_rule(Box::new(BeginRule::new(id)));

    let mut rule = container.get_rule(1).clone();
    rule.collect_patterns_recursive(&mut container);
}
