# DarkLight Command Tools

In the ``tools/`` directory there are binaries and libraries that aren't neccesary to run DarkLight but can make using
it easier. When running ``make all`` these binaries are built, they can be built seperatley using ``make build_tools``,
after doing that they can be installed using ``make move_tools`` (sudo is required) which moves them to ``/sbin``.

----

### ``dlcmd``

``dlcmd`` is used for sending "DarkLight commands" to ``darklight_drver``, it can be used to change the original 
configuration or interact with [cns](cns/add.md). At the current moment there is no documentation for DarkLight
commands and ``dlcmd`` does not verify input. All arguments to this command are sent to ``darklight_driver``

### ``dlup``

``dlup`` can be used to check if the DarkLight instance you are using is working properly. This attempts to connect to the
[``recomends``](information_servers.md#recomendations-server) server, if this is successful then DarkLight is likely
working. This does not need an arguments.

### ``new_dlukey``

``new_dlukey`` is used for getting a new DarkLight [key](driver/keys.md). These keys can be used to update the
``darklight_driver`` or given to some else to start using DarkLight. These keys are displayed after they are received,
they are not stored anywhere else. This does not need any arguments.
