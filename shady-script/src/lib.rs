extern crate lalrpop_util;

pub use image::Image;

pub mod ast;
pub mod span;

pub use analyse::AnalyseError;
pub use image::Uniform;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParseError<'a>(lalrpop_util::ParseError<usize, (usize, &'a str), ()>);

mod analyse;
mod instr;
mod grammar;
mod image;

#[derive(Debug, Eq, PartialEq)]
pub struct Shady {
    items: Vec<instr::Item>
}

impl Shady {
    fn new() -> Shady {
        Shady {
            items: Vec::new()
        }
    }

    fn get(&self, idx: usize) -> &instr::Item {
        &self.items[idx]
    }

    fn push_item(&mut self, item: instr::Item) {
        self.items.push(item)
    }

    pub fn with_images<F: FnMut(Image)>(&self, mut f: F) {
        for img in self.items.iter()
            .enumerate()
            .filter(|&(_, item)| item.kind == ast::ItemKind::Image)
            .map(|(idx, _)| Image::new(self, idx)) {

            f(img)
        }
    }
}

pub fn parse_input(input: &str) -> Result<ast::AST, ParseError>{
    grammar::parse_AST(input).map_err(ParseError)
}

#[test]
fn test() {
    let ast = parse_file("../script.shy");
    println!("{:?}", ast);
    let sdy = ast.analyse();
    println!("{:?}", sdy);
}
