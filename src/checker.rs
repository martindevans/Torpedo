use std::collections::HashMap;

pub trait Operation {
    fn try(&self, context: &mut Context) -> Result<(), &'static str>;
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Type {
    id: u64
}

impl Type {
    pub fn new(id: u64) -> Type {
        return Type {
            id: id
        }
    }
}

pub struct Context {
    parent: Option<Box<Context>>,

    bindings: HashMap<u64, Type>
}

impl Context {
    pub fn sub(self) -> Context {
        return Context {
            parent: Some(Box::new(self)),

            bindings: HashMap::new()
        }
    }

    pub fn new() -> Context {
        return Context {
            parent: None,

            bindings: HashMap::new()
        }
    }

    // Hunt up the linked list of contexts looking for a type binding for this variable
    pub fn find_type(&self, id: &u64) -> Option<Type> {
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

    pub fn set_type(&mut self, id: u64, typ: Type) {
        self.bindings.insert(id, typ);
    }

    // Check if this operation is valid in this context, if it is mutate the context to include this change
    pub fn check<O : Sized + Operation>(mut self, op: &O) -> Result<Context, &'static str> {
        let e = op.try(&mut self);
        match e {
            Err(v) => Err(v),
            Ok(_) => Ok(self)
        }
    }
}
