# Changelog

> **Language**: [🇰🇷 한국어](CHANGELOG.md) | [🇺🇸 English](CHANGELOG.en.md)

## [0.1.1] - 2026-02-22

### 추가

- 템플릿 커스텀 필터 (`src/utils/filters.rs`)
  - `date` 필터: 날짜를 지정한 포맷으로 출력 (`fmt` 키워드 인수 지원)
    - 입력: RFC3339 문자열 (`DateTime<FixedOffset>`) 또는 `YYYY-MM-DD` 문자열
    - 기본 포맷: `%Y-%m-%d`
    - 사용 예: `{{ post.date | date(fmt="%Y년 %m월 %d일") }}`
  - `slugify` 필터: 문자열을 URL 슬러그로 변환
    - 사용 예: `{{ post.title | slugify }}`

### 변경

- `slugify` 함수를 `context/context_builder.rs`에서 `utils/filters.rs`로 이동
  - 기존 내부 동작은 동일하게 유지

---

## [0.1.0] - 2026-02-04

- 마크다운 기반 정적 블로그 생성기
- `create` 명령어: 새 블로그 프로젝트 초기화
- `new` 명령어: 포스트/페이지 생성
- `build` 명령어: 정적 사이트 빌드
- `server` 명령어: 로컬 개발 서버
- 포스트와 페이지 구분
- 초안(draft) 기능
- Tags, Categories 등 사용자 정의 Taxonomy
- 아카이브 시스템 (연/월/일)
- 페이지네이션
- 커스텀 템플릿
- 코드 하이라이팅
- 포스트 네비게이션 (이전/다음)
- RSS 피드 자동 생성
- Sitemap 자동 생성
