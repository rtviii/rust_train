# Resources

- [ ] Rust Nursery: https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html

- [x] The       book        : https://doc.rust-lang.org/book/ 
    - Final project pending
- [x] Rustlings             : https://github.com/rust-lang/rustlings
- [x] Rust      By Example  : https://doc.rust-lang.org/stable/rust-by-example/index.html
- [ ] Working   with strings: https://fasterthanli.me/articles/working-with-strings-in-rust
- [ ] Snippets: https://fasterthanli.me/articles/a-half-hour-to-learn-rust    
- [ ] AoC  solution with rust :The book: https://fasterthanli.me/series/advent-of-code-2020
- [ ] On Boxes: https://fasterthanli.me/articles/whats-in-the-box

After that go to parse.

"The or signifies that if there is no result (i.e. None or Err), then here is one I provide for you! The else part signifies that I will provide the default value, but via a function. This allows a default result to be provided lazily. Passing a value to an or type method will mean it is evaluated regardless of the original value. This is because all parameters in Rust are evaluated before or is called. Because or_else uses a function to provide the default value, this means it is not evaluated unless the original value is an None or Err"

### (Error Handling) Result 

https://www.sheshbabu.com/posts/rust-error-handling/ : 

- `unwrap`
- `unwrap_or_else` 
- `?`
- `Box<dyn Error>` for multiple types
To match on an ambiguous error:

```rust
...
if let Some(err) = e.downcast_ref::<reqwest::Error>(){...}
...
```

### Option

https://www.sheshbabu.com/posts/rust-error-handling/ :

- `unwrap_or`
