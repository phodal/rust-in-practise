use dyn_clone::{clone_trait_object, DynClone};
use std::collections::HashMap;

trait BasicRule: DynClone {
    fn id(&self) -> i32;
    fn collect_patterns_recursive(&mut self, container: &mut Vec<Box<dyn BasicRule>>);
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

    fn collect_patterns_recursive(&mut self, container: &mut Vec<Box<dyn BasicRule>>) {
        let other_rule_id = 0;
        let mut rule = container[other_rule_id].clone();
        rule.collect_patterns_recursive(container);
    }
}

#[derive(Clone)]
pub struct EmptyRule {}

impl BasicRule for EmptyRule {
    fn id(&self) -> i32 { 0 }
    fn collect_patterns_recursive(&mut self, _container: &mut Vec<Box<dyn BasicRule>>) {}
}

pub struct RuleContainer<'rules> {
    index: HashMap<i32, Box<dyn BasicRule>>,
    rules: &'rules mut Vec<Box<dyn BasicRule>>,
}

impl<'rules> RuleContainer<'rules> {
    fn default(rules: &'rules mut Vec<Box<dyn BasicRule>>) -> Self {
        RuleContainer {
            index: Default::default(),
            rules,
        }
    }

    #[allow(dead_code)]
    fn get_rule(&mut self, pattern_id: usize) -> &mut Box<dyn BasicRule> {
        return &mut self.rules[pattern_id];
    }

    fn register_rule(&mut self, result: Box<dyn BasicRule>) -> i32 {
        let id = result.id();
        self.rules.resize_with((id + 1) as usize, || { Box::from(EmptyRule {}) });
        self.index.insert(id, result.clone());
        self.rules[id as usize] = result;
        id
    }

    pub fn collect_by_id(&mut self, pattern_id: i32) {
        let rule = self.index.get_mut(&1).unwrap();
        rule.collect_patterns_recursive(&mut self.rules);
    }
}

fn main() {
    let mut rules = vec![];
    let mut container = RuleContainer::default(&mut rules);

    container.register_rule(Box::new(EmptyRule {}));
    container.register_rule(Box::new(BeginRule::new(1)));

    // todo: rebuild get rule logic ? to index ?????
    container.collect_by_id(1);
}
