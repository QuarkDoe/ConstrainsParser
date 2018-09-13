extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new();
    lalrpop::Configuration::new().emit_report(true).process_current_dir().unwrap();
    //lalrpop::process_root().unwrap();
}
