# Add a CNS name

To register a name to the DarkLight Centralized Name Service you will need to send the command ``REQUEST-ADD-NAME`` to
the DarkLight command interpreter. This process can take several minutes, it is recommended that you set your DarkLight
confiuration file to a distributor that is not directly connected to the DarkLight [information server](link).

To start adding your name run:
```command line
sudo dlcmd REQUEST-ADD-NAME <name> <port>
```

