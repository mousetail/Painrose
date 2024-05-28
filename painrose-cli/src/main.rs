use painrose_lib::{geometry, language};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let code = std::fs::read_to_string(&args[1]).unwrap();
    let mut program =
        language::LanguageState::<geometry::rhomb::RhombTiling>::new_from_string(code).unwrap();
    println!("{args:?}");

    program.draw().unwrap();

    let mut i = 0;
    while program.is_running() {
        program.step();
        i += 1;
        if i > 100 {
            break;
        }
    }
}
