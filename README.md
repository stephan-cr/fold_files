Fold_Files
==========

A port of the files.pl script, which is part of the [FlameGraph
repository](https://github.com/brendangregg/FlameGraph), to Rust.

Why?: No particular reason.

How to use
----------

```shell
fold_files <director> | perl flamegraph.pl --hash --countname=bytes --nametype=Directory: > ~/out.svg
```

License
-------

Same license as the original script, which is Apache License 2.0.
