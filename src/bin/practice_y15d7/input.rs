use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub enum ParseInstructionError {
    SplitSrcDst,
    UnaryOp,
    NoSourcePattern,
    BinOp(String),
}

#[derive(Debug)]
pub struct Instruction {
    pub dst: String,
    pub src: Source,
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;
    fn from_str(inst: &str) -> Result<Self, Self::Err> {
        let line = inst;
        let (src, dst) = line
            .split_once("->")
            .ok_or(ParseInstructionError::SplitSrcDst)?;
        let src = src.trim();
        let dst = dst.trim().into();

        let src = Source::from_str(src)?;

        Ok(Instruction { src, dst })
    }
}

#[derive(Debug)]
pub enum Source {
    Value {
        value: Value,
    },
    UnaryOp {
        value: Value,
        op: UnaryOp,
    },
    BinOp {
        value1: Value,
        value2: Value,
        op: BinOp,
    },
}

impl FromStr for Source {
    type Err = ParseInstructionError;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        {
            let src: Vec<_> = src.split(' ').collect();
            match &src[..] {
                [value] => {
                    let value = value.parse()?;
                    Ok(Source::Value { value })
                }
                [op, value] => {
                    if *op == "NOT" {
                        Ok(Source::UnaryOp {
                            value: value.parse()?,
                            op: UnaryOp::Not,
                        })
                    } else {
                        Err(ParseInstructionError::UnaryOp)
                    }
                }
                [value1, op, value2] => {
                    let op: BinOp = op.parse()?;
                    Ok(Source::BinOp {
                        value1: value1.parse()?,
                        value2: value2.parse()?,
                        op,
                    })
                }
                _ => Err(ParseInstructionError::NoSourcePattern),
            }
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Imm(u16),
    Wire(String),
}

impl Value {
    fn eval(
        self: &Value,
        env: &HashMap<String, Source>,
        cache: &mut HashMap<String, u16>,
    ) -> u16 {
        match self {
            Value::Imm(imm) => *imm,
            Value::Wire(wire) => eval_rec(wire, env, cache),
        }
    }
}

pub fn eval(wire: &str, env: &HashMap<String, Source>) -> u16 {
    let mut cache = HashMap::new();
    eval_rec(wire, env, &mut cache)
}

fn eval_rec(
    wire: &str,
    env: &HashMap<String, Source>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    if let Some(result) = cache.get(wire) {
        return *result;
    }

    let input = env.get(wire).unwrap();

    let result = match input {
        Source::Value { value } => value.eval(env, cache),
        Source::UnaryOp { value, op } => op.op(value.eval(env, cache)),
        Source::BinOp { value1, value2, op } => {
            let value1 = value1.eval(env, cache);
            let value2 = value2.eval(env, cache);
            op.op(value1, value2)
        }
    };

    cache.insert(wire.to_string(), result);
    result
}

impl FromStr for Value {
    type Err = ParseInstructionError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(imm) = value.parse() {
            Ok(Self::Imm(imm))
        } else {
            Ok(Self::Wire(value.to_string()))
        }
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Not,
}

impl UnaryOp {
    pub fn op(&self, value: u16) -> u16 {
        match self {
            UnaryOp::Not => !value,
        }
    }
}

impl FromStr for UnaryOp {
    type Err = ParseInstructionError;
    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "NOT" => Ok(Self::Not),
            _ => Err(ParseInstructionError::UnaryOp),
        }
    }
}
#[derive(Debug)]
pub enum BinOp {
    And,
    Or,
    LShift,
    RShift,
}

impl FromStr for BinOp {
    type Err = ParseInstructionError;

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        Ok(match op {
            "AND" => BinOp::And,
            "OR" => Self::Or,
            "LSHIFT" => Self::LShift,
            "RSHIFT" => Self::RShift,
            _ => return Err(ParseInstructionError::BinOp(op.into())),
        })
    }
}

impl BinOp {
    pub fn op(&self, value1: u16, value2: u16) -> u16 {
        match self {
            BinOp::And => value1 & value2,
            BinOp::Or => value1 | value2,
            BinOp::LShift => value1 << value2,
            BinOp::RShift => value1 >> value2,
        }
    }
}
