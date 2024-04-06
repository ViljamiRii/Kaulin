#[derive(Debug, Clone)]
pub enum Stmt {
    Program(Program),
    VarDeclaration(VarDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct VarDeclaration {
    pub constant: bool,
    pub identifier: Identifier,
    pub value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub parameters: Vec<String>,
    pub name: String,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    AssignmentExpr(AssignmentExpr),
    MemberExpr(MemberExpr),
    CallExpr(CallExpr),
    IfExpr(IfExpr),
    Property(Property),
    ObjectLiteral(ObjectLiteral),
    ArrayLiteral(ArrayLiteral),
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
    FloatLiteral(FloatLiteral),
    Identifier(Identifier),
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    LogicalExpr(LogicalExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulus,

    // TODO: Refactor these to be comparison operators
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,

    //TODO: Refactor these to be logical operators
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: BinaryOperator,
}

#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub assignee: Box<Expr>,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub condition: Box<Expr>,
    pub then_branch: Vec<Stmt>,
    pub else_branch: Option<Vec<Stmt>>,
}

#[derive(Debug, Clone)]
pub struct MemberExpr {
    pub object: Box<Expr>,
    pub property: Box<Expr>,
    pub computed: bool,
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub args: Vec<Expr>,
    pub caller: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct NumericLiteral {
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct FloatLiteral {
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub key: String,
    pub value: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct ObjectLiteral {
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub elements: Vec<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: BinaryOperator,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: String,
    pub operand: Box<Expr>,
}