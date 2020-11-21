use std::borrow::BorrowMut;
use dyn_clone::{clone_trait_object, DynClone};
use std::collections::HashMap;

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
        let other_rule_id = 0;
        let mut rule = container.get_rule(other_rule_id).clone();
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
    pub rules: HashMap<i32, Box<dyn BasicRule>>,
}

impl RuleContainer {
    fn default() -> Self {
        RuleContainer {
            rules: Default::default(),
        }
    }

    pub fn get_rule(&mut self, pattern_id: i32) -> &mut Box<dyn BasicRule> {
        return self
            .rules
            .get_mut(&pattern_id)
            .unwrap();
    }

    pub fn register_rule(&mut self, result: Box<dyn BasicRule>) -> i32 {
        let id = result.id();
        self.rules.insert(id, result);
        id
    }
}

fn main() {
    let mut container = RuleContainer::default();
    let id = 1;
    container.register_rule(Box::new(EmptyRule { }));
    container.register_rule(Box::new(BeginRule::new(1)));

    let mut rule = container.get_rule(1).clone();
    rule.collect_patterns_recursive(&mut container);
}
