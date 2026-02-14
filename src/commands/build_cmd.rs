// src/commands/build.rs

use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;
use minijinja::{Environment, context};

use crate::config::blogconfig::{BlogConfig, BlogConfigError};
use crate::config::theme::{ThemePackage, ThemeError, ArchiveKind};
use crate::context::context_builder;
use crate::content::content_source::{ContentKind, ContentSource};
use crate::content::content_loader::{load_all_contents, ContentLoaderError};
use crate::context::common_context::SiteContext;
use crate::context::content_context::ContentContext;
use crate::context::list_context::{ListKind, PostListItem};
use crate::utils::output;


#[derive(Error, Debug)]
pub enum BuildError {
    #[error("IO error\n  Path: {path}\n  Reason: {source}")]
    Io { path: PathBuf, source: std::io::Error },

    #[error("Blog project not found\n  Path: {path}\n  Expected: blogconfig.yaml")]
    ProjectNotFound{ path: PathBuf },

    #[error("Content directory not found\n  Path: {path}\n  Expected: content/, content/posts/, or content/pages/")]
    ContentDirNotFound{ path: PathBuf },

    #[error("Template not found\n  Template: {template}")]
    TemplateNotFound { template: String },

    #[error("Data conversion error\n  Details: {0}")]
    ConvertError(String),

    #[error(transparent)]
    BlogConfigError(#[from] BlogConfigError),

    #[error(transparent)]
    ThemeError(#[from] ThemeError),

    #[error(transparent)]
    ContentLoaderError(#[from] ContentLoaderError),
}

pub fn run(root: &str) -> Result<(), BuildError> {
    let project_path = PathBuf::from(root);
    let config_path = project_path.join("blogconfig.yaml");

    if !config_path.is_file() {
        return Err(BuildError::ProjectNotFound { path: project_path.clone() });
    }

    // Load blog configuration
    let blog_config = BlogConfig::load_from_file(&config_path)
        .map_err(|e| BuildError::BlogConfigError(e))?;

    // Load theme data
    let theme_package = ThemePackage::load_from_dir(&project_path.join("themes")
        .join(&blog_config.theme.name))
        .map_err(|e| BuildError::ThemeError(e))?;

    // Load templates
    let mut template_env = Environment::new();
    template_env.set_loader(minijinja::path_loader(&theme_package.templates_dir));

    output::info(&format!("Building blog '{}' with theme '{}'", blog_config.site.name, theme_package.name));

    // output directory
    let output_dir = project_path.join(&blog_config.build.output_dir);
    let output_dir_index = output_dir.join("index.html");

    // Clean output directory if needed
    if blog_config.build.clean && output_dir.is_dir() && output_dir_index.is_file(){
        output::step("Cleaning output directory...");
        fs::remove_dir_all(&output_dir)
            .map_err(|e| BuildError::Io { path: output_dir.clone(), source: e })?;
    }

    // Create output directory if it doesn't exist
    if !output_dir.is_dir() {
        output::step("Creating output directory...");
        fs::create_dir_all(&output_dir)
            .map_err(|e| BuildError::Io { path: output_dir.clone(), source: e })?;
    }
    output::info("Output directory:");
    output::print_path(&output_dir.display().to_string());

    // Check content directories
    let content_dir = project_path.join("content");
    let content_post_dir = content_dir.join("posts");
    let content_page_dir = content_dir.join("pages");    
    if !content_dir.is_dir() || !content_post_dir.is_dir() || !content_page_dir.is_dir() {
        return Err(BuildError::ContentDirNotFound { path: project_path.clone() });
    }

    //------------------------------------------------------------------------------
    // Load all contents
    //------------------------------------------------------------------------------
    let all_contents = load_all_contents(&content_dir)
        .map_err(|e| BuildError::ContentLoaderError(e))?;
    output::info(&format!("{} content(s) loaded", all_contents.len()));

    //------------------------------------------------------------------------------
    // Filter contents to render
    //------------------------------------------------------------------------------
    let (mut render_posts, mut render_pages): (Vec<&ContentSource>, Vec<&ContentSource>) = all_contents
        .iter()
        .filter(|content| blog_config.build.include_drafts || !content.front_matter.draft)
        .partition(|content| content.kind == ContentKind::Post);

    // Posts: sort by date
    render_posts.sort_by(|a, b| b.front_matter.date.cmp(&a.front_matter.date));
    output::info(&format!("{} post(s) to render", render_posts.len()));

    // Pages: sort by title
    render_pages.sort_by(|a, b| a.front_matter.title.cmp(&b.front_matter.title));
    output::info(&format!("{} page(s) to render", render_pages.len()));

    //------------------------------------------------------------------------------
    // Build taxonomies & archives index
    //------------------------------------------------------------------------------
    let taxonomies_index = context_builder::build_taxonomies_index(&render_posts, &theme_package.manifest.taxonomies);
    let archives_index = context_builder::build_archives_index(&render_posts, &theme_package.manifest.archives);

    //------------------------------------------------------------------------------
    // Build posts & pages contexts
    //------------------------------------------------------------------------------
    let mut post_contexts = context_builder::build_content_contexts(&blog_config, &render_posts, &taxonomies_index);
    context_builder::link_prev_next_posts(&mut post_contexts);

    let page_contexts = context_builder::build_content_contexts(&blog_config, &render_pages, &taxonomies_index);  

    //------------------------------------------------------------------------------
    // Build site context
    //------------------------------------------------------------------------------
    let site_context = context_builder::build_site_context(
        &blog_config,         
        &taxonomies_index,
        &archives_index,
        &post_contexts.iter()
            .take(theme_package.manifest.recent_posts.count)
            .map(|post| context_builder::build_post_list_item(post)).collect(),
        &theme_package.manifest.others,
    );

    //------------------------------------------------------------------------------
    // Rendering posts and pages
    //------------------------------------------------------------------------------
    let render_content_tasks = [
        ("post(s)", &post_contexts, &render_posts, theme_package.manifest.template_default.post),
        ("page(s)", &page_contexts, &render_pages, theme_package.manifest.template_default.page),
    ];

    for (label, contexts, sources, template_name) in &render_content_tasks {
        let content_template = template_env.get_template(template_name.as_str())
            .map_err(|_| BuildError::TemplateNotFound { template: template_name.clone() })?;

        render_contents(&content_template, contexts, sources, &site_context, &output_dir)?;

        output::success(&format!("{} {} rendered", contexts.len(), label));
    }

    //------------------------------------------------------------------------------
    // Rendering taxonomies & archives pages
    //------------------------------------------------------------------------------
    let list_template_name = theme_package.manifest.template_default.list.clone();
    let list_template = template_env.get_template(list_template_name.as_str())
        .map_err(|_| BuildError::TemplateNotFound { template: list_template_name.clone() })?;

    // Taxonomies
    for taxonomy_config in &theme_package.manifest.taxonomies {
        let post_contexts_by_term = context_builder::group_posts_by_taxonomy(
            &post_contexts,
            &taxonomy_config.name,
        );

        for (term, contexts) in &post_contexts_by_term {
            if contexts.is_empty() {
                continue;
            }

            let slug = context_builder::slugify(term);
            let base_url = taxonomy_config.permalink.replace(":slug", &slug);
            let title = format!("{}: {}", taxonomy_config.label, term);
            let list_kind = ListKind::Taxonomy {
                name: taxonomy_config.name.clone(),
                slug: slug,
            };

            render_list(&list_template,
                contexts,
                &base_url,
                taxonomy_config.per_page,
                &title,
                &site_context,
                &list_kind,
                &output_dir,
            )?;
        }
        output::success(&format!("{} taxonomy term(s) rendered for '{}'", post_contexts_by_term.len(), taxonomy_config.name));
    }

    // Archives
    for archive_config in &theme_package.manifest.archives {
        let post_contexts_by_archive = context_builder::group_posts_by_archive(&post_contexts, &archive_config.kind);

        for (archive, contexts) in &post_contexts_by_archive {
            if contexts.is_empty() {
                continue;
            }

            let (base_url, title) = match archive_config.kind {
                ArchiveKind::Yearly => {
                    let year = archive.year;
                    (                    
                    archive_config.permalink.replace(":year", &format!("{:04}", year))
                        .replace("/:month", "")
                        .replace("/:day", "")
                    , format!("Archive: {:04}", year)
                    )
                },
                ArchiveKind::Monthly => {
                    let year = archive.year;
                    let month = archive.month.unwrap_or(1);
                    (
                    archive_config.permalink.replace(":year", &format!("{:04}", year))
                        .replace(":month", &format!("{:02}", month))
                        .replace("/:day", "")
                    , format!("Archive: {:04}-{:02}", year, month)
                    )   
                },
                ArchiveKind::Daily => {
                    let year = archive.year;
                    let month = archive.month.unwrap_or(1);
                    let day = archive.day.unwrap_or(1);
                    (
                    archive_config.permalink.replace(":year", &format!("{:04}", year))
                        .replace(":month", &format!("{:02}", month))
                        .replace(":day", &format!("{:02}", day))
                    , format!("Archive: {:04}-{:02}-{:02}", year, month, day)
                    )                    
                },
            };

            let list_kind = ListKind::Archive { year: archive.year, month: archive.month, day: archive.day };

            render_list(&list_template,
                contexts,
                &base_url,
                archive_config.per_page,
                &title,
                &site_context,
                &list_kind,
                &output_dir,
            )?;
        }
        output::success(&format!("{} archive(s) rendered for '{:?}'", post_contexts_by_archive.len(), archive_config.kind));
    }
    
    //------------------------------------------------------------------------------
    // Rendering home page
    //------------------------------------------------------------------------------
    let home_template_name = theme_package.manifest.template_default.home.clone();
    let home_template = template_env.get_template(home_template_name.as_str())
        .map_err(|_| BuildError::TemplateNotFound { template: home_template_name.clone() })?;

    let base_url = "/";
    let title = "Home";
    let list_kind = ListKind::Home;

    render_list(
        &home_template,
        &post_contexts,
        base_url,
        theme_package.manifest.pagination.default,
        title,
        &site_context,
        &list_kind,
        &output_dir,
    )?;
    output::success("Home page rendered");

    //------------------------------------------------------------------------------
    // Rendering extra templates
    //------------------------------------------------------------------------------    
    for extra_template in &theme_package.manifest.template_extra {
        let template_name = extra_template.file.clone();
        let template = template_env.get_template(template_name.as_str())
            .map_err(|_| BuildError::TemplateNotFound { template: template_name.clone() })?;

        let extra_html = template.render(context! {
            site => site_context,
        }).map_err(|e| BuildError::ConvertError(format!("Template rendering error: {}", e)))?;

        write_file(&extra_template.output, &output_dir, &extra_template.url, &extra_html)?;

        output::success(&format!("Extra template rendered: {} -> {}", extra_template.url, extra_template.output));
    }

    //------------------------------------------------------------------------------
    // Copy static files
    //------------------------------------------------------------------------------    
    let copy_tasks = [
        (&theme_package.assets_dir, output_dir.join("assets"), "Assets"),
        (&content_dir.join("images"), output_dir.join("images"), "Images"),
        (&content_dir.join("data"), output_dir.join("data"), "Data"),
    ];

    for (src, dest, label) in copy_tasks {
        if src.is_dir() {
            copy_dir_recursive(src, &dest)?;
            output::success(&format!("{label} copied successfully"));
        } else {
            output::info(&format!("No {label} directory found, skipping copy"));
        }
    }    

    //------------------------------------------------------------------------------
    // Generate RSS feed
    //------------------------------------------------------------------------------
    if blog_config.build.rss {
        output::step("Generating RSS feed...");

        let rss_xml = generate_rss(&blog_config, &site_context, &post_contexts);
        write_file("rss.xml", &output_dir, "/", &rss_xml)?;

        output::success("RSS feed generated: rss.xml");
    }
    
    //------------------------------------------------------------------------------
    // Generate sitemap.xml
    //------------------------------------------------------------------------------
    if blog_config.build.sitemap {
        output::step("Generating sitemap...");

        let sitemap_xml = generate_sitemap(&site_context, &post_contexts, &page_contexts);
        write_file("sitemap.xml", &output_dir, "/", &sitemap_xml)?;

        output::success("Sitemap generated: sitemap.xml");
    }

    //------------------------------------------------------------------------------
    // Generate robots.txt
    //------------------------------------------------------------------------------
    if blog_config.build.robots_txt {
        output::step("Generating robots.txt...");

        let robots_txt = format!("User-agent: *\nDisallow: /data\nAllow: /\nSitemap: {}/sitemap.xml\n",
            site_context.base_url.trim_end_matches('/'));
        write_file("robots.txt", &output_dir, "/", &robots_txt)?;

        output::success("robots.txt generated");
    }

    //------------------------------------------------------------------------------
    eprintln!();
    output::success("Build completed successfully");

    Ok(())
}

// Copy directory recursively (including subdirectories)
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), BuildError> {
    // Create destination directory if it doesn't exist
    if !dst.is_dir() {
        fs::create_dir_all(dst)
            .map_err(|e| BuildError::Io { path: dst.to_path_buf(), source: e })?;
    }

    // Read source directory
    let entries = fs::read_dir(src)
        .map_err(|e| BuildError::Io { path: src.to_path_buf(), source: e })?;

    for entry in entries {
        let entry = entry
            .map_err(|e| BuildError::Io { path: src.to_path_buf(), source: e })?;
        
        let src_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        let file_type = entry.file_type()
            .map_err(|e| BuildError::Io { path: src_path.clone(), source: e })?;

        if file_type.is_dir() {
            // Recursively copy subdirectory
            copy_dir_recursive(&src_path, &dst_path)?;
        } else if file_type.is_file() {
            // Copy file
            fs::copy(&src_path, &dst_path)
                .map_err(|e| BuildError::Io { path: src_path.clone(), source: e })?;
        } else if file_type.is_symlink() {
            // Ignore symlinks
            output::warning(&format!("Skipping symlink at {:?}", src_path.display()));
        }
    }

    Ok(())
}

// Function to write HTML content to index.html under given URL path
fn write_file(filename: &str, output_dir: &Path, url_path: &str, data: &str) -> Result<(), BuildError> {
    let dir = output_dir.join(url_path.trim_start_matches('/').trim_end_matches('/'));
    fs::create_dir_all(&dir)
        .map_err(|e| BuildError::Io { path: dir.clone(), source: e })?;

    let file = dir.join(filename);
    fs::write(&file, data)
        .map_err(|e| BuildError::Io { path: file.clone(), source: e })?;

    Ok(())
}

// Generate RSS feed XML
fn generate_rss(
    blog_config: &BlogConfig,
    site_context: &SiteContext,
    post_contexts: &[ContentContext],
) -> String {
    let max_items = blog_config.build.rss_max_items.min(post_contexts.len());
    let posts = post_contexts.iter().take(max_items);
    
    let mut items = String::new();
    
    for post in posts {
        let full_url = format!("{}{}", site_context.base_url.trim_end_matches('/'), post.url);
        let pub_date = post.date.to_rfc2822();
        
        let title = escape_xml(&post.title);
        let description = post.summary.as_ref()
            .map(|s| escape_xml(s))
            .unwrap_or_else(|| "".to_string());
        
        let categories = post.taxonomies.as_ref().map_or(Vec::new(), |taxonomies| {
            taxonomies.values()
                .flatten()
                .map(|term| format!("      <category>{}</category>", escape_xml(&term.label)))
                .collect::<Vec<_>>()
        }).join("\n");
        
        items.push_str(&format!(r#"
    <item>
      <title>{}</title>
      <link>{}</link>
      <guid isPermaLink="true">{}</guid>
      <pubDate>{}</pubDate>
      <description>{}</description>
{}
    </item>"#, 
            title, full_url, full_url, pub_date, description, categories
        ));
    }
    
    let build_date = chrono::Utc::now().to_rfc2822();
    
    format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>{}</title>
    <link>{}</link>
    <description>{}</description>
    <language>{}</language>
    <lastBuildDate>{}</lastBuildDate>
    <atom:link href="{}/rss.xml" rel="self" type="application/rss+xml" />
{}</channel>
</rss>"#,
        escape_xml(&site_context.title),
        site_context.base_url.trim_end_matches('/'),
        escape_xml(&site_context.description),
        site_context.language.clone(),
        build_date,
        site_context.base_url.trim_end_matches('/'),
        items
    )
}

// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}

// Generate sitemap.xml
fn generate_sitemap(
    site_context: &SiteContext,
    post_contexts: &[ContentContext],
    page_contexts: &[ContentContext],
) -> String {
    let base_url = site_context.base_url.trim_end_matches('/');
    let mut urls = Vec::new();
    
    // Home page
    urls.push(format_sitemap_url(base_url, "/", None, "daily", "1.0"));
    
    // Posts
    for post in post_contexts {
        let lastmod = Some(post.updated.as_ref().unwrap_or(&post.date).to_rfc3339());
        urls.push(format_sitemap_url(base_url, &post.url, lastmod, "weekly", "0.8"));
    }
    
    // Pages
    for page in page_contexts {
        let lastmod = Some(page.updated.as_ref().unwrap_or(&page.date).to_rfc3339());
        urls.push(format_sitemap_url(base_url, &page.url, lastmod, "monthly", "0.7"));
    }

    // taxonomy pages
    for taxonomy_items in site_context.taxonomies.values() {
        for item in taxonomy_items {
            urls.push(format_sitemap_url(base_url, &item.url, None, "weekly", "0.6"));
        }
    }
    
    // Archive pages
    for archive in &site_context.archives {
        urls.push(format_sitemap_url(base_url, &archive.url, None, "monthly", "0.5"));
    }
    
    format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}
</urlset>"#, urls.join("\n"))
}

// Format single sitemap URL entry
fn format_sitemap_url(
    base_url: &str,
    path: &str,
    lastmod: Option<String>,
    changefreq: &str,
    priority: &str,
) -> String {
    let full_url = format!("{}{}", base_url, path);
    let lastmod_tag = lastmod
        .map(|date| format!("\n    <lastmod>{}</lastmod>", date))
        .unwrap_or_default();
    
    format!(r#"  <url>
    <loc>{}</loc>{}<changefreq>{}</changefreq>
    <priority>{}</priority>
  </url>"#,
        escape_xml(&full_url),
        lastmod_tag,
        changefreq,
        priority
    )
}

// Generate redirect page for /page/1/ to base URL
fn create_page1_redirect(output_dir: &Path, base_url: &str) -> Result<(), BuildError> {
    let redirect_html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta http-equiv="refresh" content="0;url={}">
    <link rel="canonical" href="{}">
    <title>Redirecting...</title>
</head>
<body>
    <p>Redirecting to <a href="{}">{}</a>...</p>
</body>
</html>"#, base_url, base_url, base_url, base_url);

    let redirect_path = format!("{}/page/1/", base_url.trim_end_matches('/'));
    write_file("index.html", output_dir, &redirect_path, &redirect_html)?;
    
    Ok(())
}

//------------------------------------------------------------------------------
fn render_contents(
    content_template: &minijinja::Template,
    contexts: &[ContentContext],
    sources: &[&ContentSource],
    site_context: &SiteContext,
    output_dir: &Path,
) -> Result<(), BuildError> {
    // contexts and sources are parallel arrays, so we can access source data (e.g., images) while rendering context
    for (context, source) in contexts.iter().zip(sources.iter()) {
        let rendered_html = match context.kind {
            ContentKind::Post => content_template.render(context! {
                site => site_context,
                post => context,
            }),
            ContentKind::Page => content_template.render(context! {
                site => site_context,
                page => context,
            }),
        }.map_err(|e| BuildError::ConvertError(format!("Template rendering error: {}", e)))?;

        write_file("index.html", &output_dir, &context.url, &rendered_html)?;

        // Copy content images if any
        if !source.images.is_empty() {
            let dest_dir = output_dir.join(context.url.trim_matches('/'));
            for img_path in &source.images {
                let filename = img_path.file_name()
                    .ok_or_else(|| BuildError::ConvertError(format!("Invalid image path: {:?}", img_path)))?;
                fs::copy(img_path, dest_dir.join(filename))
                    .map_err(|e| BuildError::Io { path: img_path.clone(), source: e })?;
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------
fn render_list(
    list_template: &minijinja::Template,
    contexts: &[ContentContext],
    base_url: &str,
    per_page: usize,
    title: &str,
    site_context: &SiteContext,
    list_kind: &ListKind,
    output_dir: &Path,
) -> Result<(), BuildError> {
    let list_items: Vec<PostListItem> = contexts.iter()
        .map(|post_ctx| context_builder::build_post_list_item(post_ctx))
        .collect();

    let paginated = context_builder::paginate_items(&list_items, per_page);
    let total_pages = paginated.len();

    for (page_num, page_items) in paginated {
        let pagination = context_builder::build_pagination(
            &base_url,
            page_num,
            per_page,
            list_items.len(),
        );

        let list_context = context_builder::build_list_context(
            title.to_string(),
            pagination.pages.iter()
                .find(|p| p.number == page_num)
                .map(|p| p.url.clone())
                .unwrap_or_else(|| base_url.to_string()),
            list_kind.clone(),
            page_items,
            pagination,
        );

        let html = match list_kind {
            ListKind::Home => list_template.render(context! {
                site => site_context,
                home => list_context,
                list => list_context,
            }),
            _ => list_template.render(context! {
                site => site_context,
                list => list_context,
            }),
        }.map_err(|e| BuildError::ConvertError(format!("Template error: {}", e)))?;

        let file_path = if page_num == 1 {
            base_url.to_string()
        } else {
            format!("{}/page/{}/", base_url.trim_end_matches('/'), page_num)
        };

        write_file("index.html", &output_dir, &file_path, &html)?;

    }
    if total_pages > 1 {
        create_page1_redirect(&output_dir, &base_url)?;
    }

    Ok(())
}