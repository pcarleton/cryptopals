https://doc.rust-lang.org/std/iter/struct.Map.html

^ that's the map function, I want a hash map.

https://doc.rust-lang.org/book/ch08-03-hash-maps.html


```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```



what I want, but with words instead of chars:
```
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
```

To get chars from a string:
https://gist.github.com/jimmychu0807/9a89355e642afad0d2aeda52e6ad2424
```
  let src4: &str = r#"g{'race'}"#;
  let string4 = String::from(src4);
  let char4: Vec<char> = src4.chars().collect();
```