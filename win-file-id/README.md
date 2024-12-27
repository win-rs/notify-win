# Win File Id

[![» Docs](https://img.shields.io/docsrs/notify-win)][docs]

A utility to read file IDs.

Modern file systems assign a unique ID to each file. On Windows it is called `file index`.
Together with the `device id`, a file can be identified uniquely on a device at a given time.

Keep in mind though, that IDs may be re-used at some point.

## Example

```rust
let file_id = win_file_id::get_file_id(path).unwrap();

println!("{file_id:?}");
```

## Features

- `serde` for serde support, off by default

[docs]: https://docs.rs/win-file-id
