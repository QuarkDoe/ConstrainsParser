
pub mod ast;
pub mod parser;
pub mod parser_t;

#[test]
fn parser_t() {
    use parser_t as p;
    println!("{:?}", p::DefinitionsParser::new().parse("int") );

    //assert!(parser_t::DefinitionsParser::new().parse("22").is_ok());
}
