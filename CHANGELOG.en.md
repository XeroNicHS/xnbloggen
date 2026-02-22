# Changelog

> **Language**: [🇰🇷 Korean](CHANGELOG.md) | [🇺🇸 English](CHANGELOG.en.md)

## [0.1.1] - 2026-02-22

### Added

- Template custom filters (`src/utils/filters.rs`)
  - `date` filter: Format a date value with a specified format string (supports `fmt` keyword argument)
    - Input: RFC3339 string (`DateTime<FixedOffset>`) or plain `YYYY-MM-DD` string
    - Default format: `%Y-%m-%d`
    - Example: `{{ post.date | date(fmt="%B %d, %Y") }}`
  - `slugify` filter: Convert a string into a URL-safe slug
    - Example: `{{ post.title | slugify }}`

### Changed

- Moved `slugify` function from `context/context_builder.rs` to `utils/filters.rs`
  - Existing behavior remains unchanged

---

## [0.1.0] - 2026-02-04

- Markdown-based static blog generator
- `create` command: Initialize new blog project
- `new` command: Create post/page
- `build` command: Build static site
- `server` command: Local development server
- Posts and pages distinction
- Draft functionality
- Custom Taxonomies (Tags, Categories, etc.)
- Archive system (Yearly/Monthly/Daily)
- Pagination
- Custom templates
- Code highlighting
- Post navigation (previous/next)
- Auto-generate RSS feed
- Auto-generate Sitemap
