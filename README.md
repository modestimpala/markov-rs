# Markov Text Generation with Rust

This Rust code implements a second-order Markov chain-based text generation algorithm, with weighted distribution. It allows you to generate random text based on the patterns observed in a given set of input texts.

## Requirements

 - Rust/Cargo
 - Tokio crate
 - Rand crate

## Usage

Clone the repository:

    $ git clone https://github.com/modestimpala/markov-rs.git

Build and run the project:

    $ cargo run

Enter the directory path containing the text files when prompted.

The program will process the text files and generate a Markov model based on their contents.

Enter the number of letters you want to generate.

The program will output the generated text.
