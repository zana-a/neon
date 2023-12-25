use crate::core::parser::construct::function::function;

mod core;

fn main() {
    // known issues of function parser: spacing on left side of arrow is fine but on right side it causes error.
    let f = r#"
    def hello(name: string, age: int, repeat: int) -> string do
      true
      false
      123
    end
    "#
    .trim();

    let result = function(f);
    println!("{:#?}", result)
}
