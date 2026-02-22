// src/context/context_builder.rs

use std::collections::{BTreeMap, BTreeSet};

use chrono::{Datelike};
use comrak::{markdown_to_html_with_plugins, Options, options::Plugins};
use comrak::plugins::syntect::SyntectAdapter;

use crate::config::blogconfig::{BlogConfig};
use crate::config::theme::{TaxonomyConfig, ArchiveConfig, ArchiveKind};
use crate::content::content_source::{ContentKind, ContentSource};
use crate::utils::filters::slugify;

use super::{SiteContext, TaxonomyItem, ArchiveItem, NavLink, Pagination, PageLink};
use super::{ListContext, ListKind, PostListItem};
use super::{ContentContext};


pub fn build_site_context(
    blog_config: &BlogConfig, 
    taxonomies: &BTreeMap<String, Vec<TaxonomyItem>>,
    archives: &Vec<ArchiveItem>,
    recent_posts: &Vec<PostListItem>,
    theme_others: &BTreeMap<String, serde_yaml::Value>
) -> SiteContext {
    SiteContext {
        title: blog_config.site.name.clone(),
        base_url: blog_config.site.base_url.clone(),
        path: blog_config.site.path.clone(),
        description: blog_config.site.description.clone(),
        language: blog_config.site.language.clone(),

        author: Some(blog_config.author.name.clone()),
        email: Some(blog_config.author.email.clone()),
        
        taxonomies: taxonomies.clone(),
        archives: archives.clone(),
        recent_posts: recent_posts.clone(),

        theme: theme_others.clone(),
    }
}

pub fn build_content_contexts(
    blog_config: &BlogConfig,
    contents: &[&ContentSource],
    site_taxonomies: &BTreeMap<String, Vec<TaxonomyItem>>,
) -> Vec<ContentContext> {
    let mut options = Options::default();

    // Extension options
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.header_ids = Some(String::new());
    options.extension.footnotes = true;

    // Render options
    options.render.hardbreaks = false;
    options.render.github_pre_lang = true;
    options.render.r#unsafe = true;

    let adapter = SyntectAdapter::new(None);

    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    contents.iter().map(|content| {
        let dt = &content.front_matter.date;
        let y = format!("{:04}", dt.year());
        let m = format!("{:02}", dt.month());
        let d = format!("{:02}", dt.day());

        let slug = content.front_matter.slug.clone()
            .unwrap_or_else(|| slugify(&content.front_matter.title));
                
        let pattern = match content.kind {
            ContentKind::Post => &blog_config.permalinks.post,
            ContentKind::Page => &blog_config.permalinks.page,
        };

        let url_path = build_url_path(pattern, &y, &m, &d, &slug);

        let taxonomies = extract_post_taxonomies(
            &content.front_matter.taxonomies,
            site_taxonomies,
        );

        let absolute_thumbnail = content.front_matter.thumbnail.as_ref()
            .map(|t| resolve_thumbnail_path(t, &url_path));

        let content_html = markdown_to_html_with_plugins(&content.body.markdown, &options, &plugins);

        ContentContext{
            kind: content.kind,

            title: content.front_matter.title.clone(),
            url: url_path,
            description: content.front_matter.description.clone(),
            language: content.front_matter.language.clone(),

            date: content.front_matter.date.clone(),
            updated: content.front_matter.updated.clone(),

            content_html: content_html,

            extra: content.front_matter.extra.clone(),

            taxonomies: if taxonomies.is_empty() { None } else { Some(taxonomies) },
            summary: content.front_matter.summary.clone(),
            thumbnail: absolute_thumbnail,
            prev: None,
            next: None,
        }
    }).collect()
}

pub fn build_list_context(
    title: String,
    url: String,
    list_kind: ListKind,
    posts: Vec<PostListItem>,
    pagination: Pagination
) -> ListContext {
    ListContext {
        title: title,
        url: url,
        description: None,

        list_kind: list_kind,

        posts: posts,
        pagination: pagination,
    }
}

pub fn build_post_list_item(post: &ContentContext) -> PostListItem {
    PostListItem {
        title: post.title.clone(),
        url: post.url.clone(),
        date: post.date.clone(),
        taxonomies: post.taxonomies.clone(),
        summary: post.summary.clone(),
        thumbnail: post.thumbnail.clone(),
    }
}

// Link previous and next posts in the list
pub fn link_prev_next_posts(contexts: &mut [ContentContext]) {
    for i in 0..contexts.len() {
        // Set prev link (older post)
        if i < contexts.len() - 1 {
            contexts[i].prev = Some(NavLink {
                title: contexts[i + 1].title.clone(),
                url: contexts[i + 1].url.clone(),
            });
        }

        // Set next link (newer post)
        if i > 0 {
            contexts[i].next = Some(NavLink {
                title: contexts[i - 1].title.clone(),
                url: contexts[i - 1].url.clone(),
            });
        }
    }
}



//------------------------------------------------------------------------------
// Internal helper functions
//------------------------------------------------------------------------------
// Build URL path from pattern
fn build_url_path(pattern: &str, y: &str, m: &str, d: &str, slug: &str) -> String {
    pattern.replace(":year", y)
           .replace(":month", m)
           .replace(":day", d)
           .replace(":slug", slug)
}

// Resolve thumbnail path
fn resolve_thumbnail_path(thumbnail: &str, post_url: &str) -> String {
    if thumbnail.starts_with("./") {
        format!("{}{}", post_url, &thumbnail[2..])
    } else if thumbnail.starts_with("/") {
        thumbnail.to_string()
    } else {
        format!("{}{}", post_url, thumbnail)
    }
}

//------------------------------------------------------------------------------
// Taxonomy/Archive Index Builders
//------------------------------------------------------------------------------
// Build all taxonomies index
pub fn build_taxonomies_index(
    posts: &[&ContentSource],
    taxonomy_configs: &[TaxonomyConfig],
) -> BTreeMap<String, Vec<TaxonomyItem>> {
    let mut taxonomies = BTreeMap::new();

    for config in taxonomy_configs {
        if !config.enabled {
            continue;
        }

        let items = build_single_taxonomy_index(posts, &config.name, &config.permalink);
        taxonomies.insert(config.name.clone(), items);
    }

    taxonomies
}

// Build index for a single taxonomy
fn build_single_taxonomy_index(
    posts: &[&ContentSource],
    taxonomy_name: &str,
    permalink: &str,
) -> Vec<TaxonomyItem> {
    let norm_key = |s: &str| s.trim().to_lowercase();
    let mut counts: BTreeMap<String, (String, usize)> = BTreeMap::new();

    for post in posts.iter() {
        if let Some(terms) = post.front_matter.taxonomies.as_ref().and_then(|m| m.get(taxonomy_name)) {
            let mut seen: BTreeSet<String> = BTreeSet::new();

            for term in terms {
                let key = norm_key(term);
                if key.is_empty() {
                    continue;
                }
                if !seen.insert(key.clone()) {
                    continue;
                }

                counts.entry(key.clone())
                    .and_modify(|v| v.1 += 1)
                    .or_insert((key.to_string(), 1));
            }
        }
    }

    let mut items: Vec<TaxonomyItem> = counts.into_iter()
        .map(|(key, (label, count))| {
            let url = permalink.replace(":slug", &slugify(&key));
            TaxonomyItem { label, url, count }
        })
        .collect();

    items.sort_by(|a, b| b.count.cmp(&a.count));
    items
}

/// Extract matching taxonomy items from site-wide index
fn extract_post_taxonomies(
    taxonomies_raw: &Option<BTreeMap<String, Vec<String>>>,
    site_taxonomies: &BTreeMap<String, Vec<TaxonomyItem>>,
) -> BTreeMap<String, Vec<TaxonomyItem>> {
    let norm_key = |s: &str| s.trim().to_lowercase();
    let mut result = BTreeMap::new();

    let raw_map = match taxonomies_raw {
        Some(m) => m,
        None => return result,
    };

    for (taxonomy_name, terms) in raw_map {
        if let Some(site_items) = site_taxonomies.get(taxonomy_name) {
            let matched: Vec<TaxonomyItem> = terms.iter()
                .filter_map(|term| {
                    let key = norm_key(term);
                    site_items.iter()
                        .find(|item| norm_key(&item.label) == key)
                        .cloned()
                })
                .collect();
            
            if !matched.is_empty() {
                result.insert(taxonomy_name.clone(), matched);
            }
        }
    }

    result
}

// Build archives index
pub fn build_archives_index(
    posts: &[&ContentSource], 
    archive_configs: &[ArchiveConfig]
) -> Vec<ArchiveItem> {
    let mut archives = Vec::new();

    for config in archive_configs {
        let items = build_single_archive_index(posts, &config.kind, &config.permalink);
        archives.extend(items);
    }

    archives
}

fn build_single_archive_index(
    posts: &[&ContentSource],
    kind: &ArchiveKind,
    permalink: &str
) -> Vec<ArchiveItem> {
    let mut counts: BTreeMap<(u32, Option<u32>, Option<u32>), usize> = BTreeMap::new();

    for post in posts.iter() {
        let dt = &post.front_matter.date;
        let year: u32 = dt.year() as u32;
        let month: u32 = dt.month();
        let day: u32 = dt.day();

        let key = match kind {
            ArchiveKind::Yearly => (year, None, None),
            ArchiveKind::Monthly => (year, Some(month), None),
            ArchiveKind::Daily => (year, Some(month), Some(day)),
        };

        *counts.entry(key).or_insert(0) += 1;
    }
    
    let mut items: Vec<ArchiveItem> = counts.into_iter()
        .map(|((year, month_opt, day_opt), count)| {
            let mut url = permalink.replace(":year", &format!("{:04}", year));
            if let Some(month) = month_opt {
                url = url.replace(":month", &format!("{:02}", month));
            }
            if let Some(day) = day_opt {
                url = url.replace(":day", &format!("{:02}", day));
            }

            let label = match (month_opt, day_opt) {
                (None, None) => format!("{:04}", year),
                (Some(month), None) => format!("{:04}-{:02}", year, month),
                (Some(month), Some(day)) => format!("{:04}-{:02}-{:02}", year, month, day),
                _ => unreachable!(),
            };

            ArchiveItem {
                label,
                kind: kind.clone(),
                year,
                month: month_opt,
                day: day_opt,
                url,
                count,
            }
        })
        .collect();

    items.sort_by(|a, b| {
        b.year.cmp(&a.year)
            .then_with(|| a.month.cmp(&b.month))
            .then_with(|| a.day.cmp(&b.day))
    });

    items
}

//------------------------------------------------------------------------------
// Taxonomy/Archive Grouping
//------------------------------------------------------------------------------
// Group posts by any taxonomy
pub fn group_posts_by_taxonomy(
    contexts: &[ContentContext],
    taxonomy_name: &str,
) -> BTreeMap<String, Vec<ContentContext>> {
    let mut taxonomy_map: BTreeMap<String, Vec<ContentContext>> = BTreeMap::new();

    for post in contexts.iter() {
        if let Some(terms) = post.taxonomies.as_ref().and_then(|t| t.get(taxonomy_name)) {
            for term in terms {
                let key = term.label.trim().to_lowercase();

                taxonomy_map.entry(key)
                    .or_insert_with(Vec::new)
                    .push(post.clone());
            }
        }
    }

    taxonomy_map
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArchiveKey {
    pub year: u32,
    pub month: Option<u32>,
    pub day: Option<u32>,
}

// Group posts by archive (year and month)
pub fn group_posts_by_archive(
    contexts: &[ContentContext],
    kind: &ArchiveKind,
) -> BTreeMap<ArchiveKey, Vec<ContentContext>> {
    let mut archive_map: BTreeMap<ArchiveKey, Vec<ContentContext>> = BTreeMap::new();

    for post in contexts.iter() {
        let key = match kind {
            ArchiveKind::Yearly => ArchiveKey {
                year: post.date.year() as u32,
                month: None,
                day: None,
            },
            ArchiveKind::Monthly => ArchiveKey {
                year: post.date.year() as u32,
                month: Some(post.date.month()),
                day: None,
            },
            ArchiveKind::Daily => ArchiveKey {
                year: post.date.year() as u32,
                month: Some(post.date.month()),
                day: Some(post.date.day()),
            },
        };

        archive_map.entry(key)
            .or_insert_with(Vec::new)
            .push(post.clone());
    }

    archive_map
}

//------------------------------------------------------------------------------
// Pagination helper functions
//------------------------------------------------------------------------------

// Build pagination for a list
// 
// # Arguments
// * `base_url` - Base URL for the list (e.g., "/tags/rust/")
// * `current_page` - Current page number (1-indexed)
// * `per_page` - Number of items per page (e.g., 10)
// * `total_items` - Total number of items
pub fn build_pagination(
    base_url: &str,
    current_page: usize,
    per_page: usize,
    total_items: usize,
) -> Pagination {
    let total_pages = if per_page == 0 || total_items == 0 {
        1
    } else {
        (total_items + per_page - 1) / per_page
    };

    let has_prev = current_page > 1;
    let has_next = current_page < total_pages;

    let prev = if has_prev {
        let prev_page = current_page - 1;
        let url = page_url(base_url, prev_page);
        Some(NavLink {
            title: format!("Page {}", prev_page),
            url,
        })
    } else {
        None
    };

    let next = if has_next {
        let next_page = current_page + 1;
        let url = page_url(base_url, next_page);
        Some(NavLink {
            title: format!("Page {}", next_page),
            url,
        })
    } else {
        None
    };

    let first = if current_page > 1 {
        Some(NavLink {
            title: "First".to_string(),
            url: base_url.to_string(),
        })
    } else {
        None
    };

    let last = if current_page < total_pages && total_pages > 1 {
        Some(NavLink {
            title: "Last".to_string(),
            url: page_url(base_url, total_pages),
        })
    } else {
        None
    };

    let pages = if per_page == 0 {
        vec![PageLink {
            number: 1,
            url: base_url.to_string(),
            is_current: true,
        }]
    } else {
        build_page_range(base_url, current_page, total_pages)
    };

    Pagination {
        page: current_page,
        per_page: if per_page == 0 { total_items } else { per_page },
        total_items,
        total_pages,
        has_prev,
        has_next,
        prev,
        next,
        first,
        last,
        pages,
    }
}

// Generate page number range for pagination
// Shows: [1] 2 3 ... 10 (current page in brackets)
// Logic: show first 3, last 3, and pages around current
fn build_page_range(base_url: &str, current_page: usize, total_pages: usize) -> Vec<PageLink> {
    use super::PageLink;
    
    if total_pages <= 7 {
        return (1..=total_pages)
            .map(|page_num| PageLink {
                number: page_num,
                url: page_url(base_url, page_num),
                is_current: page_num == current_page,
            })
            .collect();
    }

    let mut pages = Vec::new();
    
    for page_num in 1..=3 {
        pages.push(PageLink {
            number: page_num,
            url: page_url(base_url, page_num),
            is_current: page_num == current_page,
        });
    }

    if current_page > 6 {
        pages.push(PageLink {
            number: 0,  // 0 - display as "..."
            url: String::new(),
            is_current: false,
        });
    }

    if current_page > 3 && current_page < total_pages - 2 {
        for offset in -1..=1 {
            let page_num = (current_page as i32 + offset) as usize;
            if page_num > 3 && page_num < total_pages - 2 {
                pages.push(PageLink {
                    number: page_num,
                    url: page_url(base_url, page_num),
                    is_current: page_num == current_page,
                });
            }
        }
    }

    if current_page < total_pages - 5 {
        pages.push(PageLink {
            number: 0,
            url: String::new(),
            is_current: false,
        });
    }

    for page_num in (total_pages - 2)..=total_pages {
        if page_num > 3 {
            pages.push(PageLink {
                number: page_num,
                url: page_url(base_url, page_num),
                is_current: page_num == current_page,
            });
        }
    }

    pages
}

// Generate URL for a specific page number
// Page 1 uses the base URL, other pages use /page/N/
fn page_url(base_url: &str, page: usize) -> String {
    if page == 1 {
        base_url.to_string()
    } else {
        format!("{}/page/{}/", base_url.trim_end_matches('/'), page)
    }
}

// Paginate a list of items
// Returns a vector of (page_number, items_for_page)
pub fn paginate_items<T: Clone>(items: &[T], per_page: usize) -> Vec<(usize, Vec<T>)> {
    if items.is_empty() {
        return vec![];
    }

    if per_page == 0 {
        return vec![(1, items.to_vec())];
    }

    items.chunks(per_page)
        .enumerate()
        .map(|(idx, chunk)| (idx + 1, chunk.to_vec()))
        .collect()
}
