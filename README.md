# rpwd

A Rust-based pwd alternative with advanced display options.

## Installation

From Source

To build and install this utility, ensure you have [Rust](https://www.rust-lang.org/) installed, then run:

```
git clone https://github.com/masakuni-ito/rpwd.git
cd rpwd
cargo install --path .
```

## options

| Option         | Description                                       |
|----------------|---------------------------------------------------|
| `-L, --logical` | Display the logical current working directory.   |
| `-P, --physical` | Display the physical current working directory. |
| `-d, --divide`  | Divide path components with spaces.              |
| `-c, --color`   | Display the path with colored components.        |
| `-s, --stairs`  | Display the path in staircase format.            |

## License

MIT
