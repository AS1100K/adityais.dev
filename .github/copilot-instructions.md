# Aditya Kumar's Personal Portfolio Website

**ALWAYS follow these instructions first and only fallback to additional search and context gathering if the information in these instructions is incomplete or found to be in error.**

This is an Astro-based personal portfolio website (adityais.dev) that includes a Rust-based tools workspace for generating dynamic content. The website is deployed via GitHub Pages and includes blog content, project showcases, and open source contribution tracking.

## Working Effectively

### Initial Setup and Dependencies

Bootstrap the repository with these exact commands:

```bash
# Node.js is required (tested with v20.19.4+)
npm install  # Takes ~48 seconds. NEVER CANCEL. Set timeout to 90+ seconds.
```

### Building the Application

```bash
# CRITICAL: Always generate required JSON files before building
# Create dummy JSON files if you don't have GitHub API access:
echo "[]" > prs.json
echo "{}" > repo-info.json

# Build the application
npm run build  # Takes ~9-10 seconds. NEVER CANCEL. Set timeout to 30+ seconds.
```

### Running the Application

```bash
# Development server (for active development)
npm run dev  # Starts server on http://localhost:4321/

# Preview server (for testing build output)
npm run preview  # Starts server on http://localhost:4321/
```

### Rust Tools Workspace

The `tools/` directory contains Rust utilities for content generation:

```bash
cd tools

# Build tools (first time only)
cargo build  # Takes ~1m51s initially. NEVER CANCEL. Set timeout to 180+ seconds.

# Run tests
cargo test  # Takes ~3 seconds. NEVER CANCEL. Set timeout to 30+ seconds.

# Generate content (requires GitHub API token)
export TOOLS_CONTRIBUTING_TOKEN="your_github_token"
cargo run -p tool-contributing  # Generates prs.json and repo-info.json
```

### Code Quality and Validation

**ALWAYS run these commands before committing changes:**

```bash
# Format code (exclude build artifacts)
npx prettier --write . --ignore-path .gitignore

# Check formatting (exclude build artifacts)
npx prettier --check . --ignore-path .gitignore  # Must pass for CI

# Rust linting and formatting
cd tools
cargo fmt --all -- --check  # Takes <1 second
cargo clippy --workspace --all-features -- -Dwarnings  # Takes ~8s initially, <1s after
```

## Validation

### Manual Testing Requirements

**ALWAYS manually validate changes with these complete scenarios:**

1. **Build and Development Server Test:**

   ```bash
   npm run build && npm run dev
   ```

   - Navigate to http://localhost:4321/
   - Verify homepage loads with portfolio content
   - Test navigation: Home, About, Skills, Projects, Contributions, Blog
   - Check that all pages load without errors

2. **Core User Journeys:**

   - **Portfolio Browsing**: Navigate through all sections, verify project links work
   - **Blog Reading**: Visit /blog page, verify blog listing (may show "No blogs found" if content collection has issues)
   - **Contributions Page**: Visit /contributing page, verify it loads (may be empty without API data)

3. **Build Output Validation:**
   ```bash
   npm run build && npm run preview
   ```
   - Test the static build works correctly
   - Verify all pages render properly in preview mode

### Known Working State

- The application builds successfully with dummy JSON files
- Development server runs on port 4321
- All pages are navigable and render correctly
- Some API calls may fail in restricted environments (shows 403 errors) but build still succeeds
- Blog page may show "No blogs found" due to content collection configuration

## Common Tasks

### Fixing Build Issues

- **Missing JSON files error**: Create `prs.json` with `[]` and `repo-info.json` with `{}`
- **TypeScript errors**: Run `npm run build` which includes `astro check`
- **Formatting errors**: Run `npx prettier --write .`
- **Rust clippy warnings**: Fix manually or run `cargo clippy --fix`

### Working with Content

- Blog posts are in `/src/content/blog/*.md`
- Main pages are in `/src/pages/*.astro`
- Components are in `/src/components/*.astro`
- Layouts are in `/src/layouts/*.astro`

### Understanding the Codebase Structure

```
├── src/
│   ├── components/     # Reusable Astro components
│   ├── content/        # Blog posts and content collections
│   ├── layouts/        # Page layout templates
│   ├── pages/          # Astro pages (file-based routing)
│   └── assets/         # Images and static assets
├── tools/              # Rust workspace for content generation
│   ├── contributing/   # GitHub PR/contribution fetcher
│   └── repo-info/      # Repository information fetcher
├── public/             # Static assets served directly
└── dist/               # Build output (generated)
```

### CI/CD Requirements

The GitHub Actions CI requires:

- `npx prettier --check . --ignore-path .gitignore` must pass
- `npm run build` must succeed
- `cd tools && cargo fmt --all -- --check` must pass
- `cd tools && cargo clippy --workspace --all-features -- -Dwarnings` must pass
- `cd tools && cargo test` must pass

### Time Expectations and Timeouts

- **npm install**: ~48s-1m43s (set timeout: 120+ seconds)
- **npm run build**: ~9-11 seconds (set timeout: 30+ seconds)
- **cargo build** (first time): ~20s-1m51s (set timeout: 180+ seconds)
- **cargo test**: ~1-3 seconds (set timeout: 30+ seconds)
- **cargo clippy** (first time): ~8-11 seconds (set timeout: 60+ seconds)
- **cargo fmt --check**: <1 second (set timeout: 10+ seconds)

**NEVER CANCEL long-running commands.** Builds and dependency downloads may take significant time, especially on first run.

## Environment Notes

- Requires Node.js v20+ and npm
- Requires Rust/Cargo for tools development
- GitHub API token needed for full content generation functionality
- Some external API calls may fail in restricted environments but core functionality remains intact
