#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Func {
    pub body: Block
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Statement {
    pub expr: Expression
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Block {
    pub stmts: Vec<Statement>
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Expression {}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Loc(pub usize, pub usize);

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Identifier {
    pub loc: Loc,
    pub name: String,
}
