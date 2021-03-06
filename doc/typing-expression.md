% Typing Expression

A key idea behind Oak is to give a type to parsing expression. For example, we expect `e1 e2` to have the type `(T1, T2)` if `e1` has type `T1` and `e2` has type `T2`. Indeed, it exists an obvious mapping between PEG combinators and traditional types found in programming language: choice is a sum type, sequence is a tuple, repetition is an array, etc. Oak was born to explore this mapping and to answer a question: Can we automatically generate an AST from a grammar description?

It turned out that generating the AST (data type included) was hard, mostly because we need to _name_ types and that rules does not give enough information by themselves – how to name the variants of the sum type? Of course, we could annotate expressions with names but Oak is designed to describe a grammar in the cleanest way as possible in the first place, so this is not a desirable solution. Also, the user will certainly want to use its own custom types and not arbitrary generated types, so a fully automatic generation is not such a good idea. Therefore, Oak relies on the return types of semantic actions to have a complete type inference scheme. That is, the user implicitly brings additional type information to Oak through semantic actions. This technique has at least two advantages over conventional methods:

* A closer mapping between grammar and user-code. For example `number "+" number > add` is a valid expression where `add` is a semantic action called with two arguments of the type of `number`.
* Types are used to generate more efficient code. Indeed, a value is only built if it is useful somewhere. For example the generated code of `!number` will only recognize the expression `number` but semantic actions inside `number` will not be called.

This chapter explains how Oak gives type to expression and how you can efficiently control and use it.

### Type annotation

Despite the apparent simplicity of this idea, a direct mapping between expression and type is not very useful. Consider the following grammar for parsing variable identifier.

```
var_ident = !["0-9"] ["a-zA-Z0-9_"]+ spacing
spacing = [" \n\t"]*
```

A straightforward mapping would give to this expression the type `(char, Vec<char>, Vec<char>)` since the sequence has three sub expressions and thus forms a 3-tuple. Clearly, the only value of interest in `var_ident` is the one returned by the expression `["a-zA-Z0-9_"]+` which has type `Vec<char>` (note that we could use a semantic action to transform this value into a string). It is natural to think that the rule `var_ident` will be of type `Vec<char>` too. Oak infers this type if we tell him that we do not care about the value of spaces which is not something that it can guess by itself. We use the combinator `e -> ()` to indicate to Oak that spacing is of type unit.

```
var_ident = !["0-9"] ["a-zA-Z0-9_"]+ spacing
spacing = [" \n\t"]* -> ()
```

The new type of `var_ident` is now `(char, Vec<char>, ())`. The inference algorithm automatically reduces this type to `Vec<char>` thanks to a few simplification rules:

* Everything under a syntactic predicate (`!e` or `?e`) has type `()`. The new type is `((), Vec<char>, ())`.
* Any unit type inside a tuple is removed. We now have `(Vec<char>)`.
* Type inside a 1-tuple is extracted. We finally obtain `Vec<char>`.

These _type rewriting rules_ are intuitive because they produce the type the user expects! Type annotation is only needed to specify that we are not interested by the value, such as with spaces.

### Unit propagation

A type containing a unit type is simplified if it does not erase a piece of information. If we consider the following grammar which describe the optional presence of the `mut` keyword on the left-hand side of a let-expression, the `mut_kw?` type is not rewritten into `()`.

```
let_left = let_kw mut_kw? var_ident
let_kw = "let" spacing
mut_kw = "mut" spacing
```

Indeed, the type of the expression `mut_kw?` is `Option<()>`. This is expected since the type `Option<()>` carries a boolean information. As a rule of thumb, unit inference never erase a potential piece of information. In some cases, expression are only of a pure syntactic interest such as the first optional `|` in OCaml pattern-matching. This is why we use the "invisible type" annotation `e -> (^)` to indicate that the unit type must be propagated up since it does not carry any relevant semantic information.

```
match_expr = match_kw expr with_kw bar? cases
cases = case (bar case)*
bar = "|" spacing -> (^)
```

In `match_expr`, the expression `bar?` would have type `Option<()>` if we did not annotate `bar` with `(^)`. Of course, this annotation is not strictly necessary and we could use `(bar? -> ())` to emulate the same effect. However, this is an extra-burden on the grammar so `(^)` helps to keep it more readable. The circumflex symbol in `(^)` indicates a bottom up propagation of unit in expression. The propagation is only stopped if it is composed with a value of a relevant type. For example, the expression `bar? expr` has type `Expr` because `(^)` has been propagated across `Option<(^)>` and `((^), Expr)` types.

### Recursive type

We must distinguish recursive rules that are totally valid in Oak and recursive types that can not be automatically inferred. For example, the following grammar accepts strings in which any character at position `i` is 'a' or 'b' if `i` is even and is otherwise 'c' or 'd'.

```
a_b = ["a-b"] c_d?
c_d = ["c-d"] a_b?
```

This is a totally valid grammar but it can not be typed without recursive types. Let's try anyway. Say that `a_b` has type `T` and `c_d` has type `U`. We can infer that `T = (char, Option<U>)` for rule `a_b` and `U = (char, Option<T>)` for rule `c_d`. By substitution we get `T = (char, Option<(char, Option<T>))` and thus `T` is defined by itself. It is a recursive type definition. You might think that this type is fine as long as we give an alias to the tuple types:

```rust
type T = (char, Option<U>);
type U = (char, Option<T>);
```

However the names `T` and `U` are completely arbitrary and the user probably do not want types with random names. We would need name-annotations on expressions which is not our leitmotiv in the first place. It is cleaner and easier to let the user constructs the types by himself with semantic actions. Furthermore, here the type `Option<U>` introduces the necessary indirection for building recursive types but what if the grammar does not explicitly give one? We should automatically infer a type supporting recursive type which seems a lot of work for little benefit. In the end, we forbid implicit recursive types and let the user write semantic actions to build them. However recursive rules are still allowed. The difficulty was to generate a compile-time error only in the case we care about the value produced by the recursive rules. Actually, the previous grammar will be accepted by the parser generator if we are using the rule `a_b` below a syntactic predicate, for example the expression `!a_b` implies that we do not need to build the value of `a_b`.
