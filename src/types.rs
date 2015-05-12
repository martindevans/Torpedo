//http://www.cse.chalmers.se/edu/course/DAT150/lectures/proglang-07.html

use std::collections::HashMap;

enum Operation {
    Bind(u64, Type),
    Assign(u64, u64)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Type {
    id: u64
}

impl Type {
    fn new(id: u64) -> Type {
        return Type {
            id: id
        }
    }
}

struct Context {
    parent: Option<Box<Context>>,

    bindings: HashMap<u64, Type>
}

impl Context {
    fn sub(self) -> Context {
        return Context {
            parent: Some(Box::new(self)),

            bindings: HashMap::new()
        }
    }

    fn new() -> Context {
        return Context {
            parent: None,

            bindings: HashMap::new()
        }
    }

    // Hunt up the linked list of contexts looking for a type binding for this variable
    fn find_type(&self, id: &u64) -> Option<Type> {
        match self.bindings.get(id) {
            Some(t) => Some(*t),
            None => {
                match self.parent {
                    Some(ref p) => p.find_type(id),
                    None => None
                }
            }
        }
    }

    // Check if this operation is valid in this context, if it is mutate the context to include this change
    fn check(mut self, op: Operation) -> Result<Context, &'static str> {
        match op {
            Operation::Bind(id, t) => {
                match self.bindings.get(&id) {
                    Some(_) => Err("Variable is already bound"),
                    None => {
                        self.bindings.insert(id, t);
                        Ok(self)
                    }
                }
            },
            Operation::Assign(a, b) => {
                let r = match (self.find_type(&a), self.find_type(&b)) {
                    (Some(a), Some(b)) if a == b => Ok(()),
                    (Some(a), Some(b)) => Err("Assigning incompatible types"),
                    _ => Err("Variable not defined")
                };

                match r {
                    Err(v) => Err(v),
                    Ok(_) => Ok(self)
                }
            }
        }
    }
}

#[test]
fn assign_compatible_type_from_one_var_to_another() {
    let context = Context::new();

    let t1 = Type::new(0);

    let result = context.check(Operation::Bind(0, t1)).unwrap()
        .check(Operation::Bind(1, t1)).unwrap()
        .check(Operation::Assign(0, 1));

    assert!(result.is_ok());
}

#[test]
fn assign_incompatible_type_from_one_var_to_another() {
    let context = Context::new();

    let t1 = Type::new(0);
    let t2 = Type::new(1);

    let result = context.check(Operation::Bind(0, t1)).unwrap()
        .check(Operation::Bind(1, t2)).unwrap()
        .check(Operation::Assign(0, 1));

    assert!(result.is_err());
}
