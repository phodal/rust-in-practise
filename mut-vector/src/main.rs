use std::borrow::BorrowMut;
use dyn_clone::{clone_trait_object, DynClone};
use std::collections::HashMap;

const RULE_SIZE: usize = 100;

trait BasicRule: DynClone {
    fn id(&self) -> i32;
    fn collect_patterns_recursive(&mut self, container: &mut RuleContainer);
}

clone_trait_object!(BasicRule);

#[derive(Clone)]
pub struct Rule { id: i32 }

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
        let other_rule_id = 0;
        let mut rule = container.get_rule(other_rule_id).clone();
        rule.collect_patterns_recursive(container);
    }
}

#[derive(Clone)]
pub struct EmptyRule {}

impl BasicRule for EmptyRule {
    fn id(&self) -> i32 { 0 }
    fn collect_patterns_recursive(&mut self, container: &mut RuleContainer) {}
}

pub struct RuleContainer<'rules> {
    pub rules: &'rules mut Vec<Box<dyn BasicRule>>,
}

impl<'rules> RuleContainer<'rules> {
    fn default(rules: &'rules mut Vec<Box<dyn BasicRule>>) -> Self {
        RuleContainer {
            rules
        }
    }

    pub fn get_rule(&mut self, pattern_id: usize) -> &mut Box<dyn BasicRule> {
        return &mut self
            .rules[pattern_id];
    }

    pub fn register_rule(&mut self, result: Box<dyn BasicRule>) -> i32 {
        let id = result.id();
        self.rules.resize_with((id + 1) as usize, || { Box::from(EmptyRule {}) });
        self.rules[id as usize] = result;
        id
    }
}

fn borrow_a<'a>(container: &'a mut RuleContainer, pattern_id: i32) -> &'a mut Box<dyn BasicRule> {
    &mut container.rules[pattern_id as usize]
}

fn main() {
    let mut rules = vec![];
    let mut container = RuleContainer::default(&mut rules);

    container.register_rule(Box::new(EmptyRule {}));
    container.register_rule(Box::new(BeginRule::new(1)));

    // let rule = borrow_a(&mut container, 1);
    let mut rule = container.get_rule(1).clone();
    rule.collect_patterns_recursive(&mut container);
}
