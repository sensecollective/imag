## libimagstore

The store is the heart of everything. Here lives the data, the complexity and
the performance bottleneck.

The store offeres read/write access to all entries.

The store itself does not offer functionality, but has a commandline interface
"imag-store" which can do basic things with the store.

### Internal architecture

The internal architecture of the Store has to allow reading/writing from/to
stdin/out and also from/to filesystem.

The user-facing API is the uppermost level of the architecture.
Below that sits a "cache" which holds already-loaded entries in-memory until the
store is closed.
The lowest layer is either (std)IO, Filesystem or an internal representation of
the filesystem (in case of testing).

Between these layers we need

* A layer which generalizes the access to In-Memory/Real Filesystem
* A parser for reading bytes to `Entry` and writing `Entry` to bytes
  Two ways to call this parser exist:
    * Via an API call (e.g. `Store::create()`)
    * Via loading the Store from stdin for example


```
+-------------------------------------------------------+
|                                                       |
| Store API                                             |
|                                                       |
+-------------------------------------------------------+
|                                                       |
| Internal cache                                        |
| Is either a "lazy cache" which caches entries from FS |
| or a "eager IO-read buffer" holding all entries.      |
|                                                       |
| +---------------------------------------------------+ |
| | Parsing layer                                     | |
| | Works either lazily (in the case of "lazy cache") | |
| | Or eagerly (in the case of "eager IO buffer")     | |
| +---------------------------------------------------+ |
|                                                       |
+---------------------------+---------------------------+
|                           |                           |
| Filesystem access         |                           |
| Either In-Memory or On-FS |                           |
|                           |           IO              |
+-------------+-------------+                           |
|             |             |                           |
| Filesystem  |  In-Memory  |                           |
|             |             |                           |
+-------------+-------------+---------------------------+
```

### Long-term TODO

- [ ] Merge with `libimagrt`

