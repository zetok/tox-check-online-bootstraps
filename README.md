This tool may be used to check whether bootstrap nodes can be used to bootstrap.

To get parsed list of nodes from Tox wiki, you may want to use [tox-wiki-nodes-parser](https://github.com/zetok/tox-wiki-nodes-parser).



# Installation
Running it is fairly simple.

Download [binary for Linux x86_64](https://github.com/zetok/tox-check-online-bootstraps/releases/download/v0.0.0/tox-check-online-bootstraps) and run it.

**Requires no other Tox instances running on the network and [toxcore](https://github.com/irungentoo/toxcore) installed.**

To compile yourself:

1. Install [Rust](http://www.rust-lang.org/)
2. Make with `cargo build`
3. Run with `./target/debug/./tox-check-online-bootstraps`

# Usage

Parser takes content of file `nodes_list` from working directory and prints to stdout list of working nodes.

Provide file `nodes_list` in working directory, with content from from [tox-wiki-nodes-parser](https://github.com/zetok/tox-wiki-nodes-parser), or in format
```
<IP> <PORT> <PUBLIC_KEY>
```
and run checker.



# License

Licensed under GPLv3+, for details see [COPYING](/COPYING).