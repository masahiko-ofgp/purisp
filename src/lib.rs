// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

pub mod core;
use crate::core::Form;


pub fn reader(sexp: String) -> Form {
    let sexp2 = sexp.replace("(", " ").replace(")", " ");

    let v = sexp2.split_terminator(' ')
        .collect::<Vec<_>>()
        .into_iter()
        .filter(|s| s != &"")
        .collect::<Vec<_>>();

    let mut list: Vec<Form> = vec![];

    for f in &v {
        list.push(Form::from(*f));
    }

    Form::from(list)
}


pub fn eval(form: Form) -> Form {
    match form.car() {
        Some(frm) => {
            if &frm.eq(&Form::from("quote")) == &Form::T {
                form.cdr().unwrap()
            } else if &frm.eq(&Form::from("atom")) == &Form::T {
                eval(form.cadr().unwrap()).atom()
            } else if &frm.eq(&Form::from("eq")) == &Form::T {
                eval(form.cadr().unwrap()).eq(
                        &eval(form.cddr().unwrap().car().unwrap()
                        ))
            } else if &frm.eq(&Form::from("car")) == &Form::T {
                eval(form.cadr().unwrap()).car().unwrap()
            } else if &frm.eq(&Form::from("cdr")) == &Form::T {
                eval(form.cadr().unwrap()).cdr().unwrap()
            } else if &frm.eq(&Form::from("cons")) == &Form::T {
                eval(form.cadr().unwrap())
                    .cons(eval(form.cdr().unwrap().cadr().unwrap()))
            } else {
                Form::Nil
            }
        },
        None => form
    }
}
