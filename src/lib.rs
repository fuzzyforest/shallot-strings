use anyhow::{bail, Context, Result};

extern crate shallot;

use std::fmt::Display;

use shallot::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LispString(String);

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
        if token.value.as_bytes()[0] == b'"'
            && token.value.as_bytes()[token.value.len() - 1] == b'"'
        {
            String::from_utf8(token.value.as_bytes()[1..token.value.len() - 1].to_vec())
                .ok()
                .map(LispString)
        } else {
            None
        }
    }
}

impl Display for LispString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[3;31m\"{}\"\x1b[0m", self.0)
    }
}

create_expression!(
    StringyExpression,
    Symbol,
    Number,
    List<StringyExpression>,
    BuiltinFunction<StringyExpression>,
    BuiltinMacro<StringyExpression>,
    Lambda<StringyExpression>,
    Macro<StringyExpression>,
    LispString
);

impl LispExpression for StringyExpression {
    fn as_atom(&self) -> &dyn Atom<Self> {
        match self {
            StringyExpression::Symbol(a) => a,
            StringyExpression::Number(a) => a,
            StringyExpression::List(a) => a,
            StringyExpression::BuiltinFunction(a) => a,
            StringyExpression::BuiltinMacro(a) => a,
            StringyExpression::Lambda(a) => a,
            StringyExpression::Macro(a) => a,
            StringyExpression::LispString(a) => a,
        }
    }

    fn parse_from_token(token: &Token) -> Self {
        None.or_else(|| <Number as Atom<Self>>::parse_from_token(token).map(Self::from))
            .or_else(|| <LispString as Atom<Self>>::parse_from_token(token).map(Self::from))
            .or_else(|| <Symbol as Atom<Self>>::parse_from_token(token).map(Self::from))
            // This will never fail as symbols never fail parsing
            .unwrap()
    }
}

impl Display for StringyExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_atom().fmt(f)
    }
}

impl_try_to_from!(
    StringyExpression,
    Symbol,
    Number,
    Lambda<StringyExpression>,
    Macro<StringyExpression>,
    BuiltinFunction<StringyExpression>,
    BuiltinMacro<StringyExpression>,
    List<StringyExpression>,
    LispString
);

pub fn split<E>(arguments: &[E], _env: &mut Environment<E>) -> Result<E>
where
    E: LispExpression + ToAndFrom<LispString>,
{
    if arguments.len() != 1 {
        bail!("Split needs a single argument");
    }
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

pub fn upper<E>(arguments: &[E], _env: &mut Environment<E>) -> Result<E>
where
    E: LispExpression + ToAndFrom<LispString>,
{
    if arguments.len() != 1 {
        bail!("Upper needs a single argument");
    }
    let argument: &LispString = arguments[0]
        .try_into_atom()
        .context("Argument to upper must be a string")?;
    Ok(LispString(argument.0.to_uppercase()).into())
}

pub fn set_environment<E: LispExpression + ToAndFrom<LispString>>(env: &mut Environment<E>) {
    env.set(
        Symbol("split".to_owned()),
        BuiltinFunction::new("split", split),
    );
    env.set(
        Symbol("upper".to_owned()),
        BuiltinFunction::new("upper", upper),
    );
}
