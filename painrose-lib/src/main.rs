pub mod geometry;
pub mod language;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!("{args:?}");

    let code = std::fs::read_to_string(&args[1]).unwrap();
    let program = language::LanguageState::<geometry::rhomb::RhombTiling>::new_from_string(code);

    program.draw().unwrap();
}
