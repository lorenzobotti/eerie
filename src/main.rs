use eerie::Files;
use std::path::Path;

fn main() {
    let test = include_str!("../tests/cat.eer.md");
    let (files, _) = Files::from_str(test).unwrap();

    files.run(Path::new("./temp_test")).unwrap();
}
