use parser::parse;

mod arena;
mod instruction;
mod parser;

fn main() {
    let result = parse("h(1, 2: v(3, 4), 5)");
    println!("{:#?}", result);
}
