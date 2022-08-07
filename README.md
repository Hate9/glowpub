# A Glowfic-To-Epub converter

Using this software requires [Rust](https://www.rust-lang.org/tools/install).

---

To process a specific post clone this repo and, from its directory, run:
```
cargo run --example process <post-id>
```

This will download the entire thread and cache it locally, along with all images.
It'll then generate a single html file in `/books/html/<post-id>.html`, and an epub file in `/books/epub/<post-id>.epub`.

---

To process the entire `planecrash` series:
```
cargo run --example planecrash
```
(`planecrash` posts updated on 2022-08-07 from [here](https://glowfic.com/boards/215))
