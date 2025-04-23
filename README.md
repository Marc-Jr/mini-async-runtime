# mini-async-runtime
A minimal single-threaded async runtime in Rust.

## About
This project is an educational implementation of a lightweight asynchronous runtime in Rust .It demonstrates the fundamentals of task scheduling and execution within a single thread, providing a hands-on understanding of how asynchronous runtimes operate under the hood

## Features

- **Single-threaded Execution** Designed to run on a single thread, making it ideal for embedded systems or learning environment.
- **Task Scheduling** Manages and schedules asynchronous tasks efficientl.
- **Minimal Dependencies** Built with a focus on simplicity and minimalism, avoiding unnecessary dependencie.

## Installation

Clone the repository and build the project using Cargo:
```bash
git clone https://github.com/Marc-Jr/mini-async-runtime.git
cd mini-async-runtime
cargo build
```

Run the exampe:

```bash
cargo run
```

## Example Output

```text
Runtime started...
task one: start
task two: start
task one: done [~1s]
task two: done [~2s]
```
## 📚 Learning Resources

For a deeper understanding of how asynchronous runtimes work, consider exploring:

- [What is an executor?](https://brianshih1.github.io/mini-async-runtime/executor/intro.htl): An in-depth look at the role of executors in asynchronous programing.
- [Writing a small async runtime for Cortex-M micro-controllers with Rust](https://www.ashwinnarayan.com/post/embedded-async-with-rus/): A practical guide to building an async runtime for embedded sysems.

## 🤝 Contributing

Contributions are welcome! Feel free to fork the repository, submit issues, or open pull reqests.
