use crate::ast::node::Node;
use crate::ast::types::{Type, TypeNode};

pub mod node;
pub mod types;

pub type Identifier = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int32(i32),
    Int64(i64),
    UInt32(u32),
    UInt64(u64),
    Boolean(bool),
    String(String),
    Array(Vec<Expression>),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Add(Box<ExpressionNode>, Box<ExpressionNode>),
    Sub(Box<ExpressionNode>, Box<ExpressionNode>),
    Mul(Box<ExpressionNode>, Box<ExpressionNode>),
    Div(Box<ExpressionNode>, Box<ExpressionNode>),
    Pow(Box<ExpressionNode>, Box<ExpressionNode>),
    Mod(Box<ExpressionNode>, Box<ExpressionNode>),
    Eq(Box<ExpressionNode>, Box<ExpressionNode>),
    Neq(Box<ExpressionNode>, Box<ExpressionNode>),
    Lt(Box<ExpressionNode>, Box<ExpressionNode>),
    Le(Box<ExpressionNode>, Box<ExpressionNode>),
    Ge(Box<ExpressionNode>, Box<ExpressionNode>),
    Gt(Box<ExpressionNode>, Box<ExpressionNode>),
    And(Box<ExpressionNode>, Box<ExpressionNode>),
    Or(Box<ExpressionNode>, Box<ExpressionNode>),
    Not(Box<ExpressionNode>),
    Ternary(
        Box<ExpressionNode>,
        Box<ExpressionNode>,
        Box<ExpressionNode>,
    ),
    FunctionCall(Identifier, Vec<ExpressionNode>),
    FunctionDef(FunctionNode),
}

pub type ExpressionNode = Node<Expression>;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub id: Identifier,
    pub ty: Option<Type>,
}

impl Variable {
    pub fn new(id: Identifier, ty: Option<Type>) -> Self {
        Self { id, ty }
    }
}

pub type VariableNode = Node<Variable>;

#[derive(Debug, Clone, PartialEq)]
pub enum Assignee {
    Identifier(Identifier),
}

pub type AssigneeNode = Node<Assignee>;

#[derive(Debug, Clone, PartialEq)]
pub enum DefinitionType {
    Let,
    Auto,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Declaration(VariableNode),
    Definition(DefinitionType, VariableNode, ExpressionNode),
    TypeDefinition(Identifier, TypeNode),
    Assignment(AssigneeNode, ExpressionNode),
    Condition(
        ExpressionNode,
        BlockStatementNode,
        Option<BlockStatementNode>,
    ),
    FunctionCall(Identifier, Vec<ExpressionNode>),
    Return(ExpressionNode),
}

pub type StatementNode = Node<Statement>;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement(pub Vec<StatementNode>);

pub type BlockStatementNode = Node<BlockStatement>;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub inputs: Vec<Type>,
    pub output: Option<Type>,
}

pub type ParameterNode = VariableNode;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub id: Identifier,
    pub parameters: Vec<ParameterNode>,
    pub statements: Vec<StatementNode>,
    pub return_type: Option<Type>,
    pub signature: FunctionSignature,
}

impl Function {
    pub fn new(
        id: Identifier,
        parameters: Vec<ParameterNode>,
        statements: Vec<StatementNode>,
        return_type: Option<Type>,
    ) -> Self {
        let signature = FunctionSignature {
            inputs: parameters
                .clone()
                .iter()
                .map(|p| p.value.ty.as_ref().cloned().unwrap())
                .collect(),
            output: return_type.clone().map(|r| r),
        };
        Self {
            id,
            parameters,
            statements,
            return_type,
            signature,
        }
    }
}

pub type FunctionNode = Node<Function>;

#[derive(Debug, Clone)]
pub struct Program(pub Vec<StatementNode>);
