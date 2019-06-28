// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.


#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Form {
    Atom(String),
    Nil,
    List(Vec<Form>),
}

impl<'a> From<&'a str> for Form {
    fn from(s: &'a str) -> Form {
        Form::Atom(s.to_string())
    }
}

// `list` function like
impl<'a> From<Vec<&'a str>> for Form {
    fn from(v: Vec<&'a str>) -> Form {
        Form::List(
            v.iter()
            .fold(Vec::new(), |mut vc, s| {
                vc.push(Form::Atom(s.to_string()));
                vc
            }))
    }
}

#[allow(dead_code)]
impl Form {
    pub fn quote(self) -> Self {
        self
    }
    pub fn atom(&self) -> bool {
        match self {
            Form::Atom(_) => true,
            Form::Nil => true,
            Form::List(v) => {
                if v.len() == 0 {
                    true
                } else {
                    false
                }
            }
        }
    }
    pub fn cons(&mut self, value: Form) -> Form {
        match self {
            Form::List(l) => {
                let mut new_list = vec![value];
                new_list.append(l);
                Form::List(new_list)
            },
            _ => panic!("ERROR: Not List."),
        }
    }
    pub fn car(&self) -> Self {
        match self {
            Form::List(l) => Form::List(l[..1].to_vec()),
            _ => panic!("ERROR: Not List."),
        }
    }
    pub fn cdr(&self) -> Self {
        match self {
            Form::List(l) => Form::List(l[1..].to_vec()),
            _ => panic!("ERROR: Not List."),
        }
    }
    pub fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Form::Nil, Form::Nil) => true,
            (Form::Atom(a), Form::Atom(b)) => &a[..] == &b[..],
            (Form::List(x), Form::List(y)) => &x[..] == &y[..],
            _ => panic!("ERROR: Couldn't compare."),
        }
    }
    pub fn pair(self, rhs: Self) -> Vec<(Form, Form)> {
        match (self, rhs) {
            (Form::List(l1), Form::List(l2)) => {
                if l1.len() == l2.len() {
                    l1.into_iter()
                        .zip(l2.into_iter())
                        .collect::<Vec<(Form, Form)>>()
                } else {
                    panic!("ERROR: Not List");
                }
            },
            _ => panic!("ERROR: Not List."),
        }
    }
    pub fn assoc(self, plist: Vec<(Form, Form)>) -> Option<Form> {
        match self {
            Form::Atom(_) => {
                for (key, value) in &plist {
                    if key.eq(&self) {
                        return Some(value.clone());
                    }
                }
                None
            },
            _ => panic!("ERROR: Key Error.")
        }
    }
    pub fn null(&self) -> bool {
        match self {
            Form::Atom(_) => false,
            Form::Nil => true,
            Form::List(l) => {
                l.is_empty()
            },
        }
    }
    pub fn append(&mut self, other: &mut Self) {
        match (self, other) {
            (Form::List(l1), Form::List(l2)) => {
                l1.append(l2);
            },
            _ => panic!("ERROR: Not List.")
        }
    }
}

//lambda => ||
//cond => if
