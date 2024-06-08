# Distributors

Distributors are used to send data to other DarkLight users. If ``closed`` is set to ``false`` when 
``darklight_driver`` starts it will attempt to connect to the value of ``ip_address`` or ``serial_path``, this is where
``darklight_driver`` sends and receives data. A distributor *distributes* data to users connected to it or other
distributors that users are connected to. Nothing the distributor receives is saved in storage, it only exists in
memory until it is sent. 

Distributors can help prevent activity from being easily traced. When a distributor receives input from a user it
checks if the receiver is also connected to the distributor, if it is then the input will be imediatley sent to the
receiver. Doing the means that it will be sent quickly but it can be known who the receiver if someone was able to veiw
traffic from the distributor. When something needs to be sent across distributors the entire [``Message``](doc) 
including is the receiving information is encrypted with a setting that is changed at the end of every day 
(``23:59:59 (UTC)``), this ensures that while it is being sent across distributors attempting to decrypt it can be 
difficult if it is veiwed by anyone but the receiver. 

Distributors use the [``cerpton``](https://docs.rs/cerpton/latest/cerpton/) cipher to encrypt data being sent, this is
also how DarkLight [information servers](information_servers.md) encrypt data.

Visualization of how distributors work:
```
             (already
----------   encrypted) 
|        |       v       ------
|   A    |-------------->| #1 |
|        |               ------
----------                  |        ----------
                            |        |        |
    Encrypted again         |------->| C      |<("This makes no sense")
 (receiver is no longer ->  |        |        |
          known)            |        ----------
                            |
                            |
----------                  v
|        |               ------
|   B    |<--------------| #2 |
|        |      |        ------
----------      |
                | (in the case someone finds what is being sent)
                v
            ----------
            |        |
            |   C    | <("I know 'B' but who sent this?")
            |        |
            ----------
```

### Running Your Own Distributor

It is still being determined how this will work
