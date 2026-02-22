# XNBlogGen Template Context Specification

> **Language**: [🇰🇷 Korean](template-context.md) | [🇺🇸 English](template-context.en.md)

This document provides a **detailed specification of all variables and objects available in the XNBlogGen template engine (Jinja)**.

## Table of Contents

1. [Basic Principles](#1-basic-principles)
2. [Global Variables (site)](#2-global-variables-site)
3. [Home Page Variables (home)](#3-home-page-variables-home)
4. [List Page Variables (list)](#4-list-page-variables-list)
5. [Post Page Variables (post)](#5-post-page-variables-post)
6. [Page Variables (page)](#6-page-variables-page)
7. [Data Structure Specification](#7-data-structure-specification)
8. [Available Variables by Template](#8-available-variables-by-template)
9. [Practical Examples](#9-practical-examples)
10. [Template Filters](#10-template-filters)

---

## 1. Basic Principles

### Variable Naming Rules

- **`site`**: Global site information (available in all templates)
- **`home`**: Home page-specific data (theme.yaml - template_default.home, default : "home.html")
- **`list`**: List page-specific data (theme.yaml - template_default.list, default : "list.html")
- **`post`**: Individual post-specific data (theme.yaml - template_default.post, default : "post.html")
- **`page`**: Individual page-specific data (theme.yaml - template_default.page, default : "page.html")

### Compatibility Guarantee

- Field names and types maintain compatibility across versions
- New fields may be added but existing fields will not be removed
- HTML content fields use the `_html` suffix
- Optional fields are marked with `?`

---

## 2. Global Variables (site)

**Available in**: All templates (home.html, list.html, post.html, page.html, base.html, etc.)

**Purpose**: Site-wide configuration and global data

### Field List

| Field | Type | Description |
|-------|------|-------------|
| `title` | string | Blog name |
| `base_url` | string | Base blog URL (e.g., https://blog.example.com) |
| `path` | string | Subpath (e.g., /blog, empty if root) |
| `description` | string | Blog description |
| `language` | string? | Language code (e.g., ko, en) |
| `author` | string? | Blog author name |
| `email` | string? | Author email |
| `theme` | object? | Theme-specific user-defined settings (additional fields in theme.yaml) |
| `taxonomies` | BTreeMap<string, TaxonomyItem[]> | Dynamic taxonomy map (based on theme.yaml config) |
| `archives` | ArchiveItem[] | List of all archives (supports yearly/monthly/daily) |
| `recent_posts` | PostListItem[] | Recent posts list (default 10 items) |

**taxonomies structure:**
- Auto-generated for each taxonomy defined in theme.yaml
- Examples: `site.taxonomies.tags`, `site.taxonomies.categories`, `site.taxonomies.series`
- Taxonomies not defined in theme.yaml will not exist

**theme field:**
- Contains all user-defined settings from theme.yaml excluding core fields (`meta`, `template_default`, `template_extra`, `pagination`, `taxonomies`)
- Example: If you add custom fields like `social_links`, `footer_text`, `color_scheme` to theme.yaml, they can be accessed as `site.theme.social_links`, `site.theme.footer_text`, `site.theme.color_scheme`
- Extensible area that theme authors can freely define

### Usage Example

```jinja
<header>
  <h1>{{ site.title }}</h1>
  <p>{{ site.description }}</p>
</header>

{# Only show if tags are defined in theme.yaml #}
{% if site.taxonomies.tags %}
  <aside>
    <h3>Tags</h3>
    <ul>
      {% for tag in site.taxonomies.tags %}
        <li><a href="{{ tag.url }}">{{ tag.label }} ({{ tag.count }})</a></li>
      {% endfor %}
    </ul>
  </aside>
{% endif %}

{# Recent posts sidebar #}
<aside>
  <h3>Recent Posts</h3>
  <ul>
    {% for post in site.recent_posts %}
      <li>
        <a href="{{ post.url }}">{{ post.title }}</a>
        <time>{{ post.date[:10] }}</time>
      </li>
    {% endfor %}
  </ul>
</aside>
```

### theme field usage examples

**Adding custom settings to theme.yaml:**
```yaml
# theme.yaml
name: "MyTheme"
author: "Theme Author"
version: "1.0.0"

# Core ThemeManifest fields...
template_default:
  home: "home.html"
  
# User-defined fields (accessible via site.theme)
social_links:
  github: "https://github.com/username"
  twitter: "https://twitter.com/username"
footer_text: "© 2026 My Blog"
color_scheme:
  primary: "#007bff"
  secondary: "#6c757d"
```

**Usage in templates:**
```jinja
{# Display social links #}
{% if site.theme.social_links %}
  <div class="social-links">
    {% if site.theme.social_links.github %}
      <a href="{{ site.theme.social_links.github }}">GitHub</a>
    {% endif %}
    {% if site.theme.social_links.twitter %}
      <a href="{{ site.theme.social_links.twitter }}">Twitter</a>
    {% endif %}
  </div>
{% endif %}

{# Display footer text #}
<footer>
  {% if site.theme.footer_text %}
    <p>{{ site.theme.footer_text }}</p>
  {% endif %}
</footer>

{# Set colors as CSS variables #}
{% if site.theme.color_scheme %}
<style>
  :root {
    --primary-color: {{ site.theme.color_scheme.primary }};
    --secondary-color: {{ site.theme.color_scheme.secondary }};
  }
</style>
{% endif %}
```

---

## 3. Home Page Variables (home)

**Available in**: `theme.yaml - template_default.home`, default : `home.html`

**Purpose**: Data needed for rendering the home page (index)

> ⚠️ **Important**: The `home` variable is actually of type `ListContext` and is an **alias** for the `list` variable. Internally, a `ListContext` is created and passed to the template with the name `home`. Therefore, `home` and `list` refer to exactly the same object.

### Field List

> The `home` variable is of type `ListContext`, so all fields below are fields of `ListContext`. See [List Page Variables (list)](#4-list-page-variables-list) section for more details.

| Field | Type | Description |
|-------|------|-------------|
| `title` | string | Home page title (typically site name) |
| `url` | string | Home page URL (typically /) |
| `description` | string? | Home page description |
| `list_kind` | ListKind | Type of list (For home: `{ "type": "home" }`) |
| `posts` | PostListItem[] | List of posts to display on home |
| `pagination` | Pagination | Pagination information |

### Usage Example

```jinja
<main>
  <h2>{{ home.title }}</h2>
  
  {% for post in home.posts %}
    <article>
      <h3><a href="{{ post.url }}">{{ post.title }}</a></h3>
      <time>{{ post.date }}</time>
      {% if post.summary %}
        <p>{{ post.summary }}</p>
      {% endif %}
    </article>
  {% endfor %}
  
  <nav>
    {% if home.pagination.prev %}
      <a href="{{ home.pagination.prev.url }}">Previous</a>
    {% endif %}
    {% if home.pagination.next %}
      <a href="{{ home.pagination.next.url }}">Next</a>
    {% endif %}
  </nav>
</main>
```

---

## 4. List Page Variables (list)

**Available in**: `theme.yaml - template_default.list`, default : `list.html`

**Purpose**: Rendering data for tag, category, and archive list pages

### Field List

| Field | Type | Description |
|-------|------|-------------|
| `title` | string | List title (e.g., "Tag: Rust", "January 2026") |
| `url` | string | List page URL |
| `description` | string? | List page description |
| `list_kind` | ListKind | Type of list (Taxonomy, Archive) |
| `posts` | PostListItem[] | Filtered post list |
| `pagination` | Pagination | Pagination information |

### ListKind Types

ListKind is serialized as JSON objects:

**Home:**
```json
{ "type": "home" }
```

**Taxonomy:**
```json
{ 
  "type": "taxonomy",
  "name": "tags",
  "slug": "rust"
}
```
- `name`: taxonomy name (e.g., "tags", "categories", "series")
- `slug`: taxonomy value (e.g., "rust", "programming", "web-dev")

**Archive:**
```json
{
  "type": "archive",
  "year": 2026,
  "month": 1,
  "day": 15
}
```
- `year`: Year (4 digits, always present)
- `month`: Month (1-12, not present for Yearly, Optional)
- `day`: Day (1-31, not present for Monthly/Yearly, Optional)

**Template usage:**
```jinja
{% if list.list_kind.type == "taxonomy" %}
  <h2>{{ list.list_kind.name }}: {{ list.list_kind.slug }}</h2>
{% elif list.list_kind.type == "archive" %}
  {% if list.list_kind.day %}
    <h2>{{ list.list_kind.year }}-{{ list.list_kind.month }}-{{ list.list_kind.day }}</h2>
  {% elif list.list_kind.month %}
    <h2>{{ list.list_kind.year }}-{{ list.list_kind.month }}</h2>
  {% else %}
    <h2>{{ list.list_kind.year }}</h2>
  {% endif %}
{% endif %}
```

### Usage Example

```jinja
<main>
  <h2>{{ list.title }}</h2>
  <p>{{ list.posts | length }} posts</p>
  
  {% for post in list.posts %}
    <article>
      <h3><a href="{{ post.url }}">{{ post.title }}</a></h3>
      <time>{{ post.date }}</time>
    </article>
  {% endfor %}
  
  <nav>
    {% if list.pagination.prev %}
      <a href="{{ list.pagination.prev.url }}">← Previous Page</a>
    {% endif %}
    <span>{{ list.pagination.page }} / {{ list.pagination.total_pages }}</span>
    {% if list.pagination.next %}
      <a href="{{ list.pagination.next.url }}">Next Page →</a>
    {% endif %}
  </nav>
</main>
```

---

## 5. Post Page Variables (post)

**Available in**: `theme.yaml - template_default.post`, default : `post.html`

**Purpose**: Rendering data for individual blog posts

### Field List

| Field | Type | Description |
|-------|------|-------------|
| `kind` | object | Content type ({ "type": "post" } or { "type": "page" }) |
| `title` | string | Post title |
| `url` | string | Post URL |
| `description` | string? | Post description (for meta tags) |
| `language` | string? | Post language code |
| `date` | string | Publication date (YYYY-MM-DD format) |
| `updated` | string? | Last updated date |
| `taxonomies` | BTreeMap<string, TaxonomyItem[]> | Taxonomies assigned to post (based on theme.yaml config) |
| `summary` | string? | Post summary |
| `thumbnail` | string? | Thumbnail image URL |
| `content_html` | string | HTML-converted body content |
| `prev` | NavLink? | Previous post link |
| `next` | NavLink? | Next post link |
| `extra` | object | Custom fields from Front Matter |

### Usage Example

```jinja
<article>
  <header>
    <h1>{{ post.title }}</h1>
    <time datetime="{{ post.date }}">{{ post.date }}</time>

    {# Only show if tags are defined in theme.yaml and assigned to post #}
    {% if post.taxonomies.tags %}
      <div class="tags">
        {% for tag in post.taxonomies.tags %}
          <a href="{{ tag.url }}">#{{ tag.label }}</a>
        {% endfor %}
      </div>
    {% endif %}
  </header>

  <div class="content">
    {{ post.content_html | safe }}
  </div>

  <nav class="post-nav">
    {% if post.prev %}
      <a href="{{ post.prev.url }}" class="prev">
        ← {{ post.prev.title }}
      </a>
    {% endif %}
    {% if post.next %}
      <a href="{{ post.next.url }}" class="next">
        {{ post.next.title }} →
      </a>
    {% endif %}
  </nav>
</article>
```

---

## 6. Page Variables (page)

**Available in**: `theme.yaml - template_default.page`, default : `page.html`

**Purpose**: Rendering data for static pages like About, Contact

### Field List

| Field | Type | Description |
|-------|------|-------------|
| `title` | string | Page title |
| `url` | string | Page URL |
| `description` | string? | Page description |
| `language` | string? | Page language code |
| `date` | string | Creation date |
| `updated` | string? | Last updated date |
| `content_html` | string | HTML-converted body content |
| `extra` | object | Custom fields from Front Matter |

### Usage Example

```jinja
<article>
  <h1>{{ page.title }}</h1>
  
  <div class="content">
    {{ page.content_html | safe }}
  </div>
  
  {% if page.updated %}
    <footer>
      <p>Last updated: {{ page.updated }}</p>
    </footer>
  {% endif %}
</article>
```

---

## 7. Data Structure Specification

### TaxonomyItem

Represents a tag or category item.

| Field | Type | Description |
|-------|------|-------------|
| `label` | string | Display name (e.g., "Rust", "Development") |
| `url` | string | Tag/category page URL |
| `count` | number | Number of posts with this tag/category |

```jinja
{# Only show if tags are defined in theme.yaml #}
{% if site.taxonomies.tags %}
  {% for tag in site.taxonomies.tags %}
    <a href="{{ tag.url }}">{{ tag.label }} ({{ tag.count }})</a>
  {% endfor %}
{% endif %}
```

### ArchiveItem

Represents an archive item (yearly/monthly/daily).

| Field | Type | Description |
|-------|------|-------------|
| `label` | string | Display name (e.g., "2026", "2026-01", "2026-01-15") |
| `kind` | string | Archive type ("yearly", "monthly", "daily") |
| `year` | number | Year (4 digits) |
| `month` | number? | Month (1-12, only present for Monthly/Daily) |
| `day` | number? | Day (1-31, only present for Daily) |
| `url` | string | Archive page URL |
| `count` | number | Number of posts in this period |

```jinja
{% for archive in site.archives %}
  <a href="{{ archive.url }}">{{ archive.label }} ({{ archive.count }})</a>
{% endfor %}

{# Display different formats based on kind #}
{% for archive in site.archives %}
  {% if archive.kind == "yearly" %}
    <li><a href="{{ archive.url }}">{{ archive.year }} ({{ archive.count }})</a></li>
  {% elif archive.kind == "monthly" %}
    <li><a href="{{ archive.url }}">{{ archive.year }}-{{ archive.month }} ({{ archive.count }})</a></li>
  {% elif archive.kind == "daily" %}
    <li><a href="{{ archive.url }}">{{ archive.year }}-{{ archive.month }}-{{ archive.day }} ({{ archive.count }})</a></li>
  {% endif %}
{% endfor %}
```

### PostListItem

Represents a post list item (for home/list pages).

| Field | Type | Description |
|-------|------|-------------|
| `title` | string | Post title |
| `url` | string | Post URL |
| `date` | DateTime | Publication date (ISO 8601 format) |
| `taxonomies` | BTreeMap<string, TaxonomyItem[]>? | Taxonomies assigned to post (based on theme.yaml config) |
| `summary` | string? | Post summary |
| `thumbnail` | string? | Thumbnail image URL |

```jinja
{% for post in home.posts %}
  <article>
    <h3><a href="{{ post.url }}">{{ post.title }}</a></h3>
    <time datetime="{{ post.date }}">{{ post.date[:10] }}</time>
    
    {# Display post tags #}
    {% if post.taxonomies and post.taxonomies.tags %}
      <div class="tags">
        {% for tag in post.taxonomies.tags %}
          <a href="{{ tag.url }}">#{{ tag.label }}</a>
        {% endfor %}
      </div>
    {% endif %}
    
    {% if post.thumbnail %}
      <img src="{{ post.thumbnail }}" alt="{{ post.title }}">
    {% endif %}
    {% if post.summary %}
      <p>{{ post.summary }}</p>
    {% endif %}
  </article>
{% endfor %}
```

### NavLink

Represents prev/next post or pagination links.

| Field | Type | Description |
|-------|------|-------------|
| `title` | string | Link text |
| `url` | string | Link URL |

**Pagination NavLink title values:**
- `prev`: "Page N" (N is previous page number)
- `next`: "Page N" (N is next page number)
- `first`: "First"
- `last`: "Last"

**Post navigation NavLink title values:**
- `post.prev.title`: Actual title of previous post
- `post.next.title`: Actual title of next post

```jinja
{% if post.prev %}
  <a href="{{ post.prev.url }}">← {{ post.prev.title }}</a>
{% endif %}
```

### Pagination

Represents pagination information.

| Field | Type | Description |
|-------|------|-------------|
| `page` | number | Current page number (starts from 1) |
| `per_page` | number | Items per page (if 0, set to total_items) |
| `total_items` | number | Total number of items |
| `total_pages` | number | Total number of pages |
| `has_prev` | boolean | Whether previous page exists |
| `has_next` | boolean | Whether next page exists |
| `prev` | NavLink? | Previous page link |
| `next` | NavLink? | Next page link |
| `first` | NavLink? | First page link (only exists when not on first page) |
| `last` | NavLink? | Last page link (only exists when not on last page and total pages > 1) |
| `pages` | PageLink[] | Page number list (for page number navigation) |

```jinja
<nav class="pagination">
  {# First page link #}
  {% if home.pagination.first %}
    <a href="{{ home.pagination.first.url }}">« First</a>
  {% endif %}

  {# Previous page #}
  {% if home.pagination.prev %}
    <a href="{{ home.pagination.prev.url }}">‹ Previous</a>
  {% endif %}

  {# Page number list #}
  {% for page_link in home.pagination.pages %}
    {% if page_link.is_current %}
      <span class="current">{{ page_link.number }}</span>
    {% else %}
      <a href="{{ page_link.url }}">{{ page_link.number }}</a>
    {% endif %}
  {% endfor %}

  {# Next page #}
  {% if home.pagination.next %}
    <a href="{{ home.pagination.next.url }}">Next ›</a>
  {% endif %}

  {# Last page link #}
  {% if home.pagination.last %}
    <a href="{{ home.pagination.last.url }}">Last »</a>
  {% endif %}
</nav>
```

### PageLink

Represents a page number link (items in Pagination's pages array).

| Field | Type | Description |
|-------|------|-------------|
| `number` | number | Page number (0 means ellipsis "...") |
| `url` | string | Page URL (empty string if number is 0) |
| `is_current` | boolean | Whether this is the current page |

**Note**: When `number` is 0, it represents an ellipsis "..." in the page number list. In this case, `url` is an empty string, so no link should be generated.

```jinja
{% for page_link in home.pagination.pages %}
  {% if page_link.number == 0 %}
    <span class="ellipsis">...</span>
  {% elif page_link.is_current %}
    <span class="current">{{ page_link.number }}</span>
  {% else %}
    <a href="{{ page_link.url }}">{{ page_link.number }}</a>
  {% endif %}
{% endfor %}
```

---

## 8. Available Variables by Template

| Variable | home.html | list.html | post.html | page.html | base.html |
|----------|-----------|-----------|-----------|-----------|-----------|
| `site` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `home` | ✅ | ❌ | ❌ | ❌ | ❌ |
| `list` | ✅ | ✅ | ❌ | ❌ | ❌ |
| `post` | ❌ | ❌ | ✅ | ❌ | ❌ |
| `page` | ❌ | ❌ | ❌ | ✅ | ❌ |

**Note**: 
- **`home` is an alias for `list`**: The `home` variable is actually of type `ListContext` and is exactly the same object as the `list` variable. In `home.html`, the same `ListContext` object is injected with both `home` and `list` names.
- **Recommended usage**: For readability and clarity, use the `home` variable in `home.html` and the `list` variable in `list.html`. However, since they are identical, you can also access via `list` in `home.html`.
- `base.html` is a layout that other templates extend/include, so it can use variables from the inheriting template.

---

## 9. Practical Examples

### Example 1: Sidebar (base.html)

```jinja
<aside class="sidebar">
  {# Only show if tags are defined in theme.yaml #}
  {% if site.taxonomies.tags %}
    <section>
      <h3>Tags</h3>
      <div class="tag-cloud">
        {% for tag in site.taxonomies.tags %}
          <a href="{{ tag.url }}"
             style="font-size: {{ 100 + (tag.count * 20) }}%">
            {{ tag.label }} ({{ tag.count }})
          </a>
        {% endfor %}
      </div>
    </section>
  {% endif %}

  {# Only show if categories are defined in theme.yaml #}
  {% if site.taxonomies.categories %}
    <section>
      <h3>Categories</h3>
      <ul>
        {% for category in site.taxonomies.categories %}
          <li>
            <a href="{{ category.url }}">
              {{ category.label }} <span>({{ category.count }})</span>
            </a>
          </li>
        {% endfor %}
      </ul>
    </section>
  {% endif %}

  {# Archives are automatically generated #}
  <section>
    <h3>Archives</h3>
    <ul>
      {% for archive in site.archives %}
        <li>
          <a href="{{ archive.url }}">
            {{ archive.year }}-{{ archive.month }} ({{ archive.count }})
          </a>
        </li>
      {% endfor %}
    </ul>
  </section>
</aside>
```

### Example 2: Post Detail (post.html)

```jinja
{% extends "base.html" %}

{% block content %}
<article class="post">
  <header>
    <h1>{{ post.title }}</h1>

    <div class="meta">
      <time datetime="{{ post.date }}">{{ post.date }}</time>

      {# Only show if categories are defined in theme.yaml and assigned to post #}
      {% if post.taxonomies.categories %}
        <span class="categories">
          {% for category in post.taxonomies.categories %}
            <a href="{{ category.url }}">{{ category.label }}</a>
          {% endfor %}
        </span>
      {% endif %}
    </div>

    {# Only show if tags are defined in theme.yaml and assigned to post #}
    {% if post.taxonomies.tags %}
      <div class="tags">
        {% for tag in post.taxonomies.tags %}
          <a href="{{ tag.url }}" class="tag">#{{ tag.label }}</a>
        {% endfor %}
      </div>
    {% endif %}
  </header>

  {% if post.thumbnail %}
    <img src="{{ post.thumbnail }}" alt="{{ post.title }}" class="featured-image">
  {% endif %}

  <div class="content">
    {{ post.content_html | safe }}
  </div>

  <nav class="post-navigation">
    {% if post.prev %}
      <div class="prev-post">
        <span class="label">Previous Post</span>
        <a href="{{ post.prev.url }}">{{ post.prev.title }}</a>
      </div>
    {% endif %}

    {% if post.next %}
      <div class="next-post">
        <span class="label">Next Post</span>
        <a href="{{ post.next.url }}">{{ post.next.title }}</a>
      </div>
    {% endif %}
  </nav>
</article>
{% endblock %}
```

### Example 3: Home Page (home.html)

```jinja
{% extends "base.html" %}

{% block content %}
<main class="home">
  <section class="recent-posts">
    <h2>Recent Posts</h2>
    
    <div class="post-grid">
      {% for post in home.posts %}
        <article class="post-card">
          {% if post.thumbnail %}
            <a href="{{ post.url }}">
              <img src="{{ post.thumbnail }}" alt="{{ post.title }}">
            </a>
          {% endif %}
          
          <h3><a href="{{ post.url }}">{{ post.title }}</a></h3>
          <time>{{ post.date }}</time>
          
          {% if post.summary %}
            <p class="summary">{{ post.summary }}</p>
          {% endif %}
          
          <a href="{{ post.url }}" class="read-more">Read more →</a>
        </article>
      {% endfor %}
    </div>
    
    {% if home.pagination.total_pages > 1 %}
      <nav class="pagination">
        {% if home.pagination.prev %}
          <a href="{{ home.pagination.prev.url }}" class="prev">
            ← Previous Page
          </a>
        {% endif %}
        
        <span class="page-info">
          {{ home.pagination.page }} / {{ home.pagination.total_pages }}
        </span>
        
        {% if home.pagination.next %}
          <a href="{{ home.pagination.next.url }}" class="next">
            Next Page →
          </a>
        {% endif %}
      </nav>
    {% endif %}
  </section>
</main>
{% endblock %}
```

### Example 4: List Page (list.html)

```jinja
{% extends "base.html" %}

{% block content %}
<main class="list-page">
  <header>
    <h1>{{ list.title }}</h1>
    <p>{{ list.posts | length }} posts</p>
  </header>
  
  <div class="post-list">
    {% for post in list.posts %}
      <article class="post-item">
        <h2><a href="{{ post.url }}">{{ post.title }}</a></h2>
        <time>{{ post.date }}</time>
        
        {% if post.summary %}
          <p>{{ post.summary }}</p>
        {% endif %}
      </article>
    {% endfor %}
  </div>
  
  {% if list.pagination.total_pages > 1 %}
    <nav class="pagination">
      {% if list.pagination.prev %}
        <a href="{{ list.pagination.prev.url }}">← Previous</a>
      {% endif %}
      
      <span>{{ list.pagination.page }} / {{ list.pagination.total_pages }}</span>
      
      {% if list.pagination.next %}
        <a href="{{ list.pagination.next.url }}">Next →</a>
      {% endif %}
    </nav>
  {% endif %}
</main>
{% endblock %}
```

---

## 10. Template Filters

XNBlogGen provides the following custom filters in the minijinja template engine.

---

### `date`

Formats a date/datetime string using a specified format.

**Input Types**
- RFC3339 string: `2024-01-15T09:00:00+09:00` (default format of the `date` field in front matter)
- Plain date string: `2024-01-15`

**Keyword Arguments**

| Argument | Type | Default | Description |
|----------|------|---------|-------------|
| `fmt` | string | `%Y-%m-%d` | chrono format specifier |

**Common Format Specifiers**

| Specifier | Example Output | Description |
|-----------|----------------|-------------|
| `%Y` | `2026` | 4-digit year |
| `%m` | `02` | 2-digit month (numeric) |
| `%d` | `12` | 2-digit day |
| `%B` | `February` | Full month name |
| `%b` | `Feb` | Abbreviated month name |
| `%A` | `Thursday` | Full weekday name |
| `%a` | `Thu` | Abbreviated weekday name |
| `%H` | `09` | Hour (24-hour clock) |
| `%M` | `30` | Minute |
| `%S` | `00` | Second |

**Usage Examples**

```jinja
{# Default: 2026-02-12 #}
{{ post.date | date }}

{# Full format: February 12, 2026 #}
{{ post.date | date(fmt="%B %d, %Y") }}

{# Short format: Feb 12 #}
{{ post.date | date(fmt="%b %d") }}

{# With time: 2026-02-12 09:30 #}
{{ post.date | date(fmt="%Y-%m-%d %H:%M") }}
```

---

### `slugify`

Converts a string into a URL-safe slug.

**Conversion Rules**
- Converts to lowercase
- Replaces special symbols: `c++` → `cpp`, `.net` → `dotnet`, `+` → `plus`, `#` → `sharp`, `@` → `at`, `&` → `and`
- Spaces and hyphens are collapsed into a single `-`
- Non-ASCII characters (e.g., Korean, Japanese) are preserved as-is
- Leading and trailing `-` are removed

**Usage Examples**

```jinja
{# "Hello World" → "hello-world" #}
{{ post.title | slugify }}

{# "C++ Guide" → "cpp-guide" #}
{{ post.title | slugify }}

{# "C# & .NET" → "csharp-and-dotnet" #}
{{ post.title | slugify }}

{# Generate tag link #}
<a href="/tags/{{ tag.label | slugify }}/">{{ tag.label }}</a>
```

---

## Compatibility and Version Policy

- **Adding Fields**: New fields may be added while maintaining backward compatibility
- **Removing Fields**: Existing fields are deprecated for at least 1 major version before removal
- **Type Changes**: Field type changes only occur in major version updates
- **Naming Convention**: All fields use snake_case

---

**Document Version**: 1.0
**Last Updated**: 2026-01-26
