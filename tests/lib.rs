use purisp::core::Form;

#[test]
fn test_atom() {
    let atm = Form::Atom("1".to_string());
    let nil = Form::Nil;
    let empty_list = Form::List(vec![]);
    let list = Form::from(vec!["1", "2"]);

    assert_eq!(atm.atom(), Form::T);
    assert_eq!(nil.atom(), Form::T);
    assert_eq!(empty_list.atom(), Form::T);
    assert_eq!(list.atom(), Form::Nil);
}

#[test]
fn test_cons_car_cdr() {
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
        &Form::List(vec![
                    Form::Atom("0".to_string())
        ]));
    
    assert_eq!(
        cdr,
        &Form::List(vec![
                    Form::Atom("1".to_string()),
                    Form::Atom("2".to_string()),
        ]));
}

#[test]
fn test_pair_assoc() {
    let list1 = Form::from(vec!["a", "b", "c"]);
    let list2 = Form::from(vec!["1", "2", "3"]);

    let pair_list = list1.pair(list2);

    assert_eq!(
        &pair_list,
        &vec![
            (Form::Atom("a".to_string()), Form::Atom("1".to_string())),
            (Form::Atom("b".to_string()), Form::Atom("2".to_string())),
            (Form::Atom("c".to_string()), Form::Atom("3".to_string()))
        ]);

    let key = Form::from("a");

    assert_eq!(
        key.assoc(pair_list),
        Some(Form::Atom("1".to_string()))
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
        &atm1.atom().and_(&atm2.atom()),
        &Form::T
        );

    let list = Form::from(vec!["hello"]);

    assert_eq!(
        &list.atom().not_(),
        &Form::T
        );
}
