🚀 RIJSON -> My JSON Lexer & Parser: A Learning Project
===
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

## 👋 What's This About?

This is my personal project to dive deep into how compilers and interpreters work, by building a lexer (lexical analyzer) and a parser (syntactic analyzer) specifically for JSON. My main goal is to take a JSON text input and transform it into a structured data representation that a program can easily understand and use.

## ✨ Features
This project focuses on building the core components for processing JSON data. Here's what it can do:

### JSON Lexer (Lexical Analyzer)
The lexer is the first step, breaking down raw JSON text into meaningful pieces called tokens.

- Tokenization: Converts the input JSON string into a stream of tokens, strictly following the official JSON Specification (RFC 8259).

- Literal Recognition: Accurately identifies all standard JSON literal types:

- Strings: Handles quoted text, including various escape sequences (\n, \", \uXXXX, etc.).

- Numbers: Recognizes integers, decimal numbers, and numbers with exponents (e.g., 1, 3.14, 2e-10).

- Booleans: Detects true and false.

- Null: Identifies the null value.

- Structural Character Identification: Pinpoints all the essential punctuation that defines JSON's structure: {, }, [, ], ,, and :.

- Whitespace Handling: Correctly processes and discards non-significant whitespace characters (spaces, tabs, newlines, carriage returns).

- TODO -> Error Reporting: Designed to catch and signal lexical errors, such as unrecognized characters or unclosed strings.

### JSON Parser (Syntactic Analyzer)
The parser takes the tokens from the lexer and builds a structured representation of the JSON data.

- Syntax Validation: Consumes the token stream from the lexer and verifies that the JSON input adheres to the correct grammatical rules.

- Data Structure Construction: Generates a hierarchical data structure in memory (like a nested enum in Rust) that directly mirrors the objects, arrays, and values within the JSON document. This allows easy programmatic access to the parsed data.

- Syntax Error Reporting: Provides clear messages for syntax errors, such as missing commas, misplaced brackets, or unexpected tokens, helping to pinpoint issues in the JSON structure.
