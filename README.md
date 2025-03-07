# RustyPython 
RustyPython is a Python interpreter written in Rust. 

*It is a work in progress and is not yet ready for use.*

## About

I am building the Python interpreter in Rust to learn more about both languages. 
The performance of the interpreter is also of great priority, ideally, I want to beat the current [RustPython](https://github.com/RustPython/RustPython) interpreter in terms of run speed.
I am taking a different approach to this interpreter than they did. Mine is structured far more like the CPython interpreter, while also trying to maintain simplicity.

## Instructions

Rust makes this real simple. Just clone the repo and compile like so.

```bash
cargo build --release
```

## Speed

Currently, this is looking about 2-3x slower than CPython. This is not so bad, heading in the right direction. However, it is also about 3-4x faster than [RustPython](https://github.com/RustPython/RustPython). This is a good sign.

## Tests

Currently working on developing a testing suite. I copied some of the simpler tests from the RustPython repo into `/tests/from_rustpython` and am working on getting them to pass.

For the tests I have written, they are mostly testing the existence and correct implementation of various basic features rather than edge cases yet.
Once the `assert` and method definition is added, more sophisticated tests will be written. However, for now this is the state of the testing system.

| Test Name          | Status | Notes                              |
|--------------------|--------|------------------------------------|
| test_object        | ✔️     |                                    |
| test_simple        | ✔️     | should include more int operations |
| test_addition      | ✔️     | optimized 😎                       |
| test_deep_for_loop | ✔️     | optimized 😎                       |
| test_while_add     | ✔️     | optimized 😎                       |
| test_simple_if_for | ✔️     | optimized 😎                       |
| test_primatives    | 🚧     | need to add all the primatives     |
| test_control_flow  | ❌      | need to add if, strings & indexing |
| test_tuple         | ❌      | need tuples and tuple unpacking    |


## Supported Features

| Feature                | Supported | Notes                                                        |
|------------------------|-----------|--------------------------------------------------------------|
| User-defined Variables | ✔️        |                                                              |
| Print Function         | ✔️        |                                                              |
| Operator overloading   | ✔️        |                                                              |
| For Loops              | ✔️        | Doesn't support tuple unpacking (yet)                        |
| Comments               | ✔️        |                                                              |
| If/if-else Statements  | ✔️        |                                                              |
| While Loops            | ✔️        | Close to CPython speeds!                                     |
| Basic Math Operations  | 🚧        | still needs % and //                                         |
| Math Assign Operations | 🚧        | still needs //=, @=, **=, no support for in-pace methods yet |
| Primatives             | 🚧        | int, float, bool, None (limited implementation)              |
| Built in types         | 🚧        | range only                                                   |                                                       |
| Keyword: assert        | 🚧        | parsing but not implemented                                  |
| Match Statements       | ❌         |                                                              |
| User-defined Functions | ❌         |                                                              |
| User-define classes    | ❌         |                                                              |
| User-define modules    | ❌         |                                                              |
| Error Handling         | ❌         |                                                              |
| Generators             | ❌         |                                                              |
| Importing modules      | ❌         |                                                              |
| Typeing                | ❌         |                                                              |
| Keyword: with          | ❌         |                                                              |
| Keyword: global        | ❌         |                                                              |
| Keyword: del           | ❌         |                                                              |
| Async                  | ❌         |                                                              |
