# Boxchar

A solver for the New York Times word puzzle, "Letter Boxed". 

Letter Boxed puzzles are a set of letters in a box shape. Players must connect all the letters in the puzzle 
with a chain of valid words, as in the following screenshot.

![Screenshot of NYTimes Letter Boxed](NY_Times_Letter_Boxed.png)

## Game Rules

1. **Four-sided puzzle**: Letters are arranged on four sides of a square. (Though, boxchar may allow other shapes)
2. **No same-side connections**: You cannot connect two letters from the same side. Think of it as bouncing between sides.
3. **Word chaining**: Each new word must start with the last letter of the previous word
4. **Complete coverage**: All letters must be used across your word sequence

There is no score in the New York Times' version of Letter Boxed, but some puzzles are harder than 
others. Sometimes there are hundreds of solutions, and sometimes there is only one. Skilled players try 
to complete the puzzle in fewer words.

### Example

Given the puzzle:
```
JGH NVY EID ORP
```

A possible two-word solution is: `HYPERDRIVE-ENJOINING`

A possible three-word solution is: `DOVE-ENJOYING-GRYPHON`


Note how each letter hops to a different side, and the words are connected by their first/last letters.

## Installation

Make sure you have Rust installed, then:

```bash
git clone <repository-url>
cd boxchar
cargo build --release
```

## Usage

### Basic Command Structure

```bash
cargo run -- [OPTIONS] [GAME_SPEC]
```

### Specifying the Game

You can specify the game in two ways:

#### 1. Positional Argument (Comma-separated)
```bash
# Specify game directly as comma-separated sides
cargo run -- "YFA,OTK,LGW,RNI"
```

Requirements:
- Only letters (A-Z, a-z) and commas allowed
- No spaces permitted
- Letters are automatically converted to uppercase
- Must have exactly 4 sides with equal lengths

#### 2. File Path (--game option)
```bash
# Load game from a file
cargo run -- --game data/game.txt
```

The game file should contain 4 lines, each representing one side:
```
YFA
OTK
LGW
RNI
```

### Command Line Options

| Option | Description | Default | Required |
|--------|-------------|---------|----------|
| `GAME_SPEC` | Game as comma-separated sides (e.g., "ABC,DEF,GHI,JKL") | - | Either this or `--game` |
| `--game <PATH>` | Path to game file | - | Either this or `GAME_SPEC` |
| `--wordlist <PATH>` | Path to wordlist file | `data/wordlist.txt` | No |
| `--help` | Show help information | - | No |

### Examples

```bash
# Using positional game specification
cargo run -- yfa,otk,lgw,rni

# Using game file with custom wordlist
cargo run -- --game data/game.txt --wordlist data/custom_wordlist.txt

# Get help
cargo run -- --help
```

### Error Cases

The application will exit with an error if:
- Neither game specification nor `--game` option is provided
- Both game specification and `--game` option are provided
- Game specification contains invalid characters (anything other than A-Z, a-z, comma)
- Game file cannot be read or has invalid format
- Wordlist file cannot be read

## Development Commands

```bash
# Build the project
cargo build

# Run with development profile
cargo run

# Run tests
cargo test

# Run specific test
cargo test <test_name>

# Quick syntax check
cargo check

# Run linting
cargo clippy

# Format code
cargo fmt
```

## Game File Format

Game files must follow these rules:
- Exactly 4 lines (representing the 4 sides)
- All sides must have the same length
- Only uppercase ASCII letters (A-Z)
- No duplicate letters across all sides

Example valid game file:
```
ABC
DEF
GHI
JKL
```

## Wordlist Format

Wordlist files should contain one word per line, with words in uppercase. The solver will:
1. Filter words that can be formed with the given letters
2. Ensure no letter repetitions (since all letters in the puzzle are unique)
3. Validate that consecutive letters come from different sides


## License

Copyright Neil Kandalgaonkar, 2025. 

This software is *NOT* freely redistributable.

Screenshot by The New York Times - The New York Times Games mobile app, sourced from [Wikipedia's File:NY Times Letter Boxed.png](https://en.wikipedia.org/w/index.php?curid=76415365).
