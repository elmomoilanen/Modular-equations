# Modular equations

[![main](https://github.com/elmomoilanen/Modular-equations/actions/workflows/main.yml/badge.svg)](https://github.com/elmomoilanen/Modular-equations/actions/workflows/main.yml)
[![crate](https://img.shields.io/crates/v/modular_equations.svg?logo=rust&color=orange)](https://crates.io/crates/modular_equations)

Program to solve quadratic and linear modular equations `ax^2 + bx + c = d (mod n)` where x represents the unknown and coefficients from a to d residue classes belonging to the ring of integers Z/nZ. Modulo n must be a positive integer and strictly larger than one.

Solutions, if any, are given as residue classes represented by the smallest nonnegative integers belonging to the corresponding classes.

## Install ##

For the library target, add the following to your `Cargo.toml`

```toml
[dependencies]
modular_equations = "1.0.4"
```

For the binary target, run command `cargo install modular_equations` and make sure that the installation location is in PATH. After that the command `modular_equations --help` should work and show further usage advice.

## Use ##

Use the library to solve quadratic equations as follows

```rust
use modular_equations::{QuadEq, QuadEqSigned};

// Solve equation x^2 + 3x + 4 = 0 (mod 2^60)
let quad_eq = QuadEq::<u64> {a: 1, b: 3, c: 4, d: 0, modu: 2u64.pow(60)};

// Method `solve` returns Option<Vec<T>>, T is now u64
if let Some(x) = quad_eq.solve() {
    // Check that the returned solution `x` is correct
    assert_eq!(x, vec![226_765_812_977_082_276, 926_155_691_629_764_697]);
}

// Solve equation -x^2 + 2x - 1 = 0 (mod n), modulo `n` is now a semiprime
// Coefs `a` and `c` are signed, hence must use signed type equation
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

Linear modular equations are generally much easier to solve than quadratic equations. Following code block shows an example of solving a linear equation that ultimately does not have any solutions.

```rust
use modular_equations::LinEq;

// Try to solve 17x = 1 (mod 255), or in other words find multip inverse for 17
let lin_eq = LinEq::<u8> {a: 17, b: 0, c: 1, modu: u8::MAX};

// 17 doesn't have multiplicative inverse in this case
assert_eq!(lin_eq.solve(), None);
```

For linear equations with signed coefficients there is type `LinEqSigned` available.

If the binary target was installed, CLI can be used as follows (solving the same quadratic equation as above)

```bash
modular_equations 1 3 4 0 $((2 ** 60))
```

Solutions for the equations are printed on their own lines to stdout. Notice that CLI always assumes a signed type for the equation coefficients and the modulo will take the corresponding unsigned type. This indicates that the CLI cannot take argument values above *i128::MAX* for coefficients of the equation.

Notice that some equations have a huge amount of solutions and in these cases the solver might slow down considerable or even panic when the solution count exceeds *usize::MAX*. But these are really special cases and usually not very much of interest.

## License ##

This program is licensed under the [CC0v1](https://github.com/elmomoilanen/Modular-equations/blob/main/LICENSE).
