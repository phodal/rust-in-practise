use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread::LocalKey;

use crate::rule::{BasicRule, BeginRule};

mod rule;

thread_local! {
    pub static RULES: RefCell<HashMap<i32, Rc<dyn BasicRule>>> = RefCell::new(HashMap::new());
}

#[derive(Debug, Clone)]
pub struct RuleContainer {
    pub refs: &'static LocalKey<RefCell<HashMap<i32, Rc<dyn BasicRule>>>>,
}

impl Default for RuleContainer {
    fn default() -> Self {
        RuleContainer {
            refs: &RULES
        }
    }
}

impl RuleContainer {
    pub fn get_rules<'a>() -> HashMap<i32, Rc<dyn BasicRule>> {
        RULES.with(|w| {
            w.borrow_mut().clone()
        })
    }

    pub fn register_rule(rule: Rc<dyn BasicRule>) {
        RULES.with(|w| {
            w.borrow_mut().insert(rule.id(), rule);
        });
    }
}

fn main() {
    let begin_rule = BeginRule::new(1);
    let empty_rule = BeginRule::new(2);

    RuleContainer::register_rule(Rc::new(begin_rule));
    RuleContainer::register_rule(Rc::new(empty_rule));

    let mut map = RuleContainer::get_rules();
    let rule = map.get_mut(&1).unwrap();
    println!("{:?}", rule.id());
}
