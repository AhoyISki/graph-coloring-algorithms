# graph-coloring-algorithms ![License](https://img.shields.io/crates/l/graph-coloring-algorithms) [![graph-coloring-algorithms on crates.io](https://img.shields.io/crates/v/graph-coloring-algorithms)](https://crates.io/crates/graph-coloring-algorithms)

Some graph coloring algorithms

In order to run these, just install [rust][__link0], and, from the cloned
repository’s path, just call:

```bash
cargo run --release
```

This program will read any file with a name like “\*.col” in the
directory. These files must be formatted like this:

```text
c This is a comment line, there can be anything in here
c No line can be empty, apart from the last one
c
p edge {vertices} {edges}
e {vertex} {vertex}
```

The vertices should also be 1 indexed, for example:

```text
c This is a comment line, there can be anything in here
c No line can be empty, apart from the last one
c
p edge 3 2
e 1 3
e 1 2
```


 [__link0]: https://www.rust-lang.org/tools/install
