fn hashmap_example() {
    let mut scores = HashMap::new();

    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    // * get returns an Option<&V>
    let daniel = scores.get("Daniel");
    assert_eq!(daniel, Some(&95));
    // * as_ref() converts Option<V> to Option<&V>
    assert_eq!(daniel, Some(95).as_ref());

    let daniel = scores["Daniel"];
    assert_eq!(daniel, 95);
}