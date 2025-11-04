struct Token {
    literal: String,
}


struct If {
    condition: Expression,
    expression: Expression,
    else_expression: Option<Expression>,
}

struct Function {
    identifier: Identifier,
    arguments: Vec<Expression>,
}

struct Identifier {
    identifier: String
}

enum AbstractSyntaxTree {
    Expression(Expression),
    If(If),

}

enum Expression {

}
