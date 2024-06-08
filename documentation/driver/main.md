# ``darklight_driver``

``darklight_driver`` is an [executable](setup.md) that allows users to interact with each other, this is how
[``dlwp``](link) sends and receives information. After running it for the first time it will generate a directory in
``/etc/`` called "``dlw/``", this will contain files with your first and current [key](keys.md), your DarkLight Id, and
the Id of the [distributor](../distributors.md) you are connected to.


- [Setup](setup.md) ``darklight_driver``
- [Update](updating.md) ``darklight_driver``

---

In the future this should be turned into a [kernel module](https://docs.kernel.org/kbuild/modules.html) for Linux and
[kernel extension](https://developer.apple.com/library/archive/documentation/Darwin/Conceptual/KEXTConcept/KEXTConceptIntro/introduction.html) for MacOS.