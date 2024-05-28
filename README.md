# Gale-Shapley Algorithm Implementation in Rust

This repository contains an implementation of the Gale-Shapley algorithm in Rust. The Gale-Shapley algorithm, also known as the stable marriage problem, is a solution to a problem of matching a group of individuals to another group of equal size, based on preferences, in a stable way.

## Features

- Efficient implementation of the Gale-Shapley algorithm.
- Supports an arbitrary number of individuals in each group.
- Preferences can be specified for each individual.
- The solution is always stable, meaning there are no two individuals who would both rather have each other than their current partners.

## Usage

To use this project, first clone the repository:

```bash
git clone https://github.com/yourusername/gale-shapley-rust.git
cd gale-shapley-rust
```

Then, you can run the program with:

```bash
cargo run
```

## Input Format

The program expects input in the following format:

- The first line contains two integers, `m` and `n`, the number of individuals in each group.
- The next `m` lines each contain `n` integers, the preferences of each individual in the first group.
- The next `n` lines each contain `m` integers, the preferences of each individual in the second group.

## Output Format

The program outputs a matching in the following format:

- The first line contains an integer, `k`, the number of matches.
- The next `k` lines each contain two integers, `a` and `b`, indicating that individual `a` from the first group is matched with individual `b` from the second group.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

