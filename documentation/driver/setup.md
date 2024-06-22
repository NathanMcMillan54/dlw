# Setup

To install ``darklight_driver`` you need to clone the
[DarkLight Github repository](https://github.com/NathanMcMillan54/dlw) and any requirements that are needed.

### Requirements

DarkLight is intended to run on POSIX operating systems such as Linux based OSes and MacOS, DarkLight does not support
Windows.

- [Rust](https://www.rust-lang.org/)
    - ``darklight_driver`` and [``dlwp``]() are written in the Rust language, to compile them you will need to install
    it
    - [Install instructions](https://www.rust-lang.org/learn/get-started)
- [libudev](https://packages.debian.org/sid/libudev-dev)
    - This is required for Linux OSes to interact with [``serial``](serial_com.md) devices
    - Install (``apt``) by running: ```sudo apt install libudev-dev```
- [make](https://www.gnu.org/software/make/)
    - This is used to simplify compiling

### Compiling ``darklight_driver``

```shell script
git clone https://github.com/NathanMcMillan54/dlw && cd dlw/
git checkout <latest version>
make all DLU_KEY=<your valid key> RELEASE=true # Compiles dlwp, darklight_driver, and other tools
DLU_KEY=<your valid key> cargo build -p darklight_driver --release # Or manually compile it
# It's recomended that --release is used ^
```

Running this will generate an executable in ``target/release`` called ``darklight_driver``, move this executable to a
place where it can be easily found such as ``/home/$USER/.local/bin/``. ``darklight_driver`` needs a configuration file
that will tell it which distributor to connect to and how. More information about the configuration file can be found
[here](). An empty configuration file can be found in ``darklight_driver/`` called ``empty_config.json``, it would be
easy to copy it to a place where it can be easily found (maybe with ``darklight_driver``).

### Running

Right now DarkLight mainly supports TCP for communication so this example will show how to connect to DarkLight over
TCP. The configuration file is written in JSON so it must be given valid JSON values, the empty configuration file
looks like this:
```json
{
    "tcp": false,
    "serial":false,
    "serial_path": "/dev/ttyUSB0",
    "closed": true,
    "ip_address": "127.0.0.1:5000",
    "public_instance_id": 0
}
```

Change ``tcp`` to ``true`` and ``closed`` to ``false``, the file should look like this:
```json
{
    "tcp": true,
    "serial":false,
    "serial_path": "/dev/ttyUSB0",
    "closed": false,
    "ip_address": "127.0.0.1:5000",
    "public_instance_id": 0
}
```
If ``closed`` is set to ``true`` ``darklight_driver`` will not attempt to connect to anything. When ``tcp`` is set to
``true`` ``darklight_driver`` will attempt to connect to ``ip_address`` which is the address of the distributor you will
use. [Here]() is a link to all DarkLight TCP distributors.

``darklight_driver`` will add files which it reads and writes to in the root directory (``/etc/`` and ``/tmp/``) so it
will need permissions to run:
```shell script
sudo ./path/to/darklight_driver path/to/config.json
```
