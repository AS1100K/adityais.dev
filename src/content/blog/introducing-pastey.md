---
layout: "@layout/blog.astro"
title: Introducing Pastey - Successor of Paste
description: "Introducing Pastey - A drop-in replacement for paste crate"
pubDate: 2025-03-10
---

# Introducing `pastey`

[Reddit Post](https://www.reddit.com/r/rust/comments/1j7fnsi/introducing_pastey_successor_of_paste/)
[GitHub Repo](https://github.com/as1100k/pastey)

I am excited to introduce `pastey`, a drop-in replacement for the now-archived paste crate.
Recently, the paste crate was archived i.e. it will not be maintained anymore. And since,
there were no good alternative available for paste, I felt I need to step in and fork the
repository and further maintain it.

However, I do not merely intend to fork the repository; I aim to demonstrate my commitment
to improving the code by fixing existing bugs and issues before releasing `pastey`.

## A Drop-in Replacement

The main goal of `pastey` is to maintain the same behavior as the original paste crate, ensuring
it functions as a drop-in replacement.

Since, paste is a perfectly built there are not going to be any internal design changes
_(at least for now)_. Therefore, for new behaviour additional modifiers are created for some other
uses case. More on that at [Additional Features Section](#additional-features).

I have built a special test suite named [paste-compat](https://github.com/AS1100K/pastey/tree/master/paste-compat)
that ensures the behaviour of `pastey` is similar to paste crate for common modifiers and features.

_Currently, more tests are welcomed for paste-compat._

### Migrating from `paste` crate

It is very easy to migrate from `paste` crate to `pastey`, follow either of the following steps:

```diff
[dependencies]
- paste = "1"
+ pastey = "*" # Or replace with the latest version of pastey
```

Or even better:

```diff
[dependencies]
- paste = "1"
+ paste = { package = "pastey", version = "*" }  # Or replace with the latest version of pastey
```

_You might go with this step, if you don't want to rename every **`paste::`** with **`pastey::`**_

## Additional Features

The `pastey` crate also comes with additional features:

- Raw Identifier Generation: The `pastey` crate supports a raw mode in `paste!` to generate raw
  identifiers, like `loop`. The following example shows how it's used:

  ```rust
  use pastey::paste;

  macro_rules! define_struct_and_impl {
      ($name:ident $(- $name_tail:ident)*) => {
          paste!{
              struct [< # $name:camel $( $name_tail)* >]; // '#' signals a raw identifier

              impl [< # $name:camel $( $name_tail)* >] {
                  fn [< # $name:snake $( _ $name_tail:snake)* >]() {}
              }

          }
      }
  }

  define_struct_and_impl!(loop);
  define_struct_and_impl!(loop - xyz);

  fn test_fn() {
      let _ = Loop::r#loop();
      let _ = Loopxyz::loop_xyz();
  }
  ```

- `camel_edge`: The camel_edge modifier addresses an edge case in `camel` case conversion. For
  example, while the existing camel modifier converts both `my__ident` and `_my__ident` to `MyIdent`,
  this behavior can be problematic if they are meant to be distinct. With `camel_edge`

  1. `my__ident` is converted to `My_Ident`
  2. `_my__ident` is converted to  `_My_Ident`

  [More Info](https://github.com/AS1100K/pastey/issues/3)
- `lower_camel`: The `lower_camel` modifier is a variation of `camel` case conversion that makes
  the first letter lowercase instead of uppercase.

  [More Info](https://github.com/AS1100K/pastey/issues/4)

I know that these modifiers break the rust naming convention, but can be useful for some internal crates
dealing with FFI or other use cases.
