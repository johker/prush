use std::fmt;

use crate::push::stack::PushStack;

// Atoms
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Atom<'a> {
    List { atoms: PushStack<Atom<'a>> },
    Closer,
    InstructionMeta { name: &'a str, code_blocks: u32 },
    Literal { push_type: PushType },
    Input,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PushType {
    PushBoolType { val: bool },
    PushIntType { val: i32 },
    PushFloatType { val: f32 },
}

#[allow(dead_code)]
impl<'a> Atom<'a> {
    pub fn int(arg: i32) -> Atom<'a> {
        Atom::Literal {
            push_type: PushType::PushIntType { val: arg },
        }
    }
    pub fn float(arg: f32) -> Atom<'a> {
        Atom::Literal {
            push_type: PushType::PushFloatType { val: arg },
        }
    }
    pub fn bool(arg: bool) -> Atom<'a> {
        Atom::Literal {
            push_type: PushType::PushBoolType { val: arg },
        }
    }
    pub fn noop() -> Atom<'a> {
        Atom::InstructionMeta {
            name: "NOOP",
            code_blocks: 0,
        }
    }
    pub fn empty_list() -> Atom<'a> {
        Atom::List {
            atoms: PushStack::new(),
        }
    }
    pub fn list(arg: Vec<Atom<'a>>) -> Atom<'a> {
        Atom::List {
            atoms: PushStack::from_vec(arg),
        }
    }
}

impl<'a> PartialEq for Atom<'a> {
    fn eq(&self, other: &Self) -> bool {
        match &*self {
            Atom::List { atoms: _ } => match &*other {
                Atom::List { atoms: _ } => return true,
                _ => return false,
            },
            Atom::Closer => match &*other {
                Atom::Closer => return true,
                _ => return false,
            },
            Atom::InstructionMeta {
                name: _,
                code_blocks: _,
            } => match &*other {
                Atom::InstructionMeta {
                    name: _,
                    code_blocks: _,
                } => return true,
                _ => return false,
            },
            Atom::Literal { push_type: _ } => match &*other {
                Atom::Literal { push_type: _ } => return true,
                _ => return false,
            },
            Atom::Input => match &*other {
                Atom::Input => return true,
                _ => return false,
            },
        }
    }
}

impl<'a> fmt::Display for Atom<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Atom::List { atoms } => write!(f, "List: {}", atoms.to_string()),
            Atom::Closer => write!(f, "Closer"),
            Atom::InstructionMeta {
                name,
                code_blocks: _,
            } => {
                let at = "InstructionMeta".to_string();
                write!(f, "{}({})", at, name)
            }
            Atom::Literal { push_type } => {
                let at = "Literal".to_string();
                let info;
                match push_type {
                    PushType::PushBoolType { val } => info = val.to_string(),
                    PushType::PushIntType { val } => info = val.to_string(),
                    PushType::PushFloatType { val } => info = val.to_string(),
                }
                write!(f, "{}({})", at, info)
            }
            Atom::Input => write!(f, "Input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shallow_equality_when_comparing_atoms() {
        let literal_a = Atom::int(0);
        let literal_b = Atom::int(2);
        let closer_a = Atom::Closer;
        let closer_b = Atom::Closer;
        let list_a = Atom::list(vec![Atom::Closer]);
        let list_b = Atom::list(vec![Atom::int(0)]);
        let inst_a = Atom::noop();
        let inst_b = Atom::InstructionMeta {
            name: "BOOLEAN.AND",
            code_blocks: 0,
        };
        assert_eq!(list_a, list_b);
        assert_eq!(inst_a, inst_b);
        assert_eq!(literal_a, literal_b);
        assert_eq!(closer_a, closer_b);
        assert_ne!(list_a, literal_b);
        assert_ne!(closer_a, literal_b);
    }

    #[test]
    fn print_code_block() {
        let code_block = Atom::List {
            atoms: PushStack::from_vec(vec![Atom::int(0)]),
        };
        assert_eq!(code_block.to_string(), "List: 1:Literal(0);");
    }
}
