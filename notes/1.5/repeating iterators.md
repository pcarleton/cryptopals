https://stackoverflow.com/questions/28436240/how-can-i-create-an-iterator-that-repeats-multiple-values-infinitely

```
fn main() {
    let values = [1, 2, 3];
    let repeat_123 = values.iter().cloned().cycle();
    for elt in repeat_123.take(10) {
        println!("{}", elt)
    }
}
```


OR

```
let repeat_123 = (1..4).cycle();
```