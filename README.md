# FindReplace

_FindReplace_ is a command line tool that allows you to find and replace a given pattern with a substitute inside a file.

## Installation

To install _FindReplace_, you need to have _Rust_ and _Cargo_ installed on your system. Then, clone this repository and run `cargo build --release` to build the app. The binary will be located in the `target/release` directory.

This guide provides several methods for installing the _FindReplace_ tool on your computer. Follow the steps below to set up _FindReplace_ using the method that works best for you.

### Method 1: Manual installation

1. Visit the _FindReplace_ GitHub page and navigate to the "Releases" section.
2. Download the latest release for your operating system and extract the contents of the archive.
3. Copy the extracted binary file to a directory in your `$PATH`. On macOS or Linux, this could be `/usr/local/bin`, while on Windows, you could copy the binary to `C:\Windows\system32`.
4. Verify that _FindReplace_ is installed correctly by opening a terminal or command prompt and running the command `findreplace --version`.

### Method 2: Using Cargo

If you have a Rust development environment set up on your computer, you can use the `cargo install` command to install _FindReplace_:

1. Open a terminal or command prompt and run the command `cargo install findreplace`.
2. Cargo will download, build, and install the _FindReplace_ binary.
3. The binary will be placed in `$HOME/.cargo` on macOS or Linux, or `%USERPROFILE%\.cargo\bin` on Windows.
4. Verify that _FindReplace_ is installed correctly by running the command `findreplace --version`.

## Usage

```
FindReplace
A tool to find and replace given pattern with substitute inside file.

Syntax:
  findreplace [options]
  findreplace [option] <pattern> <substitute> <file_path>

where,
  pattern      the pattern to search for in the file
  substitute   the text to replace the pattern with or a JSON/YAML/CSON formatted hash-map
  file_path    the path to the file to search and replace in

Options:
  -h --help       Show help menu
  -V --version    Show version info
  -l --log        Show verbose log

Usages:
  findreplace --help
  findreplace --version
  findreplace 'foo' 'bar' file.txt
  findreplace 'hello world' '{"hello":"hi","world":"earth"}' file.txt
  findreplace 'hello world' '{hello: hi, world: earth}' file.txt
  findreplace 'hello world' 'hello:hi;world:earth' file.txt
  findreplace --log 'hello world' 'hello:hi;world:earth' file.txt

Extras:
  findreplace "`\{[a-zA-Z0-9_-]+`\}" '{:(;}:)' file.txt   [PowerShell]
  findreplace '\{[a-zA-Z0-9_-]+\}' '{:(;}:)' file.txt     [Bash]
  findreplace "\{[a-zA-Z0-9_-]+\}" "{:(;}:)" file.txt    [CMD]

```

### JSON vs YAML vs CSON

**JSON** (JavaScript Object Notation), **YAML** (YAML Ain't Markup Language), and **CSON** (Compact Symbol Object Notation) are all data serialization formats used for storing and exchanging data.

[JSON](https://www.json.org/) is a lightweight data interchange format that is easy for humans to read and write and easy for machines to parse and generate. It is based on a subset of the JavaScript Programming Language and is often used for transmitting data between a server and a web application.

[YAML](https://yaml.org/) is a human-readable data serialization format that is commonly used for configuration files and in applications where data is being stored or transmitted. It is designed to be more readable than JSON and other data serialization formats and uses indentation to indicate nesting of data structures.

[CSON](https://akzcool.blogspot.com/2023/05/introducing-cson-serialization-data.html) is a YAML variation that defines data structures using concise YAML syntax. It strives to be smaller than JSON or YAML by excluding spaces around separators, unquoted keys, and other aspects that make it easier to write and read. It also allows for numerous special characters (symbols) to be used as separators, such as `;`, `,`, `|`, `/`, `~`. Although any of the available symbols can be chosen as a separator, it must be consistent across the document. Combining symbols is not allowed and may result in unexpected outcomes.

Each of these formats has its own strengths and weaknesses. JSON is widely supported and easy to use, but can be more difficult to read than YAML or CSON due to its strict syntax. YAML is more readable than JSON, but its reliance on indentation can make it more difficult to work with in some cases. CSON offers many of the same benefits as YAML, but its use of Compact Symbol syntax may make it less accessible to developers who are not familiar with that language.

Ultimately, the choice between these formats will depend on your specific needs and preferences. JSON is a good choice for web applications or other situations where interoperability is important. YAML or CSON may be better suited for configuration files or other situations where readability is a priority. But for this tool CSON is the best due to its compact size and no overehead while typing in terminal whereas in other formats escape characters were required.

### Working logic

The `process_file` function takes three arguments: `file_path`, `substitute`, and `pattern`. It checks if the file at the given `file_path` exists and then reads its contents. It then tries to parse the `substitute` string as JSON, YAML or CSON and calls the appropriate `search_and_replace_with_map` function to replace the contents of the file. If the `substitute` string cannot be parsed as any of these formats, it falls back to calling the `search_and_replace` function with the `substitute` string as-is.

## Build

To build the `FindReplace` CLI app, follow these steps:

1. Clone the `FindReplace` repository from GitHub by running the following command:

```
git clone https://github.com/isurfer21/FindReplace.git
```

2. Navigate to the cloned repository by running the following command:

```
cd FindReplace
```

3. Build the project using Cargo by running the following command:

```
cargo build
```

After running these commands, you should have a local copy of the `FindReplace` project that is ready to use.

The executable binary can be found in the `.\target\debug\` directory. On Windows, the binary is named `findreplace.exe`, while on macOS it is simply named `findreplace`.

## Publish

To publish the `FindReplace` crate to [crates.io](https://crates.io/), run the following command:

```
cargo publish
```

This will upload the crate to [crates.io](https://crates.io/) so that others can easily download and use it.