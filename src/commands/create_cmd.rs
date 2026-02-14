// src/commands/create.rs

use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::utils::output;

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("IO error\n  Path: {path}\n  Reason: {source}")]
    Io { path: PathBuf, source: std::io::Error },

    #[error("Project directory already exists\n  Path: {path}")]
    ProjectAlreadyExists{ path: PathBuf },
}

pub fn run(root: &str) -> Result<(), CreateError> {
    let project_path = PathBuf::from(root);
    let config_path = project_path.join("blogconfig.yaml");

    if config_path.is_file() {
        return Err(CreateError::ProjectAlreadyExists{ path: project_path.clone() });
    }

    if !project_path.is_dir() {
        output::info(&format!("Creating new blog project '{}'", project_path.display()));

        // Create project root directory    
        create_dir_logged(&project_path.as_path())?;

        // Create default blogconfig.yml
        let default_config = r#"# xnBlogGen site configuration

site:
  name: "My Blog"
  base_url: "http://localhost:8000"
  path: ""
  description: "My personal blog powered by xnBlogGen"
  language: "en"

author:
  name: ""
  email: ""

theme:
  name: "default"

permalinks:
  post: "/posts/:slug/"
  page: "/pages/:slug/"

build:
  output_dir: "public"
  clean: true
  include_drafts: false
  rss: true
  rss_max_items: 20
  sitemap: true
  robots_txt: true
"#;
        create_file_logged(&config_path.as_path(), default_config)?;

        eprintln!();
        output::step("Creating project structure...");

        // Create content directories/subdirectories
        let content_dirs = [
            "content",
            "content/posts",
            "content/pages",
            "content/images",
            "content/data",
        ];

        for dir in &content_dirs {
            create_dir_logged(&project_path.join(dir).as_path())?;
        }

        // Create themes directory
        eprintln!();
        output::step("Creating default theme...");

        // Create themes root directory
        let theme_rootdirs = [
            "themes",
            "themes/default",
        ];
        for dir in &theme_rootdirs {
            create_dir_logged(&project_path.join(dir).as_path())?;
        }

        // Create default theme.yml
        let default_theme_config = r#"meta:
  name: "default"
  version: "1.0.0"
  author: "xnBlogGen"
  description: "Minimal starter theme for xnBlogGen"

template_default:
  home: "home.html"
  list: "list.html"
  post: "post.html"
  page: "page.html"

template_extra: []

pagination:
  default: 10

taxonomies:
  - name: "tags"
    label: "Tag"
    permalink: "/tags/:slug/"
    per_page: 10
    
  - name: "categories"
    label: "Category"
    permalink: "/categories/:slug/"
    per_page: 10

archives:
  - kind: "Monthly"
    permalink: "/archives/:year/:month/"
    per_page: 10

recent_posts:
    count: 10

# Custom theme settings (optional)
# Any fields added here are accessible in templates via {{ site.theme.field_name }}
# Examples: colors, fonts, social_links, features, etc.
"#;
        create_file_logged(&project_path.join("themes/default/theme.yaml").as_path(), default_theme_config)?;

        // Create themes subdirectories
        let theme_subdirs = [
            ("themes/default/assets"),
            ("themes/default/assets/css"),
            ("themes/default/assets/js"),
            ("themes/default/assets/images"),
            ("themes/default/templates"),
        ];

        for dir in &theme_subdirs {
            create_dir_logged(&project_path.join(dir).as_path())?;
        }

        // Create template files       
        let base_html_template = r#"<!DOCTYPE html>
<html lang="{{ site.language }}">
<head>
  <title>{% block title %}{{ site.title }}{% endblock %}</title>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  {% if site.description %}
  <meta name="description" content="{{ site.description }}">
  {% endif %}

  <link rel="stylesheet" href="/assets/css/style.css">
</head>
<body>
  <header>
    <h1><a href="/">{{ site.title }}</a></h1>
    {% if site.description %}
      <p>{{ site.description }}</p>
    {% endif %}
  </header>

  <div id="layout">
    <main>
      {% block content %}{% endblock %}
    </main>

    <aside>
      {% block sidebar %}{% endblock %}
    </aside>
  </div>

  <footer>
    <p>&copy; {{ site.title }}</p>
  </footer>
</body>
</html>
"#;
        
        let home_html_template = r#"{% extends "base.html" %}

{% block content %}
  <h2>Latest posts</h2>

  <ul class="post-list">
  {% for post in home.posts %}
    <li>
      <a href="{{ post.url | safe }}">{{ post.title }}</a>
      <small>{{ post.date }}</small>
    </li>
  {% endfor %}
  </ul>

  {% set pagination = home.pagination %}
  {% if pagination and pagination.total_pages > 1 %}
    <nav class="pagination">
      {% if pagination.prev %}
        <a href="{{ pagination.prev.url | safe }}">← Prev</a>
      {% endif %}
      {% if pagination.next %}
        <a href="{{ pagination.next.url | safe }}">Next →</a>
      {% endif %}
    </nav>
  {% endif %}
{% endblock %}
"#;

        let list_html_template = r#"{% extends "base.html" %}

{% block content %}
  <h2>{{ list.title }}</h2>

  <ul class="post-list">
  {% for post in list.posts %}
    <li>
      <a href="{{ post.url | safe }}">{{ post.title }}</a>
      <small>{{ post.date }}</small>
    </li>
  {% endfor %}
  </ul>

  {% set pagination = list.pagination %}
  {% if pagination and pagination.total_pages > 1 %}
    <nav class="pagination">
      {% if pagination.prev %}
        <a href="{{ pagination.prev.url | safe }}">← Prev</a>
      {% endif %}
      {% if pagination.next %}
        <a href="{{ pagination.next.url | safe }}">Next →</a>
      {% endif %}
    </nav>
  {% endif %}
{% endblock %}
"#;
        
        let post_html_template = r#"{% extends "base.html" %}

{% block content %}
  <article class="post">
    <h1>{{ post.title }}</h1>

    <p class="post-meta">
      {{ post.date }}
      {% if post.taxonomies.categories %}
        | Categories:
        {% for c in post.taxonomies.categories %}
          <a href="{{ c.url | safe }}">{{ c.label }}</a>
        {% endfor %}
      {% endif %}
    </p>

    <div class="post-body">
      {{ post.content_html | safe }}
    </div>

    {% if post.taxonomies.tags %}
      <p class="post-tags">
        Tags:
        {% for t in post.taxonomies.tags %}
          <a href="{{ t.url | safe }}">{{ t.label }}</a>
        {% endfor %}
      </p>
    {% endif %}

    {% if post.prev or post.next %}
    <nav class="post-nav">
      {% if post.prev %}
        <a href="{{ post.prev.url | safe }}">← {{ post.prev.title }}</a>
      {% endif %}
      {% if post.next %}
        <a href="{{ post.next.url | safe }}">{{ post.next.title }} →</a>
      {% endif %}
    </nav>
    {% endif %}
  </article>
{% endblock %}
"#;
        
        let page_html_template = r#"{% extends "base.html" %}

{% block content %}
  <article class="page">
    <h1>{{ page.title }}</h1>

    <div class="page-body">
      {{ page.content_html | safe }}
    </div>
  </article>
{% endblock %}

"#;

        // Create theme files (theme.yml and template files)
        let theme_files = [
            ("themes/default/templates/base.html", base_html_template),
            ("themes/default/templates/home.html", home_html_template),
            ("themes/default/templates/list.html", list_html_template),
            ("themes/default/templates/post.html", post_html_template),
            ("themes/default/templates/page.html", page_html_template),
        ];

        for (file_path, content) in &theme_files {
            create_file_logged(&project_path.join(file_path).as_path(), content)?;
        }

        eprintln!();
        output::success(&format!("Project '{}' created successfully", project_path.display()));
    }

    Ok(())
}

fn create_dir_logged(path: &Path) -> Result<(), CreateError> {
    fs::create_dir_all(path).map_err(|e| CreateError::Io { path: path.to_path_buf(), source: e })?;

    let mut dir_path_display = path_to_log_slash(path);
    if !dir_path_display.ends_with('/') {
        dir_path_display.push('/');
    }

    output::print_check(&dir_path_display);
    Ok(())
}

fn create_file_logged(path: &Path, content: &str) -> Result<(), CreateError> {
    fs::write(path, content).map_err(|e| CreateError::Io { path: path.to_path_buf(), source: e })?;

    let file_path_display = path_to_log_slash(path);

    output::print_file(&file_path_display);
    Ok(())
}

fn path_to_log_slash(path: &Path) -> String {
    let path_str = path.to_string_lossy().to_string();
    path_str.replace(std::path::MAIN_SEPARATOR, "/")
}