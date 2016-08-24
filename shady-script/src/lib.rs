#![feature(conservative_impl_trait)]

use std::path::Path;
use std::fs::File;
use std::io::Read;

pub use image::Image;

pub mod ast;

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

pub fn parse_file<P: AsRef<Path>>(path: P) -> ast::AST {
    let mut source = String::new();

    File::open(path).and_then(|mut file| file.read_to_string(&mut source)).unwrap();
    grammar::parse_AST(&source).unwrap()
}

#[test]
fn test() {
    let ast = parse_file("../script.shy");
    println!("{:?}", ast);
    let sdy = ast.analyse();
    println!("{:?}", sdy);
}
