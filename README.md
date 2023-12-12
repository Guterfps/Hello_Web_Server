# Simple Multi-Threaded Web Server(in Rust)
[![Rust](https://img.shields.io/badge/Rust-1.50%2B-orange.svg)](https://www.rust-lang.org)

## Description
Its the final project from the Rust book :smile:.
A simple Web server handling http requests(concurrently) with a simple thread pool.

## Features

- Basic HTTP request handling
- Multi-threaded architecture with a simple thread pool

## Getting Started

Follow these steps to get the web server up and running on your local machine.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/Guterfps/Hello_Web_Server.git
    cd Hello_Web_Server
    ```

2. Build the project:

    ```bash
    cargo build
    ```

### Usage

Run the web server using:

```bash
cargo run
```
Visit http://localhost:7878 in your web browser, and you should see the server responding. 

See the Docs by running:

```bash
cargo doc --open
```

