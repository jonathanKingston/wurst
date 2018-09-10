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
    title: "2323",
    lang: "boom"
});
div.create();
div.add_to_body();

let mut input = create_element!("input", {
    // Element interface
    id: "Boop",
    // Input interface
    value: "hyyyyy",
    // HTMLElement interface
    title: "2323",
    lang: "boom"
});
input.create();
input.add_to_body();
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

## Next

```rust
let mut input = create_element!("input", {
    // Element interface
    id: "Boop",
    // Input interface
    value: "hyyyyy",
    // HTMLElement interface
    title: "2323",
    lang: "boom"
});

let el = input.create();

el.add_to_body();

el.id = "burp";
//  Flush attrs to the DOM
el.flush();
```

## Ideas

Expected API usages and ideas.

### Attribute

```rust
#[derive(serializable)]
struct serverResponse {
    title: Option<String>
}

#[derive(serializable, Element=HTMLDivElement)]
struct serverResponse {
    title: Option<String>
}

/* Element should create the methods for create() and set_attribute().
   This would be based on the types in Div but also the data passed in.
   This will create a compile time check of the types and provide useful methods on updating the data.

let sr = serverResponse {
  title: None,
};
let el = sr.create();
// or:
sr.set_attributes(el);
*/

serverResponse.to_json();

// Idea to solve unwrapping objects into assignment
let sr = serverResponse.unwrap(json)?;
setAttributes!(el, sr);
```

### Macro use

```rust
let el = create_element!("div", {
    "title": "2323",
    "value", "boom"
});

el.add_event_listener("click", |e| {

});
```

### Code generation method

Escape hatch for setting args from an unsized.

```rust
impl HTMLDivElement {
    set_argument(arg, val) -> Result<_, Err> {
        match arg {
          title => self.set_title(val);
          ...
        }
    }
}

let res: bool = setAttributesHashMap!(el, arguments);

/* Macro could produce:
arguments.map(|arg, val| {
    match arg {
        title => el.set_title(val);
    }
})
*/
```

```rust
let el: HTMLDIVElement = document.createElement("div");
el.set_title("boop");
```
