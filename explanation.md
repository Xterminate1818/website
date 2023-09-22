
## Operators
Below are the supported operators and their usage:
 - Addition `a+b`
 - Subtraction `a-b`
 - Negation `-a`
 - Multiplication `a*b`
 - Division `a/b`
 - Exponentiation `a^b`

Equations are evaluated using the standard order of operations. Parenthesis `()` and other grouping symbols `{} []` are supported as well


## Constants
Mathematical constants can be used in expressions. Below are the names of supported constants (in bold) and their values:
 - **pi** - [π](https://en.wikipedia.org/wiki/Pi)
 - **tau** - 2π
 - **e** - [Euler's constant](https://en.wikipedia.org/wiki/Euler%27s_constant)
 - **i** - [√-1](https://en.wikipedia.org/wiki/Imaginary_number)


## Functions
Functions are written in the form `f(x)` where `f` is the functions name, and `x` is it's input, which can be a constant or another expression. Below are supported function names (in bold):
 ### [Trig functions](https://en.wikipedia.org/wiki/Trigonometric_functions):
- **sin**
- **cos**
- **tan**
- **sec**
- **csc**
- **cot**

### [Inverse trig functions](https://en.wikipedia.org/wiki/Inverse_trigonometric_functions):
- **arcsin**
- **arccos**
- **arctan**

### Miscellaneous:
- **ln** - [log natural](https://en.wikipedia.org/wiki/Natural_logarithm)
- **abs** - [absolute value](https://en.wikipedia.org/wiki/Absolute_value)
- **sqrt** - [square root](https://en.wikipedia.org/wiki/Square_root)


## Notes
Implicit multiplication is supported under most conditions. Below are valid examples of implicit multiplication and its explicit equivalent:
-  `3(4) = 3 * 4` 
- `2pi = 2 * pi`
- `2sin(pi) = 2 * sin(pi)`

However, there is one caveat. The expression `pi(4)` is invalid. This is because `pi` is followed by parenthesis, causing it to look like a function rather than a constant. To resolve this, place the constant after the parenthesis `(4)pi` or use explicit multiplication `pi*(4)`.

Consecutive characters are interpreted as a single constant or function name. This means `picos(0) ≠ pi * cos(0)` because `picos` looks the name of a single function.



