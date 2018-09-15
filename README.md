# WURST

Webby DOM abstraction.

## Aims

- Provide compile time checking as a default for params
- Have an escape hatch to the type system for runtime checking

## Build

```bash
source build.sh
```

## Usage

Current usage of the library:

```rust
let mut div = create_element!("div", {
    // Element interface
    id: "Boop",

    // HTMLElement interface
    title: "hey I am a title",
    lang: "en-GB"
});
div.create();
div.add_to_body();

let mut input = create_element!("input", {
    // Element interface
    id: "Boop",

    // Input interface
    value: "hey!",

    // HTMLElement interface
    title: "2323",
    lang: "boom"
});
input.create();
input.add_to_body();

input.attrs.id = Some("boo".into());
input.update();
```

The above code internally will return an `El` which implements a `Attributish` field trait.
The concrete types are `HTMLDivElementAttributes` and `HTMLInputElementAttributes` respectively.

The macro will only accept what the concrete interface will accept. So providing the wrong element attributes to the macro will fail at compile time:

```
error[E0560]: struct `wurst::attr::HTMLDivElementAttributes` has no field named `value`
  --> src/main.rs:20:9
   |
20 |         value: "my value"
   |         ^^^^^ `wurst::attr::HTMLDivElementAttributes` does not have this field
   |
   = note: available fields are: `title`, `lang`, `dir`, `inner_text`, `access_key` ... and 3 others

error: aborting due to previous error
```


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
