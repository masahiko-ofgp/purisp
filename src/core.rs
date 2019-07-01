// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.


#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Symbol {
    Quote,
    Atom,
    Cons,
    Car,
    Cdr,
    Pair,
    Assoc,
    Eq,
    And,
    Not,
    Null,
}

impl Symbol {
    pub fn quote(self) -> Self {
        self
    }
    pub fn to_atom(&self) -> Form {
        match self {
            Symbol::Quote => Form::Atom("quote".to_string()),
            Symbol::Atom => Form::Atom("atom".to_string()),
            Symbol::Cons => Form::Atom("cons".to_string()),
            Symbol::Car => Form::Atom("car".to_string()),
            Symbol::Cdr => Form::Atom("cdr".to_string()),
            Symbol::Pair => Form::Atom("pair".to_string()),
            Symbol::Assoc => Form::Atom("assoc".to_string()),
            Symbol::Eq => Form::Atom("eq".to_string()),
            Symbol::And => Form::Atom("and".to_string()),
            Symbol::Not => Form::Atom("not".to_string()),
            Symbol::Null => Form::Atom("null".to_string()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Form {
    Atom(String),
    T,
    Nil,
    Pair(Box<(Form, Form)>),
    List(Vec<Form>),
}

impl<'a> From<&'a str> for Form {
    fn from(s: &'a str) -> Form {
        Form::Atom(s.to_string())
    }
}

impl From<(Form, Form)> for Form {
    fn from(pair: (Form, Form)) -> Form {
        Form::Pair(Box::new(pair))
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
    //TODO: use??
    pub fn quote(self) -> Self {
        self
    }
    pub fn to_symbol(self) -> Option<Symbol> {
        match self {
            Form::Atom(s) => {
                match &s[..] {
                    "quote" => Some(Symbol::Quote),
                    "atom" => Some(Symbol::Atom),
                    "cons" => Some(Symbol::Cons),
                    "car" => Some(Symbol::Car),
                    "cdr" => Some(Symbol::Cdr),
                    "pair" => Some(Symbol::Pair),
                    "assoc" => Some(Symbol::Assoc),
                    "eq" => Some(Symbol::Eq),
                    "and" => Some(Symbol::And),
                    "not" => Some(Symbol::Not),
                    "null" => Some(Symbol::Null),
                    _ => None
                }
            },
            _ => panic!("ERROR: Not Atom.")
        }
    }
    pub fn get_pair_key(&self) -> Option<Self> {
        match self {
            Form::Pair(p) => {
                let pair = &*p;
                Some(pair.0.clone())
            },
            _ => None
        }
    }
    pub fn get_pair_value(&self) -> Option<Self> {
        match self {
            Form::Pair(p) => {
                let pair = &*p;
                Some(pair.1.clone())
            },
            _ => None
        }
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
                    if self.eq(&attr.get_pair_key().unwrap()) == Form::T {
                        return Some(attr.get_pair_value().unwrap());
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
}
