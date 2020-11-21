use std::borrow::BorrowMut;

const RULE_SIZE: usize = 100;

trait BasicRule { fn id(&self) -> i32; }

pub struct Rule { id: i32, }

pub struct BeginRule { rule: Rule }

impl BeginRule {
    pub fn new(id: i32) -> Self {
        BeginRule { rule: Rule { id } }
    }
}

impl BasicRule for BeginRule {
    fn id(&self) -> i32 { self.rule.id }
}

pub struct EmptyRule { }

impl BasicRule for EmptyRule { fn id(&self) -> i32 { 0 } }

pub struct RuleContainer {
    pub rules: Vec<Box<dyn BasicRule>>
}

impl RuleContainer {
    pub fn register_rule_new(&mut self, result: Box<dyn BasicRule>) -> i32 {
        let id = result.id();
        if id >= RULE_SIZE as i32 {
            self.rules
                .resize_with(id as usize + 1, || Box::new(EmptyRule {}));
        }
        self.rules[id as usize] = result;
        id
    }

    pub fn get_rule_new(&mut self, pattern_id: i32) -> &mut Box<dyn BasicRule> {
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
    println!("Hello, world!");
}
