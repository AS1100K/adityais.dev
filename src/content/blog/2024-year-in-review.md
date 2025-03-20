---
layout: "@layout/blog.astro"
title: 2024 Year in Review
description: "A reflection on 2024: My journey of growth, learning, and achievements. From open source contributions and creating Rust projects to building a portfolio and exploring new technologies like Gerrit and Bevy-Discord integration."
---

# Year in Review: What 2024 Taught Me as a Developer

As 2024 comes to a close, I find myself reflecting on a year that has been truly
transformative. This year was filled with milestones, new opportunities, and
countless lessons that have shaped my journey as a developer and a learner. Here’s
a glimpse into what I accomplished, learned, and aspired to achieve in 2024.

## A Major Milestone: Ambala College of Engineering

This year began with a significant step in my academic journey—getting into Ambala
College of Engineering. It marked the culmination of my high school years and the
beginning of a new chapter filled with opportunities to grow as an engineer and
innovator.

## Diving Into Open Source Contributions

One of my biggest focuses this year was contributing to open source projects. Here
are some highlights:

### Wikimedia

While I wasn’t actively involved in the MediaWiki organization this year, I am
planning to be more engaged in 2025 as I deeply admire what Wikimedia does for
society—free education for everyone! Along the way, I also learned how to use
Gerrit and managed to get 4 PRs closed across multiple VCS platforms, including
Gerrit and the Wikisource project on GitHub.

### Azalea-rs

With six pull requests merged, I made significant contributions to [azalea-rs](https://github.com/azalea-rs/azalea/pulls?q=is%3Apr+author%3AAS1100K+),
a Rust library for creating Minecraft Bots, improving its functionality and
learning a lot in the process.

### Rust

This year also marked my first contribution to the Rust programming language.
While [the PR](https://github.com/rust-lang/rust/pull/134880) is still awaiting
merging due to holiday delays, it was a milestone in my journey with Rust. The
experience taught me about the rigor and community involvement required in
contributing to such a widely used language.

> The PR has been merged 🎉

## Bevy-Discord: Bridging Discord with Bevy

I created and open-sourced a Rust crate called [**bevy-discord**](https://github.com/as1100k/bevy-discord),
which connects the Discord API with the game engine Bevy. Solving the challenges
of integrating these two platforms was a fulfilling experience. While the crate is
already functional, I’m planning to add a [rich presence](https://discord.com/developers/docs/rich-presence/overview)
feature to make it even more versatile and useful for game developers.

## Initiating rs-workspace: A Successor to actions-rs

Inspired by GitHub’s [**actions-rs**](https://github.com/actions-rs) project, I
initiated [**rs-workspace**](https://github.com/rs-workspace), aiming to provide
enhanced Rust workflow automation tools. So far, I’ve built one action,
[rust-toolchain](https://github.com/rs-workspace/rust-toolchain), and I’m excited
to expand this project further in the coming months.

## Cargo-Wiki: A WIP Rust Documentation Generator

I began working on [**cargo-wiki**](https://github.com/as1100k/cargo-wiki), a WIP
tool designed to generate Rust documentation in markdown format. My goal is to
make the output exceptionally human-readable, avoiding HTML tags wherever
possible. This project led to my first Rust contribution when I identified a
documentation error in [rustdoc-types](https://github.com/rust-lang/rustdoc-types/),
but it was a error in [librustdoc](https://github.com/rust-lang/rust/tree/master/src/librustdoc)
which [my PR](https://github.com/rust-lang/rust/issues/134853) attempts to fix.
The process of resolving this issue was immensely rewarding and deepened my
understanding of Rust’s internals.

## Building a Portfolio and Blog

I created my personal portfolio and blog using Astro, employing some unconventional methods to keep the project scalable and organized:

- **Git Submodules for Content Management**: Instead of storing blog content
  directly in the repo, I used a separate content repository linked as a git
  submodule. Symbolic links for directories made the setup seamless, ensuring my
  website repo remains fork-friendly.

  [Commit `0a1bd1` to `76ab37`](https://github.com/as1100k/adityais.dev/compare/0a1bd1..76ab37)

- **Hosting Journey**: Initially, I hosted the website on Netlify, leveraging
  their dynamic functions for up-to-date GitHub repo cards. Later, I migrated to
  Cloudflare Pages for better infrastructure, and finally settled on GitHub Pages.
  By automating builds and submodule updates with GitHub Actions, I created an
  efficient, maintenance-free deployment process.

  [GitHub Pages Migration Commit](https://github.com/AS1100K/adityais.dev/commit/0a1bd18fea0e91ee9efa19fe5ac37ed2bc1f4871),
  [Cloudflare Pages Migration PR #1](https://github.com/AS1100K/adityais.dev/pull/1),
  [Original Netlify Commit](https://github.com/AS1100K/adityais.dev/commit/fc93ad83e9f13c5366379ce0ee64d4060aa795b3)

## Challenges, Lessons, and the Road Ahead

This year wasn’t without its challenges. Balancing multiple projects, managing
time, and diving into new technologies pushed me out of my comfort zone. However,
these challenges also provided invaluable lessons and skills that I’ll carry
forward into the new year.

As I look to 2025, I’m excited to continue working on my existing projects,
contributing to open source, and exploring new opportunities for growth. Here’s to
another year of innovation and learning!
