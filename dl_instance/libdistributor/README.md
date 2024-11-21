# lib_dldistributor/distributor

This package is for the DarkLight distributors which are described [here](../../../documentation/distributors.md), this
also contains a library to help write a distributor intended for other instances. The library documentaiton gives some
explaination on how to use some things but the code in ``src/distributor/`` can be used as an example.

### Build

The code for a running distributor is in ``src/distributor/`` (starting from ``main.rs``). This requires many
environemnet variables which are listed in ``envvars`` and ``test_envvars``, the ``test_envvars`` file is used when
building with debug and the distributor will on ``127.0.0.1:5000``. These can be changed if building for release.

Build (for testing):
```command line
cargo build -p distributor --features bin --bin distributor
```

Build (for release):
```
todo
```
