use purisp::core::Form;
use purisp::{reader, eval};


fn main() {
    let sexp = "(cons 1 2)".to_string();

    let purisp_list = reader(sexp);

    println!("{}", eval(purisp_list));
}
