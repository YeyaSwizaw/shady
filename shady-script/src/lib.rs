pub mod ast;
pub mod image;

mod grammar;

use image::ImageDefinition;

pub fn parse(input: &str) -> ImageDefinition {
    grammar::parse_ImageDefinition(input).unwrap()
}

#[cfg(test)]
mod test {
    use ::grammar;
    use ::ast;

    #[test]
    fn test_parse_expr() {
        assert_eq!(grammar::parse_Expr("15"), Ok(ast::lit("15")));
        assert_eq!(grammar::parse_Expr("213.87"), Ok(ast::lit("213.87")));
        assert_eq!(grammar::parse_Expr("(3.14, 6)"), Ok(ast::vec2(ast::lit("3.14"), ast::lit("6"))));
    }

    #[test]
    fn test_parse_stmt() {
        assert_eq!(grammar::parse_Stmt("return (3.14, 6)"), Ok(ast::ret(ast::vec2(ast::lit("3.14"), ast::lit("6")))));
    }
}

