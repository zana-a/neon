# Documentation

Hi! Welcome to Neon's (humble) documentation. This will eventually be replaced by a better solution, but there are,
as they say, bigger fish to fry 🐟!

> > Please note this is extremely incomplete.

## Table Of Contents

I have seperated out the language's building blocks into the following sections:

- [Overview](#overview)
- [Parsing](#parsing)
- [Translating](#translating)
- [Commandline Interface (CLI)](#commandline-interface-cli)
- [REPL (Read-Eval-Print Loop) Program](#repl-read-eval-print-loop-program)

## Overview

Before we get into the nitty-gritty detail, lets look at what Neon will look like.

Firstly, Neon inherits a lot of its ideas from another cool _functional_ language called Elixir. Interestingly enough,
Neon is also a functional programming language. The difference here is that we use JavaScript as our end result rather
than the BEAM.

While I will not discuss the reasons here for why we need yet another programming language, I will say that it could
potentially be a useful tool when it comes to correctness of programs by virtue of being a functional language.

Secondly, Neon will provide a package manager and a CLI out of the box. This should help make life easier for the
developer. Rust, does this in a very neat way — no wonder it is one of the most loved languages.

Finally, here are some of the language concepts themselves. More detail and other concepts will most likely be added in
the future.

- **Modules**: this is a container box that can hold constants, functions and other modules if need be. This is mostly
  for
  organising code. Since this is also scoped, it should also help with naming things.

```
module Hello do
  # Code goes here
end
```

- **Functions**: reusable snippets of code. I think every programmer will know what this is.

```
module Hello do
  pub def world() do
    # code goes here
  end

  pub def is_nice?() do
    # code goes here
  end
  
  pub def here_be_errors!() do
    # code goes here
  end
  
  pub def is_true?(x) do
    # code goes here
  end
  
  pub def add(x, y) do
    # code goes here
  end
  
  def a_private_function() do
    # code goes here
  end
end

Hello::world()
Hello::is_nice?()
Hello::here_be_errors!()
Hello::is_true?(false)
Hello::add(2, 3)
```

- **If-else Conditional**: we can use traditional if-else conditional logic

```
if true do
  # code goes here
end

if true do
  # code goes here
else 
  # code goes here
end

if true do
  # code goes here
else if false or true
  # code goes here
else
  # code goes here
end

if true do
  # code goes here
else if false or true
  # code goes here
else if false and true
  # code goes here
else if false and not true
  # code goes here
else
  # code goes here
end
```

- **When Conditional**: As you can see that the above is really hard to read when it comes to if and else if branches.
  In some languages like Elixir, Rust and Kotlin (and to some degree even Java) has a syntactical sugar that usually
  helps with cleaning up some of the branches.

```
when some_expression() do
  case true 
end
```

## Parsing

To get a rough idea how the language works in terms of parsing, I have created a set of parsing documents using the
EBNF notation. If you don't know what this is, please see the wikipedia page
[here](https://en.wikipedia.org/wiki/Extended_Backus–Naur_form).

For reference, this is the notation used.

| Usage            | Notation  |
|------------------|-----------|
| definition       | =         |
| concatenation    | ,         |
| termination      | ;         |
| alternation      | \|        |
| optional         | [ ... ]   |
| repetition       | { ... }   |
| grouping         | ( ... )   |
| terminal string  | " ... "   |
| terminal string  | ' ... '   |
| comment          | (* ... *) |
| special sequence | ? ... ?   |
| exception        | -         |

### Basic blocks

- [Keywords](#keywords)
- [Identifier](#identifier)

#### Keywords

Keywords in Neon are reserved words which the compiler uses to determine the type of component to be used. I have listed
below a set of keywords found in the language.

- module
- do
- end
- pub
- def
- if
- else
- true
- false
- and
- or
- not

#### Identifier

### Generic

- Expression
- Statement

## Translating

_Todo!_

## Commandline Interface (CLI)

_Todo!_

## REPL (Read-Eval-Print Loop) Program

_Todo!_
