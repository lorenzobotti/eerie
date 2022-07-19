use std::path::Path;
use eerie::Files;

fn main() {
    let test = include_str!("../tests/calculator.eer.md");
    let (files, _) = Files::from_str(test).unwrap();

    dbg!(&files.0);

    files.run(Path::new("./temp_test")).unwrap();
}
