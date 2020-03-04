#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate im;

mod error;
pub mod typechecker;

/// The simple types in the language.
#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Unit,
    Int,
    Abs(Box<Types>, Box<Types>),
}
//
// pub type Globals = im::HashMap<String, Types>;
//
// /// The Sigma, or the Context used in the language. Which stores the mapping from variables to
// /// its Type.
// pub struct Env {
//     locals: im::HashMap<String, Types>,
// }
//
// impl Env {
//     fn new(locals: im::HashMap<String, Types>) -> Self {
//         Env { locals }
//     }
//
//     fn add_binding(&mut self, name: &str) -> Types {
//         let name = name.to_owned();
//         self.locals.insert(name, Types::Unit);
//         Types::Unit
//     }
//
//     // fn get_type_of(&self, name: &str, globals: &Globals) -> Types {
//     //     let var = self.locals.get(name).unwrap_or_else(|| {
//     //         // If not find in locals, search in Globals
//     //         if let Some(var) = globals.get(name) {
//     //             var.cloned()
//     //         }
//     //     });
//     //
//     //     var.cloned()
//     // }
// }

#[cfg(test)]
mod tests {
    use rustelm_parser::parser::parse;

    #[test]
    fn it_works() {
        assert!(parse("11").is_ok())
    }
}
