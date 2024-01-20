use crate::core::parser::construct::module::module;

mod core;

fn main() {
    let result = module(
        r#"
        module A do
          module B do
          end
          module C do
          end
        end
        "#
        .trim(),
    );

    match result {
        Ok((remaining, module)) => {
            println!("Remaining:\n{:?}", remaining);
            println!("Parsed Object:\n{:?}", module);
        }
        Err(error) => panic!("{:#?}", error),
    }
}
