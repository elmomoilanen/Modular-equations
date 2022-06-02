# Modular equations

[![main](https://github.com/elmomoilanen/Modular-equations/actions/workflows/main.yml/badge.svg)](https://github.com/elmomoilanen/Modular-equations/actions/workflows/main.yml)

Program to solve quadratic and linear modular equations `ax^2 + bx + c = d (mod n)` where x represents the unknown and coefficients from a to d residue classes belonging to the ring of integers $\mathbb{Z}/n\mathbb{Z}$. Modulo n must be a positive integer and strictly larger than one.

Solutions, if any, are given as residue classes represented by the smallest nonnegative integers belonging to the corresponding classes.

## Install ##

For the library version, add the following to your `Cargo.toml`

```toml
[dependencies]
modular_equations = "1.0"
```

To use the CLI, you may just clone this repo and run `cargo build --release` againts it afterwards.

## Use ##


## License ##

This program is licensed under the [CC0v1](https://github.com/elmomoilanen/Modular-equations/blob/main/LICENSE).
