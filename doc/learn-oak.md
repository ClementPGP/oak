% Learn Oak

This section is devoted to introduce smoothly the different PEG combinators through a tutorial presenting `Calc`: a small language with arithmetic expressions and variable bindings. If you want to test the code while reading this tutorial, a skeleton project is available in the section [Getting Started](getting-started.md). Before diving into the details, we present a program written in `Calc`:

```
let a = -10 - 2 in
let b = a / 2 in
a + 3 * (b / 2)
```

It declares two local variables `a` and `b` initialized with arithmetic expressions usable within the scope of the let-binding, which is everything after the `in`. Let-bindings can be composed in cascade but must terminates with an arithmetic expression, such as `a + 3 * (b / 2)` in our example.

### What is parsing?

Traditionally, a parser is a bridge between meaning-less sequence of characters and structured representation of data. It tries to give meanings to raw characters by constructing an *Abstract Syntax Tree* (AST) that will be processed by subsequent compilation phases. We expect a parser to transform `7 - 1` into a structure such as `Minus(i32, i32)`. As a side note, you should avoid to compute the actual result of `7 - 1` in the parsing step, it works for simple language but tends to entangle syntactic and semantic analysis later. Invalid programs such as `let a = 8 in a * b` will still be correctly parsed, the semantic analysis is responsible for detecting that `b` is undeclared.

This tutorial will not cover the semantic analysis part and will only describe the grammar used for parsing `Calc`. Our parser will thus produce an AST and will not evaluate expressions.

### Syntactic atoms of `Calc`

When it comes to elaborate a grammar, we usually start by identifying atoms of the language, e.g. syntactic constructions that can not be divided into smaller ones. These atoms are called *tokens* and are often processed during a *lexical analysis* happening before the parsing. Oak is based on _Parsing Expression Grammar_ (PEG) and works directly on a stream of characters instead of a stream of tokens. An advantage is to have a unique and coherent grammar syntax which is helpful for composing grammars that do not necessarily expect the same set of tokens. Before continuing reading, try to find out what are the atoms of `Calc`.

The keywords `let` and `in`, the binding operator `=`, parenthesis `()` and arithmetic operators `+`, `-`, `*`, `/` form the *unvalued atoms* of the language. `Calc` has two *valued atoms* which are identifiers and integers. Unvalued atoms give a shape to the AST but they do not carry any specific data inserted by the user. Let write our first grammar to parse the unvalued atoms:

```
grammar! calc {
  let_kw = "let"
  in_kw = "in"
  bind_op = "="
  add_op = "+"
  sub_op = "-"
  mul_op = "*"
  div_op = "/"
}
```

A grammar is a set of rules of the form `<name> = <expr>` where `<name>` is the rule name and `<expr>` is a parsing expression. We use *string literals* of the form `"<literal>"` if we expect the input to match exactly the sequence of characters given.

The grammar is empowered with identifiers and integers this way:

```
grammar! calc {
  identifier = ["a-zA-Z0-9_"]+
  integer = ["0-9"]+

  let_kw = "let"
  in_kw = "in"
  bind_op = "="
  add_op = "+"
  sub_op = "-"
  mul_op = "*"
  div_op = "/"
}
```

Oak will generate two functions per rule, a parser and a recognizer. For example, the functions `parse_identifier` and `recognize_identifier` will be generated for rule `identifier`.

The `#![show_api]` attribute tells Oak to output, as a compilation note, the signatures of all the generated functions:

```rust
note: pub mod calc {
    pub fn parse_let_kw(input: &str, pos: usize)
     -> oak_runtime::ParseResult<()>;
    pub fn recognize_let_kw(input: &str, pos: usize)
     -> oak_runtime::ParseResult<()>;

    pub fn recognize_identifier(input: &str, pos: usize)
     -> oak_runtime::ParseResult<()>;
    pub fn parse_identifier(input: &str, pos: usize)
     -> oak_runtime::ParseResult<Vec<char>>;
    pub fn parse_integer(input: &str, pos: usize)
     -> oak_runtime::ParseResult<Vec<char>>;

  // ...
  // Rest of the output truncated for the tutorial.
}
```

We can already use these functions in our main:

```
fn main() {
  assert!(calc::parse_let_kw("let", 0).is_ok());
}
```

The difference between `parse_*` and `recognize_*`

### Exercise

Extend the grammar to support `let-in` anywhere in expressions. Note that you do not need to modify the AST.
