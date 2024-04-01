use std::fmt;

pub enum NodeType {
    Program,
    NumericLiteral,
    Identifier,
    BinaryExpr,
}

pub trait Stmt: fmt::Debug {
    fn get_kind(&self) -> NodeType;
}

pub struct Program {
    pub body: Vec<Box<dyn Stmt>>,
}

impl Stmt for Program {
    fn get_kind(&self) -> NodeType {
        NodeType::Program
    }
}

pub trait Expr: Stmt {
    fn clone_box(&self) -> Box<dyn Expr>;
}

pub struct BinaryExpr {
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
    pub operator: String,
}

impl Stmt for BinaryExpr {
    fn get_kind(&self) -> NodeType {
        NodeType::BinaryExpr
    }
}

impl Expr for BinaryExpr {
    fn clone_box(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }
}

impl Clone for BinaryExpr {
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
            operator: self.operator.clone(),
        }
    }
}

pub struct Identifier {
    pub symbol: String,
}

impl Stmt for Identifier {
    fn get_kind(&self) -> NodeType {
        NodeType::Identifier
    }
}

impl Expr for Identifier {
    fn clone_box(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }
}

impl Clone for Identifier {
    fn clone(&self) -> Self {
        Self {
            symbol: self.symbol.clone(),
        }
    }
}

pub struct NumericLiteral {
    pub value: i32,
}

impl Stmt for NumericLiteral {
    fn get_kind(&self) -> NodeType {
        NodeType::NumericLiteral
    }
}

impl Expr for NumericLiteral {
    fn clone_box(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }
}

impl Clone for NumericLiteral {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
        }
    }
}

pub struct ExprStmt {
    pub expr: Box<dyn Expr>,
}

impl Stmt for ExprStmt {
    fn get_kind(&self) -> NodeType {
        self.expr.get_kind()
    }
}


impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n kind: Program,\n body: [\n")?;
        for (_i, stmt) in self.body.iter().enumerate() {
            write!(f, "    {:?},\n", stmt)?;
        }
        write!(f, " ] \n}}")
    }
}

impl fmt::Debug for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ kind: BinaryExpr, operator: {}, left: {:?}, right: {:?} }}", self.operator, self.left, self.right)
    }
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ kind: Identifier, symbol: {} }}", self.symbol)
    }
}

impl fmt::Debug for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ kind: NumericLiteral, value: {} }}", self.value)
    }
}

impl fmt::Debug for ExprStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.expr)
    }
}