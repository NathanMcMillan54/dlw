# darklight_driver

The DarkLight "driver" is a regular executable that allows you to connect to other users of DarkLight using the 
[``dlwp``]() library. At the moment this is not an actual driver, eventually it should be written as a Linux kernel
module and a MacOS kernel extension.

Linux requirements:
- ``libudev`` (for serial communication)
