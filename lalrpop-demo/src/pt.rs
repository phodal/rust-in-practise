pub struct Func {
    pub expr: Expression
}

pub struct Expression {

}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct Loc(pub usize, pub usize);

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Identifier {
    pub loc: Loc,
    pub name: String,
}
