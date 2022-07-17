use eerie::Files;

fn main() {
    let test = include_str!("../tests/calculator.eer");
    let (files, _) = Files::from_str(test).unwrap();

    dbg!(files.0);
}
