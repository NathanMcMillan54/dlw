# DLW <img src="dl_logo.png" alt="logo" width="50"/>

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;A very dark web for very bright people

---
<br>

DarkLight Web is intended to be private, somewhat secure, [decentralized](docs/instances.md),
alternative web, seperate from the World Wide Web.

This can be used to give people more of a sense of privacy and a "fresh start" in the digital world. DarkLight is still
in early development so it is not recomended to be used commercially. All information being sent and received is
encrypted with the [Cerpton](https://nathanmcmillan54/cerpton) cipher by default but your own [encryption](docs)
algorithm can be applied. DarkLight applications don't connect directly to its receiver which slightly reduces
traceablity, all data is sent through [distributors](documentation/distributors.md) which only store information in
memory.

More information can be found in the main [documentation](documentation/) or in the
[library documentation](https://docs.rs/dlwp/latest/dlwp). Also read [instance0.md](instance0.md) for the offical
DarkLight instance.

## Project Structre:

- ``darklight_driver/``: binary application that allows users to connect to DarkLight
[docs](documentation/driver/main.md)
- ``dl_instance/``: A library and tools for creating and running an [instance](documentation/instances.md)
    - ``dl_instance/distributor/*``: DarkLight [distributors](documentation/distributors.md)
- ``dlwp/``: main library for interacting with DarkLight applications, see [docs](https://docs.rs/dlwp/latest/dlwp)
- ``documentation/``: markdown files that explain how parts of DarkLight work and how to use them
- ``test_streams/test_clinet/``: example DarkLight client, can be used for testing
- ``test_stream/test_server/``: example DarkLight server, can be used for testing
- ``src/`` Darklight [services](documentation/information_servers.md)
- ``tools/client/`` an executable for interacting with clients and servers
- ``tools/dlcmd/``: command that interacts with ``darklight_driver``, explained [here](documentation/cmd.md#dlcmd)
- ``tools/dlcns/``: library for retrieving data from the [Centeralized Name Server](documentation/cns/)
- ``tools/dlup/``: used for checking if DarkLight [is working](documentation/cmd.md#dlup)
- ``tools/new_dlukey/``: used for getting a [new DarkLight key](documentation/driver/keys.md),
[docs](documentation/cmd.md#new_dlukey)

## Motivation

The World Wide Web was originally intended to be decentralized, today it does not seem like it is. Today the majority
of the web that the average person sees is controlled by the same few hosting services and social media companies which
grealty influence what content is shown and collect large amounts of personal data. DarkLight Web can allow people to
create a new web culture, where people and groups could be entirely independent from corporations. If centralization is
supposed to be inevitable under any circumstances, a new [instance](documentation/instances.md) can
be created to prevent large centralization.
