# CNS - Search

The [dlcns]() library can be used to find a name associated with a DarkLight Id or DarkLight Ids associated with names
but this can be done manually. At the moment there is only one server dedicated to registering and searching names,
some requests might take some time to get a response.

The CNS will respond with a name or Id if the following is sent:
- GET_ID ``name``
    - Responds: ``id`` ``did`` ``port`` ``name`` ``name type`` [source](file and lines)
- GET_ALL_NAMES_ID ``distributor id`` ``local id``
    - Responds with the same contents of ``GET_ID``, if the Id contains multiple names they will be comma seperated
- GET_ALL_NAMES
    - Responds with every registered name and its owner, if many names are registered this will be sent in chunks that will need to be processed
- GET_NAME ``distributor id`` ``local id`` ``port``
    - Responds: ``name`` ``name_type``, if the Id and port of an owner is known you can use this to get the name associated with that Id and port

[Here](link to example) is an example of getting the owner information of a name with [dlcns]():
```rust
use dlcns::get::CNSGet;

fn main() {
    let mut cnsget = CNSGet::new();

    let id = cnsget.get_owner_name(String::from("info.darklight.org"));

    if id.is_some() {
        println!("Owner information: {:?}", id.unwrap());
    } else {
        println!("Owner could not be found or name does not exist");
    }
}

```
