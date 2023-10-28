# tokenizer-rs

An application written in Rust designed to perform tokenizing. It can be used for different models. This increases its versatility in performing a variety of tasks.

You can choose from multiple tokenizer modes, such as
- `p50k_base` for code models,
- `cl100k_base` for ChatGPT models,
- `p50k_edit` for edit models,
- `r50k_base` for GPT-3 models.

## Table of Contents

1. [Installation](#installation)
2. [Usage](#usage)
3. [Contributing](#contributing)
4. [License](#license)
5. [Acknowledgments](#acknowledgments)
6. [Contact](#contact)

## Installation

To use this program, you must install Rust and its package manager, Cargo. Follow the official [Rust installation guide](https://www.rust-lang.org/tools/install) to get them set up.

Once Rust and Cargo are installed, you can build and install this program using the following command:

```bash
cargo install --path .
```

## Usage

This program offers different tokenizer modes, each designed for specific models:

- `p50k_base`: Use this mode for code models.
- `cl100k_base`: Choose this mode for ChatGPT models.
- `p50k_edit`: Ideal for edit models.
- `r50k_base`: Use this mode for GPT-3 models.

To tokenize text, specify the desired mode using the `-t` or `--mode` flag. For example, to tokenize text for a code model, you can run:

```bash
./tokenizer-rs -t p50k_base input.txt
```

## Contributing

If you'd like to contribute to this project, please follow these guidelines:

1. Fork the repository.
2. Create a new branch for your feature or bug fix: `git checkout -b feature/new-feature`.
3. Make your changes and commit them.
4. Push your changes to your fork: `git push origin feature/new-feature`.
5. Create a pull request to the `main` branch of the original repository.

## License

This project is licensed under the dual MIT and Apache v2 licenses. See the [Apache License v2](LICENSE_APACHE.md) and [MIT License](LICENSE_MIT.md) files for details.

## Acknowledgments

This program relies on external libraries to provide tokenization services. These libraries include:

- [tiktoken_rs](https://crates.io/crates/tiktoken-rs): A Rust library for counting tokens in text.
- [StructOpt](https://crates.io/crates/structopt): A Rust library for command-line argument parsing.

## Contact

If you have any questions or need further assistance, you can contact me at <walker84837@gmail.com>.
