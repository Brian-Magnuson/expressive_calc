# Expressive Calc

A simple calculator designed to scan strings and evaluate them as mathematical expressions.

## Features

- Evaluates primary expressions: `"1"` -> `1.0`
- Supports negation: `"-1"` -> `-1.0`
- Supports binary operators: `+`, `-`, `*`, `/`, `^`, `%`
  - `"1 + 2"` -> `3.0`
  - Order of operations is as follows: `^`, then `*`, `/`, `%`, then `+`, `-`
- Supports parentheses: `"(1 + 2) * 3"` -> `9.0`
- Supports special constants: `"pi / 2"` -> `1.5707963267948966`
- Supports special functions: `"sin(pi / 2)"` -> `1.0`

### State

The calculator is designed to store some state between evaluations. Each call to `Calculator::evaluate` will store the result, if valid, to variables `$0`, `$1`, `$2`, etc. The last result is also stored in `$ans`. The state can be cleared by calling `Calculator::clear`.

The user can then reference these variables in future expressions. For example, evaluating `"1 + 2"` will store `3.0` in `$0`. The user can then evaluate `"$0 + 3"` to get `6.0`.

Additionally, the user can evaluate an expression without storing the result by calling `Calculator::quick_evaluate`.
