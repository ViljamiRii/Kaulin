#[derive(Debug, Clone)]
pub enum NodeType {
    //Statements
    Program,
    VarDeclaration,

    //Exprerssions
    AssignmentExpr,

    //Literals
    Property,
    ObjectLiteral,
    NumericLiteral,
    Identifier,
    BinaryExpr,
}

#[derive(Debug)]
pub enum Stmt {
    Program(Program),
    VarDeclaration(VarDeclaration),

    Expr(Expr),
    AssignmentExpr(AssignmentExpr),

    Property(Property),
    ObjectLiteral(ObjectLiteral),
    NumericLiteral(NumericLiteral),
    Identifier(Identifier),
    BinaryExpr(BinaryExpr),
    
}

#[derive(Debug)]
pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct VarDeclaration {
    pub kind: NodeType,
    pub constant: bool,
    pub identifier: Identifier,
    pub value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    AssignmentExpr(AssignmentExpr), 

    Property(Property),
    ObjectLiteral(ObjectLiteral),
    NumericLiteral(NumericLiteral),
    Identifier(Identifier),
    BinaryExpr(BinaryExpr),

    Empty,
}

#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub kind: NodeType,
    pub assignee: Box<Expr>,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct NumericLiteral {
    pub kind: NodeType,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub kind: NodeType,
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub kind: NodeType,
    pub key: String,
    pub value: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct ObjectLiteral {
    pub kind: NodeType,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub kind: NodeType,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: String,
}