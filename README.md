# CLI Arithmetic Evaluator

Appears to be a calculator, but is in fact a way for me to test the depths of Rust's core concepts like scanning, lexing, parsing, and other fundementals behind compiler design.

## Project Requirements

[] user should be able to input into command line as normal/free text
[] valid input should return correct calculations
[] invalid input should return error message
[] handle edge cases like empty "()"

```mermaid
flowchart TD
    User-Input --> Scanner;
    Scanner --> Lexer;
    Lexer --> Parser;
    Parser --> Evaluator;
    Evaluator --> User-Input;
```
