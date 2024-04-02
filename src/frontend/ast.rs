#[derive(Debug)]
pub enum NodeType {
    Program,
    NumericLiteral,
    Identifier,
    BinaryExpr,
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    NumericLiteral(NumericLiteral),
    Identifier(Identifier),
    BinaryExpr(BinaryExpr),
    Program(Program),
}

#[derive(Debug)]
pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    Identifier(Identifier),
    NumericLiteral(NumericLiteral),
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub kind: NodeType,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: String,
}

#[derive(Debug)]
pub struct Identifier {
    pub kind: NodeType,
    pub symbol: String,
}

#[derive(Debug)]
pub struct NumericLiteral {
    pub kind: NodeType,
    pub value: f64,
}