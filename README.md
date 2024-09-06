# caecli

caecli is a command-line interface (CLI) tool written in Rust, designed to provide various utility functions.

## Features
- Base64 encoding/decoding: Encode and decode strings using Base64 encoding.
- CSV processing: Convert CSV files between JSON and YAML formats.
- Password generation: Generate random passwords with customizable length and complexity.
- HTTP file server: Serve files over HTTP.
- JWT sign/verify: Sign and verify JWT tokens.
- Signature sign/verify: Sign and verify text/file with ed25519/blake3.
- Time processing: Convert timestamps to human-readable date and time strings with support for different timezones.

## Installation

To install caecli, make sure you have Rust and Cargo installed on your system. Then, clone the repository and build the project:

```shell
git clone https://github.com/caelansar/caecli.git
cd caecli
cargo build --release
```

The compiled binary will be available in the `target/release` directory.

## Usage

To use caecli, run the compiled binary with the desired subcommand and options:

```shell
caecli [SUBCOMMAND] [OPTIONS]
```

For example, to convert a timestamp to a human-readable date and time string, you can use the `time` subcommand:

```shell
caecli time --timestamp 1633072800s --tz America/New_York
```

This will output the date and time in the specified timezone.

For detailed usage instructions, use the `--help` flag:

```shell
caecli --help
caecli [SUBCOMMAND] --help
```

All supported subcommands:

```shell
caecli base64
caecli csv
caecli genpass
caecli http
caecli jwt
caecli text
caecli time
```


## Contributing

Contributions to caecli are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.
