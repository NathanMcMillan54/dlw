# Add a CNS name

To register a name to the DarkLight Centralized Name Service you will need to send the command ``REQUEST-ADD-NAME`` to
the DarkLight command interpreter. This process can take several minutes. It is recommended that you set your DarkLight
configuration file to a distributor that is not directly connected to the DarkLight
[information server](../information_servers.md).

To start adding your name run:
```command line
sudo dlcmd REQUEST-ADD-NAME <name> <port> <type>
```

### Name types

To organize names and make their use clear, names will be prefixed with ``info.`` or ``visu.`` and suffixed with
``.org``, ``.com``, or ``.prs``. ``info.`` should be used for servers that are only for sending and receiving
information that usually would not be displayed, it could be used for APIs that would be widely used. ``visu.`` could
be used for servers that return ``html`` or ``md`` files that can be displayed and interacted with in a
[browser](../browser.md). ``.org`` should be used for organizations that are not intended to be for-profit, using this
is a mild suggestion. ``.com`` should be used for organizations or businesses that are intended for making profit,
using this is a mild suggestion. ``.prs`` is intended for individual personal servers, like a personal website, or a
personal storage server.

Name type numbers:

0. ``info.``name``.org``
1. ``info.``name``.com``
2. ``visu.``name``.org``
3. ``visu.``name``.com``
4. ``info.``name``.prs``
5. ``visu.``name``.prs``

### Valid names

Before trying to register a name, you should [check](search.md) if the name has already been registered. DarkLight CNS
will accept any name that is considered [human readable](https://en.wikipedia.org/wiki/List_of_Unicode_characters) such
as alphabets, numbers, and characters that look like numbers or letters. Allowing these characters can make more names
to be available and found more easily by non-latin speakers.

Example:

    visu.ℂⓄoЛ-ǝɯɐu4Ⅱ.prs
    ("cool name 42")
Is a valid name

Spaces, punctuation, [white space](https://en.wikipedia.org/wiki/Whitespace_character) characters,
[emojis](https://en.wikipedia.org/wiki/Emoji#In_Unicode), and special characters that do not resemble real letters are
not valid in names.

### Registering

To identify owners of names CNS uses your first and current [DLU key](../driver/keys.md), if you have recently started
using DarkLight and want to register a name you should wait until your key has been [changed](../driver/updating.md) at
least once to prevent it from being [stolen](../driver/keys.md). A name can only be registered once per month, this
prevents users from registering many names and prevents [name parking](#parking). It is strongly recommended that you
do not register a name through a [third party](../third_party_services.md) that claims to be a CNS registry, you should
always use it through [``darklight_driver``](../driver/main.md).

### Parking and Unethical Registered Names

Name parking is where a person or organization will register a name and not use it in any way for various reasons. 
Coming up with original or simple names can be hard, when a person has a great idea it would be nice for them to have a
name that will be associated with that idea. Name parking is wrong and there is not much that can be done about it. If
you are going to register a name please use it well. If you suspect that someone is parking a name make an issue about
it [here](gh issue). 

If you suspect a registered name is being used for unethical purposes such as:
- Doxing
- Non-consensual Data collection
- Distribution of Malicious Software or Content
- Etc...

Open an issue [here](gh issue) for it to be reviewed and possibly taken down.
