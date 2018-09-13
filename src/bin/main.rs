extern crate parser;

#[allow(dead_code)]
fn test_parser_t() {
    use parser::parser_t as p;
    println!("{:?}", p::DefinitionsParser::new().parse("def _t : int; def _t : int") );
    println!("{:?}", p::DefinitionsParser::new().parse("def identifier : int ; def _t : int") );
    println!("{:?}", p::DefinitionsParser::new().parse("def identifier : int ; def identifier : int;") );
    //assert!(parser_t::DefinitionsParser::new().parse("22").is_ok());
}

#[allow(dead_code)]
fn test_parser() {
    use parser::parser as p;
    println!("Hello, world!");
    println!("{:?}", p::DefinitionsParser::new().parse("def var : int => $ >= 0 && $ <= 255 ") );
    println!("{:?}", p::DefinitionsParser::new().parse("int") );
    println!("{:?}", p::DefinitionsParser::new().parse("float where val < 22 or val < 3 ") );
    println!("{:?}", p::DefinitionsParser::new().parse("float") );
    println!("{:?}", p::DefinitionsParser::new().parse("string where val < 22 + 1 AND val > ( 0 + 1 ) * 2") );
    println!("{:?}", p::DefinitionsParser::new().parse("int where val in ( 0.22 12 2 )") );
    println!("{:?}", p::DefinitionsParser::new().parse("int where val not in ( 0.22 12 2 )") );
}

#[allow(dead_code)]
fn test_parser_2() {
    use parser::parser as p;
    println!("{:?}", p::DefinitionsParser::new().parse("def nulable_int : null | int ;") );
    println!("{:?}", p::DefinitionsParser::new().parse("def nulable_byte : null | int where value >= 0 and value <= 255;") );
    println!("{:?}", p::DefinitionsParser::new().parse("def nulable_struct : null | { req id : int, name : string, misc : null | int  } ;") );
    println!("{:?}", p::DefinitionsParser::new().parse("def array : [ null | int ; 24 ] ;") );
    println!("{:?}", p::DefinitionsParser::new().parse("def int : null | int ;") ); // return error and it is a right behaivior
    println!("{:?}", p::DefinitionsParser::new().parse("def array : [ int ; 24 ] ;") );
    println!("{:?}", p::DefinitionsParser::new().parse("def array : [ int ; 24.. ] ;") );
    println!("{:?}", p::DefinitionsParser::new().parse("def array : [ int ; 24..32 ] ;") );
    println!("{:?}", p::DefinitionsParser::new().parse("def array:[int;..32];") );
    println!("{:?}", p::DefinitionsParser::new().parse("def array:[];") );
    println!("{:?}", p::DefinitionsParser::new().parse("def object:{};") );
    println!("{:?}", p::DefinitionsParser::new().parse("def tuple:();") );
    println!("{:?}", p::DefinitionsParser::new().parse("def tuple:([{}]);/* a */") );

}

fn main() {
    test_parser_2();
}
