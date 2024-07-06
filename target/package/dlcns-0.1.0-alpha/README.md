# DLCNS

DarkLight Centeralized Name Serice (library) is used for getting human-readable names that are asociated with the Id and port
of a DarkLight server. This library can get the Id of a name or a name from an Id. See ``examples/`` for usages.

```rust
use dlcns::get::CNSGet;

fn main() {
    let mut cnsget = CNSGet::new();

    let id = cnsget.get_owner_name(String::from("test"));

    if id.is_some() {
        println!("{:?}", id.unwrap());
    } else {
        println!("Not found");
    }
}
```
