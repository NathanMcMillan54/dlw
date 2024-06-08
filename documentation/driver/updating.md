# Updating

It is important to update ``darklight_driver`` regularly. If you are going to update ``darklight_driver`` make sure you
have a [new key](keys.md) so you don't lose access to DarkLight. To update ``darklight_driver`` simply run these
commands:

```shell script
git clone https://github.com/NathanMcMillan54/dlw.git && cd dlw/ # Install repository if you haven't already
git checkout <new version number>
# Be absolutely certain that you are not using your last key
make all DLU_KEY=<your new DLU key> BUILD_RELEASE=true
```

The output can be found in ``target/release/`` named ``darklight_driver``
