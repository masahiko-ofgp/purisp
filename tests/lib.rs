use purisp::core::Form;


#[test]
fn test_from_form() {
    let atm = Form::from("1");
    let pair = Form::from(("1", "2"));
    let list = Form::from(vec!["1", "2"]);
    let list2 = Form::from(vec![Form::from("1"), Form::from("2")]);

    assert_eq!(atm, Form::Atom("1".to_string()));
    assert_eq!(
        pair, 
        Form::Pair(Box::new(
                (Form::Atom("1".to_string()), Form::Atom("2".to_string()))
                )
            )
        );
    assert_eq!(
        &list,
        &Form::List(vec![
            Form::Atom("1".to_string()),
            Form::Atom("2".to_string())
        ]));

    assert_eq!(list, list2);
}

#[test]
fn test_into_rust_type() {
    let atom = Form::from("hello");
    let rust_string: String = atom.into();

    assert_eq!(rust_string, "hello".to_string());

    let pair = Form::from(("1", "2"));
    let rust_tuple: (String, String) = pair.into();
    
    assert_eq!(rust_tuple, ("1".to_string(), "2".to_string()));

    let list = Form::from(vec!["1", "2"]);
    let rust_vec: Vec<String> = list.into();
    
    assert_eq!(
        rust_vec,
        vec!["1".to_string(), "2".to_string()]
        );

    let list1 = Form::from(vec!["a", "b"]);
    let list2 = Form::from(vec!["1", "2"]);
    let plist = list1.pair(list2);

    let rust_tuple_vec: Vec<(String, String)> = plist.into();

    assert_eq!(
        rust_tuple_vec,
        vec![
        ("a".to_string(), "1".to_string()),
        ("b".to_string(), "2".to_string()),
        ]
        );
}

#[test]
fn test_atom() {
    let atm = Form::Atom("1".to_string());
    let nil = Form::Nil;
    let pair = Form::Pair(Box::new(
            (Form::Atom(1.to_string()), Form::Atom(2.to_string()))
            ));
    let empty_list = Form::List(vec![]);
    let list = Form::from(vec!["1", "2"]);

    assert_eq!(atm.atom(), Form::T);
    assert_eq!(nil.atom(), Form::T);
    assert_eq!(pair.atom(), Form::Nil);
    assert_eq!(empty_list.atom(), Form::T);
    assert_eq!(list.atom(), Form::Nil);
}

#[test]
fn test_cons_car_cdr() {
    let pair = Form::from("key").cons(Form::from("value"));

    assert_eq!(
        &pair.car().unwrap(),
        &Form::Atom("key".to_string())
        );
    assert_eq!(
        &pair.cdr().unwrap(),
        &Form::Atom("value".to_string())
        );

    let mut list1 = Form::from(vec!["1", "2"]);
    let list2 = &mut list1.cons(Form::from("0"));
    
    assert_eq!(
        list2,
        &mut Form::List(vec![
                        Form::Atom("0".to_string()),
                        Form::Atom("1".to_string()),
                        Form::Atom("2".to_string()),
        ]));
    
    let car = &list2.car();
    let cdr = &list2.cdr();

    assert_eq!(
        car, 
        &Some(Form::Atom("0".to_string()))
        );
    
    assert_eq!(
        cdr,
        &Some(Form::List(vec![
                    Form::Atom("1".to_string()),
                    Form::Atom("2".to_string()),
        ])));
}

#[test]
fn test_pair_assoc() {
    let list1 = Form::from(vec!["a", "b", "c"]);
    let list2 = Form::from(vec!["1", "2", "3"]);

    let pair_list = list1.pair(list2);

    assert_eq!(
        &pair_list,
        &Form::List(vec![
            Form::Pair(Box::new(
                    (Form::Atom("a".to_string()), Form::Atom("1".to_string()))
                    )),
            Form::Pair(Box::new(
                    (Form::Atom("b".to_string()), Form::Atom("2".to_string()))
                    )),
            Form::Pair(Box::new(
                    (Form::Atom("c".to_string()), Form::Atom("3".to_string()))
                    )),
        ]));

    let key = Form::from("a");
    let key2 = Form::from("d");

    assert_eq!(
        key.assoc(&pair_list),
        Form::Atom("1".to_string())
        );
    assert_eq!(
        key2.assoc(&pair_list),
        Form::Nil
        );
}

#[test]
fn test_append() {
    let mut list1 = Form::from(vec!["1"]);
    let mut list2 = Form::from(vec!["2"]);

    &mut list1.append(&mut list2);

    assert_eq!(
        list1,
        Form::List(vec![
            Form::Atom("1".to_string()),
            Form::Atom("2".to_string()),
        ]));
}

#[test]
fn test_and_not() {
    let atm1 = Form::from("1");
    let atm2 = Form::from("2");

    assert_eq!(
        &atm1.atom().and(&atm2.atom()),
        &Form::T
        );

    let list = Form::from(vec!["hello"]);

    assert_eq!(
        &list.atom().not(),
        &Form::T
        );
}

#[test]
fn test_lambda() {
    let hello = Form::Lambda(|_| Form::Atom("Hello".to_string()));

    assert_eq!(
        hello.apply(None),
        Form::Atom("Hello".to_string())
        );

    let lambda = Form::Lambda(|list| list.unwrap().car().unwrap());
    assert_eq!(
        lambda.apply(Some(Form::from(vec!["1", "2", "3"]))),
        Form::Atom("1".to_string())
        );
}
