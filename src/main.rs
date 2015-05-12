extern crate torpedo;

use torpedo::{ Operation, Type, Context };

fn main() {
    let context = Context::new();

    let t1 = Type::new(0);

    let result = context
        .check(&Bind { id: 1, typ: t1 }).unwrap()
        .check(&Bind { id: 2, typ: t1 }).unwrap()
        .check(&Assign { a: 1, b: 2 });

    assert!(result.is_ok());
}

//Operation "binding" a field as a certain type
struct Bind {
    id: u64,
    typ: Type
}

impl Operation for Bind {
    fn try(&self, context: &mut Context) -> Result<(), &'static str> {
        let t = context.find_type(&self.id);
        match t {
            Some(_) => Err("Variable is already bound"),
            None => {
                context.set_type(self.id, self.typ);
                Ok(())
            }
        }
    }
}

//Operation "assigning" the value of B to A (fails if the types are not the same)
struct Assign {
    a: u64,
    b: u64
}

impl Operation for Assign {
    fn try(&self, context: &mut Context) -> Result<(), &'static str> {
        let a = context.find_type(&self.a);
        let b = context.find_type(&self.b);
        match (a, b) {
            (Some(a), Some(b)) if a == b => Ok(()),
            (Some(_), Some(_)) => Err("Assigning incompatible types"),
            _ => Err("Variable not defined")
        }
    }
}

#[test]
fn assign_compatible_type_from_one_var_to_another() {
    let context = Context::new();

    let t1 = Type::new(0);

    let result = context
        .check(&Bind { id: 1, typ: t1 }).unwrap()
        .check(&Bind { id: 2, typ: t1 }).unwrap()
        .check(&Assign { a: 1, b: 2 });

    assert!(result.is_ok());
}

#[test]
fn assign_incompatible_type_from_one_var_to_another() {
    let context = Context::new();

    let t1 = Type::new(0);
    let t2 = Type::new(1);

    let result = context
        .check(&Bind { id: 1, typ: t1 }).unwrap()
        .check(&Bind { id: 2, typ: t2 }).unwrap()
        .check(&Assign { a: 0, b: 1 });

    assert!(result.is_err());
}
