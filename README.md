# STDSHOUT!!!1!

**CAUTION: It will shout at you!!!1!**

> [@Argorak](https://github.com/skade): stdshout
> [@pobocks](https://github.com/pobocks): now I kind of want a magic fd that assumes utf-8 and upcases everything and appends !!!1! to it.

It all started [with a tweet](https://twitter.com/Argorak/status/836532440420204544) and [a reply](https://twitter.com/pobocks/status/836534631822409728).

Turns out it takes just a few lines of Rust to handle stuff as UTF-8 and append things.
It's more C code to turn that into your program's default stdout behaviour.

But with some tricky file descriptor redirection, forking and preloading we can turn your precious stdout
into the shouting machine you always wanted.

Currently only works on Linux.

## Requirements

* rustc and cargo (any version)
* gcc

## Example

```
$ make run
$ echo Hello World
HELLO WORLD!!!1!
```

## Shout yourself

```
make run
```

## License

MIT LICENSE!!!1! SEE [LICENSE!!!1!](LICENSE)
