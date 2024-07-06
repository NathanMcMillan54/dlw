# dlwp

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;DarkLight Web Protocol Library

This library is used for interacting with [``darklight_driver``]() to create or connect to DarkLight streams and send
or receive [``Messages``](https://docs.rs/dlwp/latest/dlwp/message/struct.Message.html). Documentation can be found on
[docs.rs](https://docs.rs/dlwp/latest/dlwp/), examples can be found in the 
[``tests``](https://github.com/NathanMcMillan54/dlw/tree/main/dlwp/src/tests) and
[``test_streams/``](https://github.com/NathanMcMillan54/dlw/tree/main/test_streams)

### Requirements

As mentioned in the ``darklight_driver`` [setup](../documentation/driver/setup.md), ``libudev`` is needed to compile on
Linux OSes if the ``use_io`` feature is enabled.

### Features in ``Cargo.toml``:
```toml
[features]
# Enable if testing a stream
test_stream = []
# If enabled the serde and serde_json crates will be publicly available from dlwp
include_serde = ["serde/serde_derive", "serde/std"]
# If enabled the chrono crate will be publicly available from dlwp
include_chrono = ["chrono"]
# Used by DarkLight driver, this is not needed for regular use
use_io = ["serialport"]
```
