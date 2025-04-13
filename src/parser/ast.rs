pub type Program = Vec<Stmt>;

/// Statement
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    LetStmt(Ident, Expr),
    ReturnStmt(Expr),
    ExprStmt(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IdentExpr(Ident),
    LiteralExpr(Literal),
    PrefixExpr(Prefix, Box<Expr>),
    InfixExpr(Infix, Box<Expr>, Box<Expr>),
    IfExpr {
        cond: Box<Expr>,
        consequence: Program,
        alternative: Option<Program>,
    },
    FnExpr {
        params: Vec<Ident>,
        body: Program,
    },
    CallExpr {
        function: Box<Expr>,
        arguments: Vec<Expr>,
    },
    ArrayExpr(Vec<Expr>),
    HashExpr(Vec<(Literal, Expr)>),
    IndexExpr {
        array: Box<Expr>,
        index: Box<Expr>,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    IntLiteral(i64),
    BoolLiteral(bool),
    StringLiteral(String),
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Prefix {
    PrefixPlus,
    PrefixMinus,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    PLowest,
    PEquals,
    PLessGreater,
    PSum,
    PProduct,
    PCall,
    PIndex,
}
