# RustBuster

RustBuster is a lightweight and efficient directory brute-forcing tool inspired by the original DirBuster. This tool helps in discovering hidden directories and files on web servers by making HTTP requests based on a wordlist. Built using Rust, it leverages asynchronous programming to handle multiple requests efficiently.

## Features

- Command-line interface for ease of use
- Ability to specify target URL and wordlist file
- Optional delay between requests to avoid overloading the server
- Color-coded output for better readability (green for found URLs, red for not found)
- Concurrent request handling for fast performance

## Usage

1. **Clone the repository:**

```sh
git clone https://github.com/yourusername/dirbuster-clone-rust.git
cd dirbuster-clone-rust
```
   
2. Build the project

```sh
cargo build --release
```
3. Run the tool:

```sh
cargo run -- --url http://example.com --wordlist wordlist.txt --delay 2
```


## License
This project is licensed under the Attribution License, which requires that you give appropriate credit to the original author.

## Contribution
Feel free to open issues or submit pull requests if you have suggestions or improvements.

## Acknowledgements
Special thanks to the Rust community for providing excellent resources and support.

