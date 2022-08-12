# 🤯 Brainfuck

A [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter, written as an opportunity to practice [TDD](https://en.wikipedia.org/wiki/Test-driven_development).

## How it works

1. The source code goes through a *lexer* that produces a series of tokens
2. These tokens are *parsed* into an abstract syntax tree (AST)
3. The AST is visited (depth-first) and interpreted by the runtime

While this seems overkill, the goal of this project is to familiarize with compiler concepts, hence the apprent complexity for such a simple problem.

## Todo

- Refactor to a [LR parser](https://en.wikipedia.org/wiki/LR_parser) based on a defined grammar
- Optimization ([constant folding](https://en.wikipedia.org/wiki/Constant_folding), [run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding), replace idioms...)
- Add visitor support to transpile code
- Better syntax error handling
- Comment support

## BNF

Brainfuck's [Backus-Naur form](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form) could be:

- \<prg>     ::= \<expr> | \<expr> \<prg>
- \<expr>    ::= `+` | `-` | `<` | `>` | `.` | `,` | \<loop>
- \<loop>    ::= `[` + \<expr> + `]`
