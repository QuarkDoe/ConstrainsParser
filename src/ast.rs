
#[derive(Debug)]
pub enum Literal {
    BooleanLiteral,
    DecimalLiteral,
    HexLiteral,
    OctalLiteral,
    FloatLiteral,
    StringLiteral,
    RegexpLiteral,
}

#[derive(Debug)]
pub enum LogicOp {
    AND,
    OR,
}

#[derive(Debug)]
pub enum CompOp {
    EQ, // ==
    NE, // !=
    GE, // >=
    GT, // >
    LE, // =<
    LT, // <
    RX, // =~ <regexpr>
    //NR, // !~ <regexpr>
}

#[derive(Debug)]
pub enum ArithOp {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
}

#[derive(Debug)]
pub enum UnaryOp {
    NEG,
    NOT,
}


#[derive(Debug)]
pub enum Expression {
    None,
    LogicExpr( Box<Expression>, LogicOp, Box<Expression> ),
    IsNullExpr( Box<Expression>, bool ),
    CompExpr( Box<Expression>, CompOp, Box<Expression> ),
    // LikeExpr( Box<Expression>, Literals, Option<Literals>, bool /*'NOT LIKE' flag*/ ),
    BetweenExpr( Box<Expression>, Box<Expression>, Box<Expression>, bool /*'NOT BETWEEN' flag*/ ),
    InExpr( Box<Expression>, Vec<Literal>, bool /*'NOT IN' flag*/ ),
    ArithExpr( Box<Expression>, ArithOp ,Box<Expression> ),
    UnaryExpr( UnaryOp, Box<Expression> ),
    ConstExpr( Box<Literal> ),
}
