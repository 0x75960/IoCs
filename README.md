IoCs
======

IoC scraper

Usage
-------

```sh
# scrape hashes from url
curl https://example.com | IoCs hashes

# scrape ips from url
curl https://example.com | IoCs ips

# scrape urls from url
curl https://example.com | IoCs urls
```

```sh
$ IoCs help
USAGE:
    IoCs <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    hashes    scrape hashes(sha256, sha1, md5) from stdin pipe
    help      Prints this message or the help of the given subcommand(s)
    ips       scrape ip addresses(allow to contain "[]") from stdin pipe
    urls      scrape URLs(only start with hxxp, allow to contain "[]") from stdin pipe
```

Installation
-------------

```sh
cargo install --git https://github.com/0x75960/IoCs
```
