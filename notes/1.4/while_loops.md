
handling [[1.4/lifetimes]] can encourage making a Vec and then fully consuming that Vec to put it an owned value in a "result" Vec.

For instance, if I have a Vec<String>, and I want to add scores, so I have Vec<(i16, String)>, the easiest way I've found so far to do that is to pop things off the original Vec and push them on to a new Vec.

Maybe Rc's would be easier?  Might be worth knowing the overhead.  I'll see if there's a way to incorporate them in the next one.


https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html

```
 while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
```

