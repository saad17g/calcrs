## CalcRS

This project implements a simple command line application that evaluates mathematical expressions.
The application handles both double and integer values, and supports the basic arithmetic operations (+, -, /, \*) as well as the following mathematical functions:

- cos; acos
- sin; asin
- tan; atan
- sqrt
- pow

An example of expression that the application can evaluate is: 'cos(1) + (2 \* 3 - 10.5)/sqrt(4)'

For the trigonometrical function, the input is considered to be in radians (not degrees)

### Run the app

To run the app:

- Run: `rustc ./src/main.rs` to generate the binary
- `./main '1 + (2 * 3 - 10.5)'` or `./main.exe` (Windows/ Powershell)

### Documentation

- Run `cargo doc --open`
