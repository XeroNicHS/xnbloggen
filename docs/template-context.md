# XNBlogGen 템플릿 컨텍스트 명세

> **Language**: [🇰🇷 한국어](template-context.md) | [🇺🇸 English](template-context.en.md)

이 문서는 **XNBlogGen 템플릿 엔진(Jinja)에서 사용 가능한 모든 변수와 객체**의 상세 명세입니다.

## 목차

1. [기본 원칙](#1-기본-원칙)
2. [전역 변수 (site)](#2-전역-변수-site)
3. [홈 페이지 변수 (home)](#3-홈-페이지-변수-home)
4. [목록 페이지 변수 (list)](#4-목록-페이지-변수-list)
5. [포스트 페이지 변수 (post)](#5-포스트-페이지-변수-post)
6. [일반 페이지 변수 (page)](#6-일반-페이지-변수-page)
7. [데이터 구조 명세](#7-데이터-구조-명세)
8. [템플릿별 사용 가능 변수](#8-템플릿별-사용-가능-변수)
9. [실전 예제](#9-실전-예제)
10. [템플릿 필터](#10-템플릿-필터)

---

## 1. 기본 원칙

### 변수 네이밍 규칙

- **`site`**: 사이트 전역 정보 (모든 템플릿에서 사용 가능)
- **`home`**: 홈 페이지 전용 데이터 (theme.yaml - template_default.home, 기본값 : "home.html")
- **`list`**: 목록 페이지 전용 데이터 (theme.yaml - template_default.list, 기본값 : "list.html")
- **`post`**: 개별 포스트 전용 데이터 (theme.yaml - template_default.post, 기본값 : "post.html")
- **`page`**: 개별 페이지 전용 데이터 (theme.yaml - template_default.page, 기본값 : "page.html")

### 호환성 보장

- 필드명과 타입은 버전 간 호환성을 유지합니다
- 새 필드는 추가될 수 있으나 기존 필드는 제거되지 않습니다
- HTML 콘텐츠 필드는 `_html` 접미사를 사용합니다
- 선택적 필드는 `?`로 표시됩니다

---

## 2. 전역 변수 (site)

**사용 가능 템플릿**: 모든 템플릿 (home.html, list.html, post.html, page.html, base.html 등)

**의미**: 사이트 전체 설정 및 전역 데이터

### 필드 목록

| 필드 | 타입 | 설명 |
|------|------|------|
| `title` | string | 블로그 이름 |
| `base_url` | string | 블로그 기본 URL (예: https://blog.example.com) |
| `path` | string | 서브 경로 (예: /blog, 루트이면 빈 문자열) |
| `description` | string | 블로그 설명 |
| `language` | string? | 언어 코드 (예: ko, en) |
| `author` | string? | 블로그 작성자 이름 |
| `email` | string? | 작성자 이메일 |
| `theme` | object? | 테마별 사용자 정의 설정 (theme.yaml의 추가 필드) |
| `taxonomies` | BTreeMap<string, TaxonomyItem[]> | 동적 taxonomy 맵 (theme.yaml 설정 기반) |
| `archives` | ArchiveItem[] | 전체 아카이브 목록 (연도별/월별/일별 지원) |
| `recent_posts` | PostListItem[] | 최신 포스트 목록 (기본 10개) |

**taxonomies 구조:**
- theme.yaml에 정의된 taxonomy별로 자동 생성
- 예: `site.taxonomies.tags`, `site.taxonomies.categories`, `site.taxonomies.series`
- 정의하지 않은 taxonomy는 존재하지 않음

**theme 필드:**
- theme.yaml에서 기본 필드(`meta`, `template_default`, `template_extra`, `pagination`, `taxonomies`)를 제외한 모든 사용자 정의 설정을 포함
- 예: theme.yaml에 `social_links`, `footer_text`, `color_scheme` 등의 커스텀 필드를 추가하면 `site.theme.social_links`, `site.theme.footer_text`, `site.theme.color_scheme`로 접근 가능
- 테마 제작자가 자유롭게 정의할 수 있는 확장 영역

### 사용 예제

```jinja
<header>
  <h1>{{ site.title }}</h1>
  <p>{{ site.description }}</p>
</header>

{# theme.yaml에 tags가 정의된 경우만 표시 #}
{% if site.taxonomies.tags %}
  <aside>
    <h3>태그</h3>
    <ul>
      {% for tag in site.taxonomies.tags %}
        <li><a href="{{ tag.url }}">{{ tag.label }} ({{ tag.count }})</a></li>
      {% endfor %}
    </ul>
  </aside>
{% endif %}

{# 최신 포스트 사이드바 #}
<aside>
  <h3>최근 게시물</h3>
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

### theme 필드 활용 예제

**theme.yaml에 커스텀 설정 추가:**
```yaml
# theme.yaml
name: "MyTheme"
author: "Theme Author"
version: "1.0.0"

# 기본 ThemeManifest 필드들...
template_default:
  home: "home.html"
  
# 사용자 정의 필드 (site.theme으로 접근)
social_links:
  github: "https://github.com/username"
  twitter: "https://twitter.com/username"
footer_text: "© 2026 My Blog"
color_scheme:
  primary: "#007bff"
  secondary: "#6c757d"
```

**템플릿에서 사용:**
```jinja
{# 소셜 링크 표시 #}
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

{# 푸터 텍스트 표시 #}
<footer>
  {% if site.theme.footer_text %}
    <p>{{ site.theme.footer_text }}</p>
  {% endif %}
</footer>

{# CSS 변수로 색상 설정 #}
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

## 3. 홈 페이지 변수 (home)

**사용 가능 템플릿**: `theme.yaml - template_default.home`, 기본값 : `home.html`

**의미**: 홈 페이지(인덱스) 렌더링에 필요한 데이터

> ⚠️ **중요**: `home` 변수는 실제로 `ListContext` 타입이며, `list` 변수의 **alias**입니다. 내부적으로 `ListContext`를 생성한 후 `home`이라는 이름으로 템플릿에 전달합니다. 따라서 `home`과 `list`는 완전히 동일한 객체를 가리킵니다.

### 필드 목록

> `home` 변수는 `ListContext` 타입이므로, 아래 필드들은 모두 `ListContext`의 필드입니다. 자세한 내용은 [목록 페이지 변수 (list)](#4-목록-페이지-변수-list) 섹션을 참조하세요.

| 필드 | 타입 | 설명 |
|------|------|------|
| `title` | string | 홈 페이지 제목 (일반적으로 사이트 이름) |
| `url` | string | 홈 페이지 URL (일반적으로 /) |
| `description` | string? | 홈 페이지 설명 |
| `list_kind` | ListKind | 목록 종류 (Home의 경우 `{ "type": "home" }`) |
| `posts` | PostListItem[] | 홈에 표시할 포스트 목록 |
| `pagination` | Pagination | 페이지네이션 정보 |

### 사용 예제

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
      <a href="{{ home.pagination.prev.url }}">이전</a>
    {% endif %}
    {% if home.pagination.next %}
      <a href="{{ home.pagination.next.url }}">다음</a>
    {% endif %}
  </nav>
</main>
```

---

## 4. 목록 페이지 변수 (list)

**사용 가능 템플릿**: `theme.yaml - template_default.list`, 기본값 : `list.html`

**의미**: 태그별, 카테고리별, 아카이브별 목록 페이지 렌더링 데이터

### 필드 목록

| 필드 | 타입 | 설명 |
|------|------|------|
| `title` | string | 목록 제목 (예: "태그: Rust", "2026년 1월") |
| `url` | string | 목록 페이지 URL |
| `description` | string? | 목록 페이지 설명 |
| `list_kind` | ListKind | 목록 종류 (Taxonomy, Archive) |
| `posts` | PostListItem[] | 필터링된 포스트 목록 |
| `pagination` | Pagination | 페이지네이션 정보 |

### ListKind 종류

ListKind는 다음과 같은 JSON 객체로 직렬화됩니다:

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
- `name`: taxonomy 이름 (예: "tags", "categories", "series")
- `slug`: taxonomy 값 (예: "rust", "programming", "web-dev")

**Archive:**
```json
{
  "type": "archive",
  "year": 2026,
  "month": 1,
  "day": 15
}
```
- `year`: 연도 (4자리, 항상 존재)
- `month`: 월 (1-12, Yearly인 경우 없음, Optional)
- `day`: 일 (1-31, Monthly/Yearly인 경우 없음, Optional)

**템플릿 사용 예:**
```jinja
{% if list.list_kind.type == "taxonomy" %}
  <h2>{{ list.list_kind.name }}: {{ list.list_kind.slug }}</h2>
{% elif list.list_kind.type == "archive" %}
  {% if list.list_kind.day %}
    <h2>{{ list.list_kind.year }}년 {{ list.list_kind.month }}월 {{ list.list_kind.day }}일</h2>
  {% elif list.list_kind.month %}
    <h2>{{ list.list_kind.year }}년 {{ list.list_kind.month }}월</h2>
  {% else %}
    <h2>{{ list.list_kind.year }}년</h2>
  {% endif %}
{% endif %}
```

### 사용 예제

```jinja
<main>
  <h2>{{ list.title }}</h2>
  <p>{{ list.posts | length }}개의 포스트</p>
  
  {% for post in list.posts %}
    <article>
      <h3><a href="{{ post.url }}">{{ post.title }}</a></h3>
      <time>{{ post.date }}</time>
    </article>
  {% endfor %}
  
  <nav>
    {% if list.pagination.prev %}
      <a href="{{ list.pagination.prev.url }}">← 이전 페이지</a>
    {% endif %}
    <span>{{ list.pagination.page }} / {{ list.pagination.total_pages }}</span>
    {% if list.pagination.next %}
      <a href="{{ list.pagination.next.url }}">다음 페이지 →</a>
    {% endif %}
  </nav>
</main>
```

---

## 5. 포스트 페이지 변수 (post)

**사용 가능 템플릿**: `theme.yaml - template_default.post`, 기본값 : `post.html`

**의미**: 개별 블로그 포스트 렌더링 데이터

### 필드 목록

| 필드 | 타입 | 설명 |
|------|------|------|
| `kind` | object | 콘텐츠 종류 ({ "type": "post" } 또는 { "type": "page" }) |
| `title` | string | 포스트 제목 |
| `url` | string | 포스트 URL |
| `description` | string? | 포스트 설명 (메타 태그용) |
| `language` | string? | 포스트 언어 코드 |
| `date` | string | 발행 날짜 (YYYY-MM-DD 형식) |
| `updated` | string? | 수정 날짜 |
| `taxonomies` | BTreeMap<string, TaxonomyItem[]> | 포스트에 할당된 taxonomies (theme.yaml 설정 기반) |
| `summary` | string? | 포스트 요약 |
| `thumbnail` | string? | 썸네일 이미지 URL |
| `content_html` | string | HTML로 변환된 본문 |
| `prev` | NavLink? | 이전 포스트 링크 |
| `next` | NavLink? | 다음 포스트 링크 |
| `extra` | object | Front Matter의 사용자 정의 필드 |

### 사용 예제

```jinja
<article>
  <header>
    <h1>{{ post.title }}</h1>
    <time datetime="{{ post.date }}">{{ post.date }}</time>

    {# theme.yaml에 tags가 정의되고 포스트에 할당된 경우 #}
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

## 6. 일반 페이지 변수 (page)

**사용 가능 템플릿**: `theme.yaml - template_default.page`, 기본값 : `page.html`

**의미**: About, Contact 등 정적 페이지 렌더링 데이터

### 필드 목록

| 필드 | 타입 | 설명 |
|------|------|------|
| `title` | string | 페이지 제목 |
| `url` | string | 페이지 URL |
| `description` | string? | 페이지 설명 |
| `language` | string? | 페이지 언어 코드 |
| `date` | string | 생성 날짜 |
| `updated` | string? | 수정 날짜 |
| `content_html` | string | HTML로 변환된 본문 |
| `extra` | object | Front Matter의 사용자 정의 필드 |

### 사용 예제

```jinja
<article>
  <h1>{{ page.title }}</h1>
  
  <div class="content">
    {{ page.content_html | safe }}
  </div>
  
  {% if page.updated %}
    <footer>
      <p>마지막 수정: {{ page.updated }}</p>
    </footer>
  {% endif %}
</article>
```

---

## 7. 데이터 구조 명세

### TaxonomyItem

태그 또는 카테고리 항목을 나타냅니다.

| 필드 | 타입 | 설명 |
|------|------|------|
| `label` | string | 표시 이름 (예: "Rust", "개발") |
| `url` | string | 태그/카테고리 페이지 URL |
| `count` | number | 해당 태그/카테고리의 포스트 수 |

```jinja
{# theme.yaml에 tags가 정의된 경우 #}
{% if site.taxonomies.tags %}
  {% for tag in site.taxonomies.tags %}
    <a href="{{ tag.url }}">{{ tag.label }} ({{ tag.count }})</a>
  {% endfor %}
{% endif %}
```

### ArchiveItem

아카이브 항목을 나타냅니다 (연도별/월별/일별).

| 필드 | 타입 | 설명 |
|------|------|------|
| `label` | string | 표시 이름 (예: "2026", "2026-01", "2026-01-15") |
| `kind` | string | 아카이브 종류 ("yearly", "monthly", "daily") |
| `year` | number | 연도 (4자리) |
| `month` | number? | 월 (1-12, Monthly/Daily에만 존재) |
| `day` | number? | 일 (1-31, Daily에만 존재) |
| `url` | string | 아카이브 페이지 URL |
| `count` | number | 해당 기간의 포스트 수 |

```jinja
{% for archive in site.archives %}
  <a href="{{ archive.url }}">{{ archive.label }} ({{ archive.count }})</a>
{% endfor %}

{# kind에 따라 다른 형식으로 표시 #}
{% for archive in site.archives %}
  {% if archive.kind == "yearly" %}
    <li><a href="{{ archive.url }}">{{ archive.year }}년 ({{ archive.count }})</a></li>
  {% elif archive.kind == "monthly" %}
    <li><a href="{{ archive.url }}">{{ archive.year }}년 {{ archive.month }}월 ({{ archive.count }})</a></li>
  {% elif archive.kind == "daily" %}
    <li><a href="{{ archive.url }}">{{ archive.year }}-{{ archive.month }}-{{ archive.day }} ({{ archive.count }})</a></li>
  {% endif %}
{% endfor %}
```

### PostListItem

포스트 목록 항목을 나타냅니다 (홈/목록 페이지용).

| 필드 | 타입 | 설명 |
|------|------|------|
| `title` | string | 포스트 제목 |
| `url` | string | 포스트 URL |
| `date` | DateTime | 발행 날짜 (ISO 8601 형식) |
| `taxonomies` | BTreeMap<string, TaxonomyItem[]>? | 포스트에 할당된 taxonomies (theme.yaml 설정 기반) |
| `summary` | string? | 포스트 요약 |
| `thumbnail` | string? | 썸네일 이미지 URL |

```jinja
{% for post in home.posts %}
  <article>
    <h3><a href="{{ post.url }}">{{ post.title }}</a></h3>
    <time datetime="{{ post.date }}">{{ post.date[:10] }}</time>
    
    {# 포스트의 태그 표시 #}
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

이전/다음 포스트 또는 페이지네이션 링크를 나타냅니다.

| 필드 | 타입 | 설명 |
|------|------|------|
| `title` | string | 링크 텍스트 |
| `url` | string | 링크 URL |

**페이지네이션 NavLink의 title 값:**
- `prev`: "Page N" (N은 이전 페이지 번호)
- `next`: "Page N" (N은 다음 페이지 번호)
- `first`: "First"
- `last`: "Last"

**포스트 네비게이션 NavLink의 title 값:**
- `post.prev.title`: 이전 포스트의 실제 제목
- `post.next.title`: 다음 포스트의 실제 제목

```jinja
{% if post.prev %}
  <a href="{{ post.prev.url }}">← {{ post.prev.title }}</a>
{% endif %}
```

### Pagination

페이지네이션 정보를 나타냅니다.

| 필드 | 타입 | 설명 |
|------|------|------|
| `page` | number | 현재 페이지 번호 (1부터 시작) |
| `per_page` | number | 페이지당 항목 수 (0이면 전체 항목 수로 설정됨) |
| `total_items` | number | 전체 항목 수 |
| `total_pages` | number | 전체 페이지 수 |
| `has_prev` | boolean | 이전 페이지 존재 여부 |
| `has_next` | boolean | 다음 페이지 존재 여부 |
| `prev` | NavLink? | 이전 페이지 링크 |
| `next` | NavLink? | 다음 페이지 링크 |
| `first` | NavLink? | 첫 페이지 링크 (현재 페이지가 첫 페이지가 아닐 때만 존재) |
| `last` | NavLink? | 마지막 페이지 링크 (현재 페이지가 마지막이 아니고 전체 페이지가 1보다 클 때만 존재) |
| `pages` | PageLink[] | 페이지 번호 목록 (페이지 번호 네비게이션용) |

```jinja
<nav class="pagination">
  {# 첫 페이지 링크 #}
  {% if home.pagination.first %}
    <a href="{{ home.pagination.first.url }}">« 처음</a>
  {% endif %}

  {# 이전 페이지 #}
  {% if home.pagination.prev %}
    <a href="{{ home.pagination.prev.url }}">‹ 이전</a>
  {% endif %}

  {# 페이지 번호 목록 #}
  {% for page_link in home.pagination.pages %}
    {% if page_link.is_current %}
      <span class="current">{{ page_link.number }}</span>
    {% else %}
      <a href="{{ page_link.url }}">{{ page_link.number }}</a>
    {% endif %}
  {% endfor %}

  {# 다음 페이지 #}
  {% if home.pagination.next %}
    <a href="{{ home.pagination.next.url }}">다음 ›</a>
  {% endif %}

  {# 마지막 페이지 링크 #}
  {% if home.pagination.last %}
    <a href="{{ home.pagination.last.url }}">마지막 »</a>
  {% endif %}
</nav>
```

### PageLink

페이지 번호 링크를 나타냅니다 (Pagination의 pages 배열 항목).

| 필드 | 타입 | 설명 |
|------|------|------|
| `number` | number | 페이지 번호 (0은 생략 표시 "..."를 의미) |
| `url` | string | 페이지 URL (number가 0이면 빈 문자열) |
| `is_current` | boolean | 현재 페이지 여부 |

**참고**: `number`가 0인 항목은 페이지 번호 목록에서 "..."로 표시하기 위한 것입니다. 이 경우 `url`은 빈 문자열이므로 링크를 생성하지 않아야 합니다.

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

## 8. 템플릿별 사용 가능 변수

| 변수 | home.html | list.html | post.html | page.html | base.html |
|------|-----------|-----------|-----------|-----------|-----------|
| `site` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `home` | ✅ | ❌ | ❌ | ❌ | ❌ |
| `list` | ✅ | ✅ | ❌ | ❌ | ❌ |
| `post` | ❌ | ❌ | ✅ | ❌ | ❌ |
| `page` | ❌ | ❌ | ❌ | ✅ | ❌ |

**참고**: 
- **`home`은 `list`의 alias**: `home` 변수는 실제로 `ListContext` 타입이며, `list` 변수와 완전히 동일한 객체입니다. `home.html`에서는 같은 `ListContext` 객체가 `home`과 `list` 두 이름으로 모두 주입됩니다.
- **권장 사용법**: 가독성과 명확성을 위해 `home.html`에서는 `home` 변수를 사용하고, `list.html`에서는 `list` 변수를 사용하는 것을 권장합니다. 하지만 두 변수는 동일하므로 `home.html`에서도 `list`로 접근 가능합니다.
- `base.html`은 다른 템플릿에서 extend/include되는 레이아웃이므로, 상속받는 템플릿의 변수를 사용할 수 있습니다.

---

## 9. 실전 예제

### 예제 1: 사이드바 (base.html)

```jinja
<aside class="sidebar">
  {# theme.yaml에 tags가 정의된 경우만 표시 #}
  {% if site.taxonomies.tags %}
    <section>
      <h3>태그</h3>
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

  {# theme.yaml에 categories가 정의된 경우만 표시 #}
  {% if site.taxonomies.categories %}
    <section>
      <h3>카테고리</h3>
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

  {# 아카이브는 자동 생성 #}
  <section>
    <h3>아카이브</h3>
    <ul>
      {% for archive in site.archives %}
        <li>
          <a href="{{ archive.url }}">
            {{ archive.year }}년 {{ archive.month }}월 ({{ archive.count }})
          </a>
        </li>
      {% endfor %}
    </ul>
  </section>
</aside>
```

### 예제 2: 포스트 상세 (post.html)

```jinja
{% extends "base.html" %}

{% block content %}
<article class="post">
  <header>
    <h1>{{ post.title }}</h1>

    <div class="meta">
      <time datetime="{{ post.date }}">{{ post.date }}</time>

      {# theme.yaml에 categories가 정의되고 포스트에 할당된 경우 #}
      {% if post.taxonomies.categories %}
        <span class="categories">
          {% for category in post.taxonomies.categories %}
            <a href="{{ category.url }}">{{ category.label }}</a>
          {% endfor %}
        </span>
      {% endif %}
    </div>

    {# theme.yaml에 tags가 정의되고 포스트에 할당된 경우 #}
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
        <span class="label">이전 글</span>
        <a href="{{ post.prev.url }}">{{ post.prev.title }}</a>
      </div>
    {% endif %}

    {% if post.next %}
      <div class="next-post">
        <span class="label">다음 글</span>
        <a href="{{ post.next.url }}">{{ post.next.title }}</a>
      </div>
    {% endif %}
  </nav>
</article>
{% endblock %}
```

### 예제 3: 홈 페이지 (home.html)

```jinja
{% extends "base.html" %}

{% block content %}
<main class="home">
  <section class="recent-posts">
    <h2>최신 포스트</h2>
    
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
          
          <a href="{{ post.url }}" class="read-more">더 읽기 →</a>
        </article>
      {% endfor %}
    </div>
    
    {% if home.pagination.total_pages > 1 %}
      <nav class="pagination">
        {% if home.pagination.prev %}
          <a href="{{ home.pagination.prev.url }}" class="prev">
            ← 이전 페이지
          </a>
        {% endif %}
        
        <span class="page-info">
          {{ home.pagination.page }} / {{ home.pagination.total_pages }}
        </span>
        
        {% if home.pagination.next %}
          <a href="{{ home.pagination.next.url }}" class="next">
            다음 페이지 →
          </a>
        {% endif %}
      </nav>
    {% endif %}
  </section>
</main>
{% endblock %}
```

### 예제 4: 목록 페이지 (list.html)

```jinja
{% extends "base.html" %}

{% block content %}
<main class="list-page">
  <header>
    <h1>{{ list.title }}</h1>
    <p>{{ list.posts | length }}개의 포스트</p>
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
        <a href="{{ list.pagination.prev.url }}">← 이전</a>
      {% endif %}
      
      <span>{{ list.pagination.page }} / {{ list.pagination.total_pages }}</span>
      
      {% if list.pagination.next %}
        <a href="{{ list.pagination.next.url }}">다음 →</a>
      {% endif %}
    </nav>
  {% endif %}
</main>
{% endblock %}
```

---

## 10. 템플릿 필터

XNBlogGen은 minijinja 템플릿 엔진에 다음 커스텀 필터를 제공합니다.

---

### `date`

날짜/시간 문자열을 지정한 형식으로 변환합니다.

**입력 타입**
- RFC3339 문자열: `2024-01-15T09:00:00+09:00` (front matter의 `date` 필드 기본 형식)
- 날짜 문자열: `2024-01-15`

**키워드 인수**

| 인수 | 타입 | 기본값 | 설명 |
|------|------|--------|------|
| `fmt` | string | `%Y-%m-%d` | chrono 포맷 지정자 |

**주요 포맷 지정자**

| 지정자 | 출력 예시 | 설명 |
|--------|-----------|------|
| `%Y` | `2026` | 4자리 연도 |
| `%m` | `02` | 2자리 월 (숫자) |
| `%d` | `12` | 2자리 일 |
| `%B` | `February` | 월 전체 이름 (영어) |
| `%b` | `Feb` | 월 약어 (영어) |
| `%A` | `Thursday` | 요일 전체 이름 (영어) |
| `%a` | `Thu` | 요일 약어 (영어) |
| `%H` | `09` | 시 (24시간) |
| `%M` | `30` | 분 |
| `%S` | `00` | 초 |

**사용 예제**

```jinja
{# 기본값: 2026-02-12 #}
{{ post.date | date }}

{# 한국식: 2026년 02월 12일 #}
{{ post.date | date(fmt="%Y년 %m월 %d일") }}

{# 영문식: February 12, 2026 #}
{{ post.date | date(fmt="%B %d, %Y") }}

{# 짧은 형식: Feb 12 #}
{{ post.date | date(fmt="%b %d") }}
```

---

### `slugify`

문자열을 URL 슬러그로 변환합니다.

**변환 규칙**
- 소문자로 변환
- 특수 기호 치환: `c++` → `cpp`, `.net` → `dotnet`, `+` → `plus`, `#` → `sharp`, `@` → `at`, `&` → `and`
- 공백과 하이픈은 `-` 하나로 치환
- 한글 등 비ASCII 문자는 그대로 보존
- 앞뒤 `-` 제거

**사용 예제**

```jinja
{# "Hello World" → "hello-world" #}
{{ post.title | slugify }}

{# "C++ 입문" → "cpp-입문" #}
{{ post.title | slugify }}

{# "C# & .NET" → "csharp-and-dotnet" #}
{{ post.title | slugify }}

{# 태그 링크 생성 #}
<a href="/tags/{{ tag.label | slugify }}/">{{ tag.label }}</a>
```

---

## 호환성 및 버전 정책

- **필드 추가**: 새로운 필드는 하위 호환성을 유지하며 추가될 수 있습니다
- **필드 제거**: 기존 필드는 최소 1개 메이저 버전 동안 deprecated 표시 후 제거됩니다
- **타입 변경**: 필드 타입 변경은 메이저 버전 업데이트에서만 발생합니다
- **명명 규칙**: 모든 필드는 snake_case를 사용합니다

---

**문서 버전**: 1.0
**최종 수정일**: 2026-01-26
