// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

use std::fmt;


#[derive(Debug, PartialEq, Clone)]
pub enum Form {
    Atom(String),
    T,
    Nil,
    Pair(Box<(Form, Form)>),
    List(Vec<Form>),
    Lambda(fn(Option<Form>) -> Form),
}

impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Form::Atom(a) => write!(f, "{:?}", a),
            Form::T => write!(f, "T"),
            Form::Nil => write!(f, "NIL"),
            Form::Pair(p) => write!(f, "{:?}", p),
            Form::List(l) => write!(f, "{:?}", l),
            Form::Lambda(_) => write!(f, "LAMBDA"),
        }
    }
}

impl<'a> From<&'a str> for Form {
    fn from(s: &'a str) -> Form {
        Form::Atom(s.to_string())
    }
}

impl Into<String> for Form {
    fn into(self) -> String {
        match self {
            Form::Atom(a) => a[..].to_string(),
            _ => panic!("ERROR")
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for Form {
    fn from(pair: (&'a str, &'a str)) -> Form {
        Form::Pair(Box::new(
                (Form::from(pair.0), Form::from(pair.1))
                ))
    }
}

impl From<(Form, Form)> for Form {
    fn from(pair: (Form, Form)) -> Form {
        Form::Pair(Box::new(pair))
    }
}

impl Into<(String, String)> for Form {
    fn into(self) -> (String, String) {
        match self {
            Form::Pair(_) => {
                let car: String = self.car().unwrap().into();
                let cdr: String = self.cdr().unwrap().into();
                (car, cdr)
            },
            _ => panic!("ERROR")
        }
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

// `list` function like
impl From<Vec<Form>> for Form {
    fn from(v: Vec<Form>) -> Form {
        Form::List(
            v.iter()
            .fold(Vec::new(), |mut vc, f| {
                vc.push(f.clone());
                vc
            })
            )
    }
}

impl Into<Vec<String>> for Form {
    fn into(self) -> Vec<String> {
        match self {
            Form::List(l) => {
                l.clone()
                    .into_iter()
                    .fold(Vec::new(), |mut v, s| {
                        let atm: String = s.into();
                        v.push(atm);
                        v
                    })
            },
            _ => panic!("ERROR")
        }
    }
}

impl Into<Vec<(String, String)>> for Form {
    fn into(self) -> Vec<(String, String)> {
        match self {
            Form::List(l) => {
                l.clone()
                    .into_iter()
                    .fold(Vec::new(), |mut v, tp| {
                        let tuple: (String, String) = tp.into();
                        v.push(tuple);
                        v
                    })
            },
            _ => panic!("ERROR")
        }
    }
}

impl Form {
    //TODO: use??
    pub fn quote(self) -> Self {
        self
    }
    pub fn atom(&self) -> Self {
        match self {
            Form::Atom(_) => Form::T,
            Form::T => Form::T,
            Form::Nil => Form::T,
            Form::Pair(_) => Form::Nil,
            Form::List(v) => {
                if v.len() == 0 {
                    Form::T
                } else {
                    Form::Nil
                }
            },
            Form::Lambda(_) => Form::Nil,
        }
    }
    pub fn cons(&mut self, value: Form) -> Form {
        match self {
            Form::List(l) => {
                let mut new_list = vec![value];
                new_list.append(l);
                Form::List(new_list)
            },
            _ => Form::Pair(Box::new((self.clone(), value))),
        }
    }
    pub fn car(&self) -> Option<Self> {
        match self {
            Form::Pair(p) => {
                let pair = &*p;
                Some(pair.0.clone())
            },
            Form::List(l) => Some(Form::from(l[0].clone())),
            _ => None,
        }
    }
    pub fn cdr(&self) -> Option<Self> {
        match self {
            Form::Pair(p) => {
                let pair = &*p;
                Some(pair.1.clone())
            },
            Form::List(l) => Some(Form::List(l[1..].to_vec())),
            _ => None,
        }
    }
    pub fn cadr(&self) -> Option<Self> {
        match self.cdr() {
            Some(frm) => frm.car(),
            None => None,
        }
    }
    pub fn cddr(&self) -> Option<Self> {
        match self.cdr() {
            Some(frm) => frm.cdr(),
            None => None,
        }
    }
    pub fn eq(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Form::T, Form::T) => Form::T,
            (Form::Nil, Form::Nil) => Form::T,
            (Form::Atom(a), Form::Atom(b)) => {
                if &a[..] == &b[..] {
                    Form::T
                } else {
                    Form::Nil
                }
            },
            (Form::Pair(pa), Form::Pair(pb)) => {
                if &*pa == &*pb {
                    Form::T
                } else {
                    Form::Nil
                }
            },
            (Form::List(x), Form::List(y)) => {
                if &x[..] == &y[..] {
                    Form::T
                } else {
                    Form::Nil
                }
            },
            _ => panic!("ERROR: Couldn't compare."),
        }
    }
    pub fn pair(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Form::List(l1), Form::List(l2)) => {
                if l1.len() == l2.len() {
                    let pairs = l1.into_iter()
                        .zip(l2.into_iter())
                        .map(|t| Form::from(t))
                        .collect::<Vec<Form>>();
                    Form::List(pairs)
                } else {
                    panic!("ERROR: Not List");
                }
            },
            _ => panic!("ERROR: Not List."),
        }
    }
    pub fn assoc(self, plist: Self) -> Option<Self> {
        match plist {
            Form::List(l) => {
                for attr in &l {
                    if self.eq(&attr.car().unwrap()) == Form::T {
                        return Some(attr.cdr().unwrap());
                    } else {
                        return None;
                    }
                }
                None
            },
            _ => panic!("ERROR: Not List."),
        }
    }
    pub fn null(&self) -> Self {
        match self {
            Form::Atom(_) => Form::Nil,
            Form::T => Form::Nil,
            Form::Nil => Form::T,
            Form::Pair(_) => Form::Nil,
            Form::List(l) => {
                if l.is_empty() {
                    Form::T
                } else {
                    Form::Nil
                }
            },
            Form::Lambda(_) => Form::Nil,
        }
    }
    pub fn and(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Form::T, Form::T) => Form::T,
            _ => Form::Nil,
        }
    }
    pub fn not(&self) -> Self {
        match self {
            Form::Nil => Form::T,
            _ => Form::Nil,
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
    pub fn apply(self, params: Option<Form>) -> Self {
        match self {
            Form::Lambda(f) => match params {
                Some(p) => f(Some(p)),
                None => f(None)
            }
            _ => panic!("ERROR: Not Function.")
        }
    }
}
