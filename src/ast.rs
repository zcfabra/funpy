use std::any::Any;
use std::fmt::format;

use crate::parser::ParseError;
use crate::token::Token;

pub trait Node: Any {
    fn repr(&self) -> String;
    fn eval(self) -> Box<dyn Node>;
}

pub trait Expression {}

/*
Expressions:
1. Binary
2. Unary
3. Literal
4. Walrus?
5. Pipe
*/

pub struct IntegerNode {
    value: i32,
}

impl IntegerNode {
    pub fn new(token: Token) -> Result<Self, ParseError> {
        if let Token::Int(i) = token.clone() {
            return Ok(IntegerNode {
                value: i
                    .parse::<i32>()
                    .map_err(|_| ParseError::InvalidTypeData(format!("")))?,
            });
        } else {
            return Err(ParseError::InvalidTypeData(format!(
                "Expected Integer - Found {:?}",
                token
            )));
        }
    }
}

impl Node for IntegerNode {
    fn eval(self) -> Box<dyn Node> {
        return Box::new(self);
    }
    fn repr(&self) -> String {
        return self.value.to_string();
    }
}

pub struct BinaryExpr {
    op: Token,
    l: Box<dyn Node>,
    r: Box<dyn Node>,
}

impl BinaryExpr {
    pub fn new(op_token: Token, l: Box<dyn Node>, r: Box<dyn Node>) -> Self {
        return BinaryExpr {
            op: op_token,
            l: l,
            r: r,
        };
    }
}

impl Node for BinaryExpr {
    fn eval(self) -> Box<dyn Node> {
        return Box::new(self);
    }
    fn repr(&self) -> String {
        let l = &self.l.repr();
        let r = &self.r.repr();
        return format!("({} {} {})", l, self.op, r);
    }
}

/*
Statements:
1. Assignment
2. Block
3. If
4. Return
5. Switch
6. Fn declaration
*/

#[derive(Debug)]
pub struct Identifier {
    literal: String,
}
impl Identifier {
    pub fn new(literal: String) -> Self {
        return Identifier { literal };
    }
}

impl Node for Identifier {
    fn eval(self) -> Box<dyn Node> {
        return Box::new(self);
    }
    fn repr(&self) -> String {
        return format!("{}", self.literal);
    }
}

pub struct CallStmt {
    name: Identifier,
    args: Vec<Box<dyn Node>>,
}
impl CallStmt {
    pub fn new(name: Identifier, args: Vec<Box<dyn Node>>) -> Self {
        return CallStmt { name, args };
    }
}
impl Node for CallStmt {
    fn eval(self) -> Box<dyn Node> {
        return Box::new(self);
    }
    fn repr(&self) -> String {
        let args = self
            .args
            .iter()
            .map(|e| e.repr())
            .collect::<Vec<String>>()
            .join(", ");
        return format!("{}({})", self.name.literal, args);
    }
}

pub struct ConditionalStmt {
    conditional: Box<dyn Node>,
    pass_block: Box<BlockStmt>,
    fail_block: Option<Box<BlockStmt>>,
}

impl ConditionalStmt {
    pub fn new(
        cond: Box<dyn Node>,
        pass_block: Box<BlockStmt>,
        fail_block: Option<Box<BlockStmt>>,
    ) -> Self {
        return ConditionalStmt {
            conditional: cond,
            pass_block,
            fail_block,
        };
    }
}

impl Node for ConditionalStmt {
    fn eval(self) -> Box<dyn Node> {
        return Box::new(self);
    }
    fn repr(&self) -> String {
        let mut else_ = "".to_string();
        if self.fail_block.is_some() {
            let spaces =
                "    ".repeat(self.fail_block.as_ref().unwrap().indent - 1);
            else_ = format!(
                "{spaces}else:\n{}",
                self.fail_block.as_ref().unwrap().repr()
            );
        }
        return format!(
            "if ({}):\n{}\n{else_}",
            self.conditional.repr(),
            self.pass_block.repr(),
        );
    }
}

pub struct ReturnStmt {
    expr: Box<dyn Node>,
}

impl ReturnStmt {
    pub fn new(expr: Box<dyn Node>) -> Self {
        return ReturnStmt { expr };
    }
}

impl Node for ReturnStmt {
    fn eval(self) -> Box<dyn Node> {
        return Box::new(self);
    }
    fn repr(&self) -> String {
        return format!("return {}", &self.expr.repr());
    }
}
pub struct AssignmentStmt {
    identifier: Identifier,
    expr: Box<dyn Node>,
}

impl AssignmentStmt {
    pub fn new(identifier: Identifier, expression: Box<dyn Node>) -> Self {
        return AssignmentStmt {
            identifier: identifier,
            expr: expression,
        };
    }
}

impl Node for AssignmentStmt {
    fn eval(self) -> Box<dyn Node> {
        return Box::new(self);
    }
    fn repr(&self) -> String {
        let expr_repr = &self.expr.repr();
        return format!("{} = {}", self.identifier.repr(), expr_repr);
    }
}

pub struct Statement {}

pub struct BlockStmt {
    indent: usize,
    statements: Vec<Box<dyn Node>>,
}
impl BlockStmt {
    pub fn new(indent: usize, statements: Vec<Box<dyn Node>>) -> BlockStmt {
        return BlockStmt { indent, statements };
    }
}

impl Node for BlockStmt {
    fn eval(self) -> Box<dyn Node> {
        return Box::new(self);
    }
    fn repr(&self) -> String {
        let spaces = "    ".repeat(self.indent);
        return self
            .statements
            .iter()
            .map(|e| format!("{}{}", spaces, e.repr()))
            .collect::<Vec<String>>()
            .join("\n");
    }
}

pub struct FnArg {
    name: Identifier,
    // default: Literal
}
pub struct FnLiteral {
    name: Identifier,
    args: Vec<Identifier>,
    definition: Box<BlockStmt>,
}

impl FnLiteral {
    pub fn new(
        name: Identifier,
        args: Vec<Identifier>,
        definition: Box<BlockStmt>,
    ) -> Self {
        return FnLiteral {
            name,
            args,
            definition,
        };
    }
}

impl Node for FnLiteral {
    fn eval(self) -> Box<dyn Node> {
        return Box::new(self);
    }
    fn repr(&self) -> String {
        let args: String = self
            .args
            .iter()
            .map(|e| e.repr())
            .collect::<Vec<String>>()
            .join(", ");

        return format!(
            "def {}({}):\n{}",
            self.name.literal,
            args,
            self.definition.repr()
        );
    }
}
