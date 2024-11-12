use kompar::parser::parser;
use chumsky::Parser;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    dbg!(parser().parse(src));
}
