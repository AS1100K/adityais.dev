---
layout: "@layout/blog.astro"
title: Introducing Release Butler
description: "Introducing Release Butler - A GitHub App that automates the process of creating pull requests for version bumps and changelogs based on issues with a specific label. When the PR is merged, it can create a tag and github release (optional)."
---

# What is Release Butler?

Release Butler is a GitHub App that automates the creation of pull requests for version bumps and
changelogs based on issues with a specific label. Once the pull request is merged, it can also create
a tag and GitHub release, saving you time and effort.

> Currently, the app is in the process of listing on GitHub Marketplace. This page will be updated once it's listed.

<iframe width="560" height="315" src="https://www.youtube.com/embed/gJtMNcaxnDw?si=vVOAAcq_FXV0oGio" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## Motivation

When your code matures, it's common to restrict direct pushes to the main branch and instead use
pull requests. In this scenario, creating a CHANGELOG file, bumping the version, and creating
GitHub tags and releases can become tedious. This is where Release Butler comes into action. It
automates these repetitive tasks, ensuring consistency and saving you valuable time.

## Key Features

- **Automated Pull Requests**: The App listens to issues labeled `release-butler` and create pull
  requests with the specified version bump and changelog.
- **Tag and Release Creation**: Optionally creates a tag and GitHub release upon merging the pull
  request.
- **No Data Storage**: Process information directly from your GitHub repository without storing
  any data.

## How it Works

1. **Install the App**: Add [Release Butler](https://github.com/apps/release-butler) to your GitHub repository.
2. **Configuration**: Create a `.github/release-butler.toml` file to configure the app.
   [See Template](https://github.com/rs-workspace/release-butler/blob/main/repository.template.toml)
3. **Create an Issue**: Label an issue with `release-butler` and specify the version in the title and changelog in the body.
4. **Automatic PR Creation**: Release Butler will generate a pull request with the version bump and changelog.
5. **Merge and Release**: Merge the pull request to apply the changes and optionally create a tag and GitHub release.

## Benefits

- **Efficiency**: Automates repetitive tasks, allowing you to focus on development.
- **Consistency**: Ensures that version bumps and changelogs are handled consistently.
- **Security**: Hosted on [shuttle.rs](https://shuttle.rs), providing secure and reliable hosting.

## Getting Started

To get started with Release Butler, follow the detailed instructions in our [README](https://github.com/rs-workspace/release-butler/blob/main/README.md).

## Conclusion

We hope Release Butler will become an invaluable tool in your development workflow. Try it out today and experience the benefits of automated release management!

For more information, visit our [GitHub repository](https://github.com/rs-workspace/release-butler) or watch our [demo video](https://www.youtube.com/watch?v=gJtMNcaxnDw).

Happy releasing!
