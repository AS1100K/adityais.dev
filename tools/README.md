# Repository Tools

This directory contains internal tools used by this website to update it's
content. The tools are written in `Rust` language even though most of the
tools are just calling API(s) and serializing them to json which can be
performed by any language like `Python`, `JavaScript`, etc.

## Motive for `Rust`

Since I am very fimiliar and comfortable with rust, I am extreamly productive
in rust. Building these same tools in python would take me little more time as
I am highly comfortable with rust. Another reason of choosing a strongly
typed language like Rust rather than dynamically typed language is that the
type system boosts productivity, and debugging as you know what types you
are dealing with before hand rather than at runtime. Also, I choosed rust
over C, because rust provides High Level Approach while not compromising
Low Level Access _(this is not a major reason, just want to throw this in)_.

## Tools Available

1. [`Contributing`](./contributing/): Gathers data for [adityais.dev/contributing](https://adityais.dev/contributing).
2. [`Stargazer`](./stargazer/): Gathers number of stars of a repository.
