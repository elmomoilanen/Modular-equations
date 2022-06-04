# Modular equations

[![main](https://github.com/elmomoilanen/Modular-equations/actions/workflows/main.yml/badge.svg)](https://github.com/elmomoilanen/Modular-equations/actions/workflows/main.yml)
[![crate](https://img.shields.io/crates/v/modular_equations.svg?logo=rust&color=orange)](https://crates.io/crates/modular_equations)

Program to solve quadratic and linear modular equations `ax^2 + bx + c = d (mod n)` where x represents the unknown and coefficients from a to d residue classes belonging to the ring of integers Z/nZ. Modulo n must be a positive integer and strictly larger than one.

Solutions, if any, are given as residue classes represented by the smallest nonnegative integers belonging to the corresponding classes.

## Install ##

For the library target, add the following to your `Cargo.toml`

```toml
[dependencies]
modular_equations = "1.0"
```

For the binary target, easiest way to install is just to run `cargo install modular_equations` and make sure that the installation location is in $PATH. Then command `modular_equations --help` should work and show further usage advice.

## Use ##

After installation, use the library to solve quadratic equations as follows

```rust
use modular_equations::{QuadEq, QuadEqSigned};

// Solve equation x^2 + 3x + 2 = 0 (mod 2^30)
let quad_eq = QuadEq::<u32> {a: 1, b: 3, c: 2, d: 0, modu: 2u32.pow(30)};

if let Some(x) = quad_eq.solve() {
    // Test that the returned solution `x` is correct
    assert_eq!(x, vec![1_073_741_822, 1_073_741_823]);
}

// Solve other equation -x^2 + 2x - 1 = 0 (mod n), where modulo `n` is now a semiprime
// Coefs `a` and `c` are signed, hence must use signed type equation (both types must have the same size in bytes!)
let quad_eq = QuadEqSigned::<i128, u128> {
    a: -1,
    b: 2,
    c: -1,
    d: 0,
    modu: 2_082_064_493_491_567_088_228_629_031_592_644_077,
};

if let Some(x) = quad_eq.solve() {
    // Residue class [1] is the only solution
    assert_eq!(x, vec![1]);
}
```

Linear modular equations are generally much easier to solve than quadratic equation. Following code block shows an example of solving a linear equation that ultimately does not have any solutions.

```rust
use modular_equations::LinEq;

// Try to solve equation 17x = 1 (mod 255), which basically tries to find the multiplicative inverse of 17 in Z/255Z
let lin_eq = LinEq::<u8> {a: 17, b: 0, c: 1, modu: u8::MAX};

// As 17 doesn't have multiplicative inverse in this case, there aren't solutions for the equation
assert_eq!(lin_eq.solve(), None);
```

CLI usage should also be simple as the following example of solving the same quadratic equation as above indicates

```bash
modular_equations 1 3 2 0 $((2 ** 30))
```

Solutions for the equation are printed on their own lines to stdout. Notice that CLI always assumes a signed type for the equation coefficients and the modulo will take the corresponding unsigned type.

## License ##

This program is licensed under the [CC0v1](https://github.com/elmomoilanen/Modular-equations/blob/main/LICENSE).
