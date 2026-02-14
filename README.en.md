# xnBlogGen

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org) <br/>
![Windows](https://img.shields.io/badge/OS-Windows-green.svg)
![Linux](https://img.shields.io/badge/OS-Linux-green.svg)
![Mac](https://img.shields.io/badge/OS-Mac-green.svg)

> **Language**: [🇰🇷 Korean](README.md) | [🇺🇸 English](README.en.md)

**Fast and Simple Static Blog Generator Written in Rust**

![xnBlogGen.png](xnBlogGen.png)

xnBlogGen is a blog generator that converts posts written in Markdown to static HTML. It uses a Jinja-style template engine and allows easy design customization through a theme system.

## Features

- 🚀 **Fast Build**: Quickly process large numbers of posts with Rust's performance
- 🎨 **Theme System**: Flexible theme customization based on Jinja templates
- 📝 **Markdown Support**: Write posts using intuitive Markdown syntax
- 🏷️ **Dynamic Taxonomy System**: Flexible taxonomy system definable in themes (expandable beyond tags and categories)
- 📄 **Pagination**: Automatic page splitting for home, taxonomy, and archive pages
- ✨ **Code Highlighting**: Automatic syntax highlighting for Markdown code blocks (syntect)
- 📡 **RSS & Sitemap**: Auto-generated RSS feed and sitemap
- 🔗 **Flexible Permalinks**: Date and slug-based URL pattern support
- 📋 **Page Support**: Create independent pages like About, Contact in addition to posts

## Installation

### Install via Cargo

```bash
git clone https://github.com/XeroNicHS/xnbloggen
cd xnbloggen
cargo build --release
```

The built executable will be created at `target/release/xnbloggen.exe` (Windows) or `target/release/xnbloggen` (Linux/macOS).

### System Requirements

- Rust 2024 Edition or higher
- Windows, Linux, macOS supported

## Quick Start

Create your blog in 5 minutes!

```bash
# 1. Create a new blog project
xnbloggen create --root myblog

# 2. Create your first post
cd myblog
xnbloggen new "My First Post"

# 3. Edit the markdown file
# Open content/posts/YYYY-MM-DD-my-first-post.md and write your content

# 4. Build
xnbloggen build

# 5. Check locally (temporary method)
xnbloggen server
# Access http://localhost:8000 in your browser
```

### 1. Project Creation (`create`)

Initialize a new blog project. Automatically generates basic directory structure, configuration files, and default theme.

```bash
# Create project at specified path
xnbloggen create --root myblog

# Initialize project in current directory
cd myblog
xnbloggen create
```

**Options:**
- `--root <path>`: Project root directory (default: current directory)

**Generated Structure:**
```
myblog/
├── blogconfig.yaml     # Blog configuration file
├── content/            # Content folder
│   ├── posts/          # Posts location
│   ├── pages/          # Pages location
│   ├── images/         # Images location
│   └── data/           # Data location
└── themes/             # Theme folder
    └── default/        # Default theme
        ├── theme.yaml
        ├── assets/
        └── templates/
```

### 2. Writing Content (`new`)

Create a new post or page. A Markdown file with Front Matter is automatically created.

```bash
# Create post (default)
xnbloggen new "My First Post"
# → content/posts/2026-01-24-my-first-post.md

# Create page
xnbloggen new "About" --page
# → content/pages/About.md

# Specify path
xnbloggen new "New Post" --root myblog
```

**Options:**
- `--page`: Create page (instead of post)
- `--root <path>`: Project root directory

**Difference between Posts and Pages:**
- **Posts**: Date-based sorted blog articles (news, diary, tutorials, etc.)
- **Pages**: Independent static pages (About, Contact, Portfolio, etc.)

### Image Management

xnBlogGen supports 3 image management patterns and **automatically detects and processes** them. You can mix multiple patterns in the same project.

#### Pattern 1: Centralized Approach (Existing Method)

Store all images in the `content/images/` folder and reference them with absolute paths.

**Structure:**
```
content/
├── images/
│   ├── thumbnails/
│   │   └── shared-thumb.jpg
│   └── 2024/01/
│       └── diagram.png
└── posts/
    └── 2024-01-15-my-post.md
```

**Front Matter & Body:**
```yaml
---
thumbnail: "/images/thumbnails/shared-thumb.jpg"
---

![Diagram](/images/2024/01/diagram.png)
```

**Advantages:**
- Reuse same images across multiple posts
- Preview works when opening `content/` folder as Obsidian vault

---

#### Pattern 2: Flat Structure + Filename Convention

Place images in the same folder as posts, linking them with filename prefix.

**Structure:**
```
content/posts/
├── 2024-01-15-my-post.md
├── 2024-01-15-my-post-diagram.png    # With date
├── my-post-thumbnail.jpg              # Slug only
└── my-post-photo.jpg
```

**Front Matter & Body:**
```yaml
---
thumbnail: "./my-post-thumbnail.jpg"
---

![Diagram](./my-post-diagram.png)
```

**Advantages:**
- Image preview works when opening single md file (images in same folder)
- Clear association through filenames
- Auto-recognizes both with/without date

---

#### Pattern 3: Folder Structure

Create post as folder containing `index.md` and images together.

**Structure:**
```
content/posts/
└── 2024-01-15-my-post/
    ├── index.md
    ├── diagram.png        # Short filename
    ├── photo.jpg
    └── thumbnail.jpg
```

**Front Matter & Body:**
```yaml
---
thumbnail: "./thumbnail.jpg"
---

![Diagram](./diagram.png)
```

**Advantages:**
- Cleanest structure
- Shortest image filenames (no prefix needed)
- Highest post-image cohesion
- Perfect preview when opening folder in VSCode

---

#### Build Processing

- **Body Images**: Relative paths kept as-is → Browser auto-resolves
- **Thumbnails**: Relative paths in Front Matter → Auto-converted to absolute paths
- All images copied to `public/{post-url}/` folder
- Pattern mixing allowed (multiple patterns can coexist in project)

**Supported Image Formats:**
- `png`, `jpg`, `jpeg`, `gif`, `webp`, `svg`, `avif`, `bmp`, `ico`, `tiff`, `tif`

### 3. Build (`build`)

Convert Markdown files to HTML to generate a deployable static site.

```bash
# Build project
xnbloggen build

# Specify path
xnbloggen build --root myblog
```

**Generated Files:**
- Home page (`index.html`, `/page/N/index.html` - pagination)
- Individual post pages (according to permalink pattern)
- Individual pages
- **Taxonomy list pages** (based on theme.yaml settings, with pagination)
  - e.g., `/tags/:tag/index.html`, `/categories/:category/index.html`
  - Only taxonomies defined in theme.yaml are generated
- Archive pages (Yearly/Monthly/Daily selectable, with pagination)
  - e.g., `/archives/YYYY/index.html`, `/archives/YYYY/MM/index.html`, `/archives/YYYY/MM/DD/index.html`
- RSS feed (`/rss.xml`)
- Sitemap (`/sitemap.xml`)
- robots.txt (`/robots.txt`)
- Static assets (CSS, JS, images)

**Options:**
- `--root <path>`: Project root directory

### Permalink Patterns

Configure post URL structure globally in the `permalinks` section of `blogconfig.yaml`.

**Supported Patterns:**
- `:year` - Year (4 digits, e.g., 2026)
- `:month` - Month (2 digits, e.g., 01)
- `:day` - Day (2 digits, e.g., 24)
- `:slug` - Slug extracted from filename

**Examples:**
```yaml
# blogconfig.yaml
permalinks:
  post: "/post/:year/:month/:day/:slug/"
# → /post/2026/01/24/my-first-post/

permalinks:
  post: "/:slug/"
# → /my-first-post/
```

### Taxonomy System

Systematically categorize posts and automatically generate list pages.

#### Dynamic Taxonomy System

You can freely define any taxonomy you want in theme settings (`theme.yaml`).

**Fully Dynamic Structure:**
- Only taxonomies defined in `theme.yaml`'s `taxonomies` setting are generated
- tags and categories are not special, just types of taxonomy
- Taxonomies not configured are not generated
- Each taxonomy has its own independent URL structure and list page
- All taxonomy pages support pagination

**Common Examples:**
- `tags`: Keyword-based free classification (URL: `/tags/<tag-name>/`)
- `categories`: Hierarchical topic classification (URL: `/categories/<category-name>/`)
- `series`: Series classification (URL: `/series/<series-name>/`)
- `authors`: Author-based classification (URL: `/authors/<author-name>/`)

> 💡 Any taxonomy name can be defined, and specifying values with that name in Front Matter automatically generates index and list pages.

**Archives:**
- Date-based automatic classification
- Yearly/Monthly/Daily grouping selectable (configured in theme.yaml)
- **Customizable permalink pattern** (configured in theme.yaml's `archives.permalink`)
  - Can use `:year`, `:month`, `:day` variables
  - e.g., `/archives/:year/`, `/archives/:year/:month/`, `/blog/:year/:month/:day/`
- Pagination support

### 4. Local Development Server (`server`)

Provides an HTTP server to preview the built site locally.

```bash
# Start server on default port (8000)
xnbloggen server
# → Check at http://localhost:8000

# Specify port
xnbloggen server --port 3000

# Specify path
xnbloggen server --root myblog --port 8080
```

**Options:**
- `--root <path>`: Project root directory (default: current directory)
- `--port <port>`: Server port number (default: 8000)

**Features:**
- Static file serving (HTML, CSS, JS, images, etc.)
- Automatic Content-Type detection
- Automatic index.html serving
- Security enhancement (path traversal protection)

> ⚠️ **Warning**: This is for development/testing only. Do not use for production. For deployment, upload the built `public/` folder to a web server (Nginx, Apache) or hosting service (GitHub Pages, Netlify, Vercel).

## Configuration Files

### blogconfig.yaml

Blog global configuration file. Located at project root, configures site information, author information, theme, and build options.

```yaml
site: 
  name: "My xnBlogGen Blog"          # Blog name
  base_url: "https://yourblog.com"   # Blog URL
  path: ""                           # Sub-path (e.g., /blog)
  description: "Personal Blog"       # Blog description
  language: "en"                     # Language code

author:
  name: "Your Name"                  # Author name
  email: "your@email.com"            # Email (optional)

theme: 
  name: "default"                    # Theme name to use

permalinks:
  post: "/posts/:slug/"              # Post permalink
  page: "/pages/:slug/"              # Page permalink

build:
  output_dir: "public"               # Build output directory
  clean: true                        # Clean output directory before build
  include_drafts: false              # Include drafts
  rss: true                          # Generate RSS feed
  rss_max_items: 20                  # Maximum RSS items
  sitemap: true                      # Generate sitemap
  robots_txt: true                   # Generate robots.txt
```

### Using Themes

To use a theme other than the default, add the theme to the `themes` folder and specify it in `blogconfig.yaml`.

**Example: Using the simple-black theme**

1. Clone the theme repository into the `themes` folder:
   ```bash
   git clone https://github.com/XeroNicHS/theme-simple-black themes/simple-black
   ```

2. Specify the theme in `blogconfig.yaml`:
   ```yaml
   theme:
     name: "simple-black"
   ```

3. Run build:
   ```bash
   xnbloggen build
   ```

### theme.yaml

Theme-specific configuration file. Located at `themes/<theme-name>/theme.yaml`.

```yaml
meta:
  name: "default"
  version: "1.0.0"
  author: "xnBlogGen"
  description: "Minimal starter theme for xnBlogGen"

template_default:
  post: "post.html"                 # Post template
  page: "page.html"                 # Page template
  home: "home.html"                 # Home page template
  list: "list.html"                 # List page template

# Custom template definitions (optional)
# Define additional pages beyond default templates
template_extra: []
# Example:
# template_extra:
#   - name: "Search"
#     file: "search.html"           # Searches relative to templates folder
#     url: "/search/"
#     output: "index.html"          # Output filename (default: "index.html", optional)
#
#   # When specific filename is needed (e.g., 404 page)
#   - name: "404 Page"
#     file: "404.html"
#     url: "/"
#     output: "404.html"            # Creates /404.html

# Taxonomy settings (optional)
# Freely define any classification system you want
taxonomies:
  - name: "tags"                    # Key name to use in Front Matter
    label: "Tag"                    # Display label for list pages
    permalink: "/tags/:slug/"       # Taxonomy list page URL pattern
    per_page: 10                    # Posts per page

  - name: "categories"
    label: "Category"
    permalink: "/categories/:slug/"
    per_page: 10

  # Additional taxonomy examples:
  # - name: "series"
  #   label: "Series"
  #   permalink: "/series/:slug/"
  #   per_page: 10

# Archive settings (optional)
# Yearly, Monthly, Daily selectable
archives:
  - kind: "Monthly"  # Choose from "Yearly", "Monthly", "Daily"
    permalink: "/archives/:year/:month/"  # Can use :year, :month, :day variables
    per_page: 10
  # Yearly example: kind: "Yearly", permalink: "/archives/:year/"
  # Daily example: kind: "Daily", permalink: "/archives/:year/:month/:day/"

recent_posts:
  count: 10

# Custom theme settings (optional)
# All fields defined here can be accessed in templates via {{ site.theme.field_name }}
# Examples:
# colors:
#   primary: "#58a6ff"
#   secondary: "#1f6feb"
# social_links:
#   github: "https://github.com/username"
#   twitter: "https://twitter.com/username"
# features:
#   enable_search: true
#   enable_toc: true
```

### Front Matter

YAML metadata at the top of Markdown files.

**Post Front Matter:**
```yaml
---
title: "Post Title"
slug: "post-title"
date: 2026-01-24T14:30:00+09:00
updated: 2026-01-26T10:00:00+09:00   # Update date (optional)

# Specify classifications according to theme.yaml's taxonomies settings
taxonomies:
  tags: ["Rust", "Blog"]        # When tags are defined in theme.yaml
  categories: ["Development"]   # When categories are defined in theme.yaml
  series: ["Rust Tutorial"]     # When series are defined in theme.yaml (example)

summary: "Write a brief summary of your post here."
description: "Page description for SEO (meta description)"  # SEO description (optional)
thumbnail: ""
language: "en"                     # Post language (optional, default: site language)

draft: false

# Custom fields (extra)
author: "John Doe"                   # Access in template via post.extra.author
reading_time: 5                      # Access in template via post.extra.reading_time
difficulty: "Intermediate"           # Access in template via post.extra.difficulty
---

Post content...
```

**Page Front Matter:**
```yaml
---
title: "About"
slug: "about"
date: 2026-01-24T14:30:00+09:00

draft: false
---

Page content...
```

**Field Descriptions:**
- `title` (required): Title
- `date` (required): Publish date
- `slug` (optional): URL slug (auto-generated from title if not specified)
- `updated` (optional): Update date (used for sitemap's lastmod)
- `taxonomies` (optional): Array of values for each taxonomy defined in theme.yaml
  - key is the name defined in theme.yaml's taxonomies
  - value is an array of values for that taxonomy
- `summary` (optional): Post summary
- `description` (optional): Page description for SEO (used in meta description tag)
- `thumbnail` (optional): Thumbnail image path
- `language` (optional): Post language code (default: site language)
- `draft` (optional): Draft status (excluded from build if true, default: false)
- **Custom fields (extra)**: All fields not defined above are stored in the `extra` object, accessible in templates as `post.extra.field_name` or `page.extra.field_name`

## Custom Fields (extra)

Define additional fields in Front Matter to freely use them in templates.

**Front Matter Example:**
```yaml
---
title: "Post Title"
date: 2026-01-24T14:30:00+09:00

# Custom fields
author: "John Doe"
reading_time: 5
difficulty: "Intermediate"
featured: true
---
```

**Template Usage (post.html):**
```jinja
{% if post.extra.author %}
  <span class="author">Author: {{ post.extra.author }}</span>
{% endif %}

{% if post.extra.reading_time %}
  <span class="reading-time">Reading time: {{ post.extra.reading_time }} min</span>
{% endif %}

{% if post.extra.difficulty %}
  <span class="difficulty">Difficulty: {{ post.extra.difficulty }}</span>
{% endif %}

{% if post.extra.featured %}
  <span class="badge">✨ Featured</span>
{% endif %}
```

This feature allows you to add various metadata per post without modifying the theme.

## Theme Customization

Themes use the Jinja template engine. For detailed template variables and usage, refer to [docs/template-context.en.md](docs/template-context.en.md).

**Template Files:**
- `home.html` - Home page (latest posts list)
- `post.html` - Individual post page
- `page.html` - Individual page
- `list.html` - List page (tags/categories/archives)
- `base.html` - Base layout

**Main Template Variables:**

**Common Variables (all templates):**
- `{{ site.title }}` - Blog name
- `{{ site.base_url }}` - Blog base URL
- `{{ site.description }}` - Blog description
- `{{ site.author }}` - Author name
- `{{ site.recent_posts }}` - Recent posts list (up to theme.yaml's recent_posts.count)
  - Each item includes: title, url, date, taxonomies, summary, thumbnail
- `{{ site.taxonomies }}` - All taxonomy map (BTreeMap<String, Vec<TaxonomyItem>>)
  - Auto-generated for each taxonomy defined in theme.yaml
  - e.g., `{{ site.taxonomies.tags }}` - Tag list (when tags are defined in theme.yaml)
  - e.g., `{{ site.taxonomies.categories }}` - Category list (when categories are defined in theme.yaml)
  - Each item includes: label, url, count
- `{{ site.archives }}` - Archive list
  - Each item: kind ("yearly"/"monthly"/"daily"), label, year, month (Optional), day (Optional), url, count

**home.html specific:**
- `{{ home.title }}` - Home page title
- `{{ home.posts }}` - Latest posts list (title, url, date, summary, thumbnail)
- `{{ home.pagination }}` - Pagination info
  - `page`, `per_page`, `total_items`, `total_pages` - Page information
  - `has_prev`, `has_next` - Previous/next page existence
  - `prev`, `next` - Previous/next page links (title, url)
  - `first`, `last` - First/last page links (title, url)
  - `pages` - Page number list (number, url, is_current)

**list.html specific:**
- `{{ list.title }}` - List title (e.g., "Tag: Rust", "January 2026")
- `{{ list.posts }}` - Filtered posts list (title, url, date, summary, thumbnail)
- `{{ list.list_kind }}` - List type
- `{{ list.pagination }}` - Pagination info (same structure as home.pagination)

**post.html specific:**
- `{{ post.title }}` - Post title
- `{{ post.url }}` - Post URL
- `{{ post.date }}` - Publish date
- `{{ post.content_html }}` - HTML-converted content (code highlighting applied)
- `{{ post.taxonomies }}` - Post's taxonomy map (BTreeMap)
  - Auto-generated for each taxonomy specified in Front Matter
  - e.g., `{{ post.taxonomies.tags }}` - Post's tag list (label, url)
  - e.g., `{{ post.taxonomies.categories }}` - Post's category list (label, url)
  - Each item includes: label, url
- `{{ post.summary }}` - Summary
- `{{ post.thumbnail }}` - Thumbnail image
- `{{ post.prev }}` - Previous post link (title, url)
- `{{ post.next }}` - Next post link (title, url)

**page.html specific:**
- `{{ page.title }}` - Page title
- `{{ page.url }}` - Page URL
- `{{ page.date }}` - Publish date
- `{{ page.content_html }}` - HTML-converted content

**Template Examples:**

```jinja
{# post.html example #}
<article>
  <h1>{{ post.title }}</h1>
  <time>{{ post.date }}</time>

  <div class="content">
    {{ post.content_html | safe }}
  </div>

  {# When tags are defined in theme.yaml #}
  {% if post.taxonomies.tags %}
    <div class="tags">
      {% for tag in post.taxonomies.tags %}
        <a href="{{ tag.url }}">{{ tag.label }}</a>
      {% endfor %}
    </div>
  {% endif %}

  <nav class="post-nav">
    {% if post.prev %}
      <a href="{{ post.prev.url }}">← {{ post.prev.title }}</a>
    {% endif %}
    {% if post.next %}
      <a href="{{ post.next.url }}">{{ post.next.title }} →</a>
    {% endif %}
  </nav>
</article>

{# home.html or list.html - pagination example #}
{% if home.pagination.total_pages > 1 %}
  <nav class="pagination">
    {% if home.pagination.has_prev %}
      <a href="{{ home.pagination.prev.url }}">← Previous</a>
    {% endif %}

    {% for page in home.pagination.pages %}
      {% if page.is_current %}
        <strong>{{ page.number }}</strong>
      {% else %}
        <a href="{{ page.url }}">{{ page.number }}</a>
      {% endif %}
    {% endfor %}

    {% if home.pagination.has_next %}
      <a href="{{ home.pagination.next.url }}">Next →</a>
    {% endif %}
  </nav>
{% endif %}

{# Sidebar (common - base.html) #}
<aside>
  {# Show only taxonomies defined in theme.yaml #}
  {% if site.taxonomies.tags %}
    <h3>Tags</h3>
    <ul>
      {% for tag in site.taxonomies.tags %}
        <li>
          <a href="{{ tag.url }}">{{ tag.label }} ({{ tag.count }})</a>
        </li>
      {% endfor %}
    </ul>
  {% endif %}

  {% if site.taxonomies.categories %}
    <h3>Categories</h3>
    <ul>
      {% for category in site.taxonomies.categories %}
        <li>
          <a href="{{ category.url }}">{{ category.label }} ({{ category.count }})</a>
        </li>
      {% endfor %}
    </ul>
  {% endif %}

  <h3>Archives</h3>
  <ul>
    {% for archive in site.archives %}
      <li>
        <a href="{{ archive.url }}">{{ archive.label }} ({{ archive.count }})</a>
      </li>
    {% endfor %}
  </ul>
</aside>
```

## Project Structure

```
myblog/
├── blogconfig.yaml         # Blog global configuration
├── content/                # Content folder
│   ├── posts/              # Posts (.md files)
│   │   └── 2026-01-24-first-post.md
│   ├── pages/              # Pages (.md files)
│   │   └── About.md
│   ├── images/             # Images (copied to public/images)
│   │   └── profile.jpg
│   └── data/               # Additional data files (copied to public/data)
│       └── example.json
├── themes/                 # Theme folder
│   └── default/            # Default theme
│       ├── theme.yaml      # Theme configuration
│       ├── assets/         # Static files
│       │   ├── css/
│       │   │   └── style.css
│       │   ├── js/
│       │   │   └── main.js
│       │   └── images/
│       │       └── logo.png
│       └── templates/      # Template files
│           ├── base.html
│           ├── home.html
│           ├── post.html
│           ├── page.html
│           └── list.html
└── public/                 # Build output (auto-generated)
    ├── index.html
    ├── page/               # Home pagination (from page 2)
    ├── posts/              # Individual post pages
    ├── pages/              # Individual static pages
    ├── tags/               # When tags are defined in theme.yaml
    ├── categories/         # When categories are defined in theme.yaml
    ├── archives/           # Archives (Yearly/Monthly/Daily, auto-generated)
    ├── assets/             # Theme static files
    ├── images/             # content/images copy
    ├── data/               # content/data copy
    ├── rss.xml
    ├── sitemap.xml
    └── robots.txt
```

## Deployment

Once the build is complete, upload the contents of the directory specified in `build.output_dir` of `blogconfig.yaml` (default: `public/`) to your web server or hosting service.

- **GitHub Pages**: Push the `public/` folder to the gh-pages branch
- **Netlify, Vercel**: Deploy the `public/` folder with static file hosting
- **Self-hosted**: Copy to the Document Root of web servers like Nginx or Apache

## License

This project is distributed under the MIT License. For details, refer to the [LICENSE](LICENSE) file.

## Contact

- **Issues**: [GitHub Issues](https://github.com/XeroNicHS/xnbloggen/issues)
- **Email**: janghs1117@gmail.com
- **Blog**: https://xeronichs.github.io

---

**Made with ❤️ by XeroNicHS**