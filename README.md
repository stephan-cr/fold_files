Fold_Files
==========

Display disk usage of some directory tree as a flame graph.

This is a port of the files.pl script, which is part of the
[FlameGraph repository](https://github.com/brendangregg/FlameGraph),
to [Rust](https://www.rust-lang.org/).

Why?: No particular reason.

Installation
------------

```
cargo install --git https://github.com/stephan-cr/fold_files.git
```

How to use
----------

```shell
fold_files <directory> | perl flamegraph.pl --hash --countname=bytes --nametype=Directory: > out.svg
```

Alternatively, instead of the original Flamegraph implementation,
[Inferno](https://github.com/jonhoo/inferno) might be used.

```shell
fold_files --buffered . | inferno-flamegraph --hash --countname=MB --nametype=Directory: --height 24 --factor $(( 1. / 1024 / 1024 )) > out.svg
```

Note: `$(( 1. / 1024 / 1024 ))` is [zsh](https://www.zsh.org/)
specific, bash has something similar, but won't work with floating
point numbers. [Fish](https://fishshell.com/) has
[`math`](https://fishshell.com/docs/current/cmds/math.html).

License
-------

Same license as the original script, which is Apache License 2.0.
