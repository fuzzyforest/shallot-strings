#![feature(macro_metavar_expr)]
use anyhow::{ensure, Context, Result};

extern crate shallot;

use std::fmt::Display;

use shallot::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LispString(pub String);

impl<E: LispExpression> Atom<E> for LispString {
    fn sized_name() -> &'static str
    where
        Self: Sized,
    {
        "string"
    }

    fn name(&self) -> &'static str {
        "string"
    }

    fn parse_from_token(token: &Token) -> Option<Self>
    where
        Self: Sized,
    {
        Some(LispString(
            token.value.strip_prefix('"')?.strip_suffix('"')?.to_owned(),
        ))
    }
}

impl Display for LispString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[3;31m\"{}\"\x1b[0m", self.0)
    }
}

pub fn split<E>(arguments: &[E], _env: &mut Environment<E>) -> Result<E>
where
    E: LispExpression + ToAndFrom<LispString>,
{
    ensure!(arguments.len() == 1, "Split needs a single argument");

    let argument: &LispString = arguments[0]
        .try_into_atom()
        .context("Argument to split must be a string")?;
    let splits: Vec<E> = argument
        .0
        .split_whitespace()
        .map(|s| LispString(s.to_owned()).into())
        .collect();
    Ok(List(splits).into())
}

pub fn upper(argument: &LispString) -> Result<LispString> {
    Ok(LispString(argument.0.to_uppercase()).into())
}

create_layer!(
   atoms  LispString
   | builtins
     "split" -> BuiltinFunction::new("split", split),
     "upper" -> BuiltinFunction::new_wrapped("upper", upper)
);
