# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Boxchar is a Rust word game application that implements a letter-based puzzle where players form words using letters from four sides of a box. The game validates letter combinations (digraphs) that can be formed between different sides.

## Commands

### Build and Run
- `cargo build` - Build the project
- `cargo run` - Run the main application (loads and displays game from data/game.txt)
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run a specific test

### Development
- `cargo check` - Quick syntax check without building
- `cargo clippy` - Run linting
- `cargo fmt` - Format code

## Architecture

### Core Components

**Game Structure (`src/game.rs`)**
- `Game` struct manages the four-sided letter box and valid digraph combinations
- Validates game files must have exactly 4 sides with equal lengths
- Ensures no duplicate letters across all sides and only uppercase ASCII letters
- Computes all valid digraphs (letter pairs from different sides)

**Library Structure (`src/lib.rs`)**
- Exports `game` and `build_digraph_list` modules
- Main application (`src/main.rs`) uses the library to load and display games

**Data Processing (`src/build_digraph_list.rs`)**
- Handles wordlist file reading and processing
- Functions for extracting digraphs from word lists

### Key Validation Rules
- Game files must contain exactly 4 lines (sides)
- All sides must have the same length
- Only uppercase ASCII letters allowed
- No duplicate letters across any sides
- Valid digraphs can only be formed between letters from different sides

### Test Structure
- Tests use `tempfile` crate for isolated file testing
- Comprehensive validation testing in `tests/game_validation.rs`
- Tests cover structural validation, content validation, and digraph generation

### Data Files
- `data/game.txt` - Main game configuration (4 sides of letters)
- `data/wordlist.txt` - Word dictionary for validation
- `data/wordlist_test.txt` - Test word list
- `data/answers.txt` - Valid answers/solutions