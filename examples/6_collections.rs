use std::collections::HashMap;
fn main() {
    //Vec
    let mut a = vec![1, 2, 3];

    let b = a.get(0);
    assert_eq!(b, Some(&1));

    assert_eq!(a.capacity(), 3);
    assert_eq!(a.len(), 3);
    a.push(4);
    assert_eq!(a.capacity(), 6);
    assert_eq!(a.len(), 4);
    a.push(5);
    a.push(6);
    a.push(7);
    assert_eq!(a.capacity(), 12);
    assert_eq!(a.len(), 7);

    //String
    let s = "你好".to_string();
    assert_eq!(s.capacity(), 6);

    //hash map
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);
    let team_name = "Blue".to_string();
    let _ = scores.get(&team_name).copied().unwrap();

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
