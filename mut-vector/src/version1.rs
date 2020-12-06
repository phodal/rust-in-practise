use dyn_clone::{clone_trait_object, DynClone};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

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

fn old_test() {
    let mut rules = vec![];
    let mut container = RuleContainer::default(&mut rules);

    container.register_rule(Box::new(EmptyRule {}));
    container.register_rule(Box::new(BeginRule::new(1)));

    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(container.index));

    let mut rules = shared_map.borrow_mut();
    let rule = rules.get_mut(&1).unwrap();
    rule.collect_patterns_recursive(&mut container.rules);
}
