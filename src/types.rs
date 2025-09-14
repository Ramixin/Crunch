use std::fmt::{Display};
use crate::statements::Statement;

#[derive(Debug)]
pub(crate) struct Function {

    pub(crate) name: String,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) return_type: Option<Type>,
    pub(crate) statements: Vec<Statement>

}

#[derive(Debug)]
pub(crate) enum Type {
    Int,
    String,
    Float,
    Complex,
    IntList,
    FloatList,
    ComplexList,
    Matrix
}

#[derive(Debug)]
pub(crate) struct  Parameter {
    pub(crate) name: String,
    pub(crate) typetype: Type
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Char(char),
    Bool(bool),
    Int(i64),
    Float(f64),
    Null
}