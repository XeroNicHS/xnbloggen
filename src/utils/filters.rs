// src/utils/filters.rs

use chrono::{DateTime, FixedOffset, NaiveDate};
use minijinja::{Environment, Value, Error, ErrorKind};
use minijinja::value::Kwargs;

//------------------------------------------------------------------------------
// slugify
//------------------------------------------------------------------------------

/// Convert a string into a URL-safe slug.
/// - Replaces common tech symbols: c++ → cpp, .net → dotnet, # → sharp, etc.
/// - Keeps alphanumeric characters (including Korean/Unicode)
/// - Replaces whitespace and hyphens with a single '-'
/// - Trims leading/trailing hyphens
pub fn slugify(input: &str) -> String {
    let lowercased = input.to_lowercase();
    let normalized = lowercased
        .replace("c++", "cpp")
        .replace(".net", "dotnet")
        .replace("+", "plus")
        .replace("#", "sharp")
        .replace("@", "at")
        .replace("&", "and");

    let mut output = String::new();
    let mut prev_dash = false;
    for ch in normalized.chars() {
        if ch.is_alphanumeric() {
            output.push(ch);
            prev_dash = false;
        } else if ch == '-' || ch.is_whitespace() {
            if !prev_dash {
                output.push('-');
                prev_dash = true;
            }
        }
    }
    output.trim_matches('-').to_string()
}

/// minijinja filter: `{{ value | slugify }}`
pub fn filter_slugify(value: Value) -> Result<Value, Error> {
    let s = value.as_str().ok_or_else(|| {
        Error::new(ErrorKind::InvalidOperation, "slugify: expected a string value")
    })?;
    Ok(Value::from(slugify(s)))
}

//------------------------------------------------------------------------------
// date
//------------------------------------------------------------------------------

/// minijinja filter: `{{ value | date }}` or `{{ value | date(fmt="%Y년 %m월 %d일") }}`
///
/// Input: RFC3339 string (e.g., "2024-01-15T09:00:00+09:00") as serialized by
/// `chrono::DateTime<FixedOffset>`, or plain date string "2024-01-15".
///
/// `fmt` defaults to `"%Y-%m-%d"` if not provided.
pub fn filter_date(value: Value, kwargs: Kwargs) -> Result<Value, Error> {
    let s = value.as_str().ok_or_else(|| {
        Error::new(ErrorKind::InvalidOperation, "date: expected a string value")
    })?;

    let fmt = kwargs.get::<Option<String>>("fmt")?.unwrap_or_else(|| "%Y-%m-%d".to_string());
    kwargs.assert_all_used()?;

    // 1) Try RFC3339 (DateTime<FixedOffset>) — primary path
    if let Ok(dt) = DateTime::<FixedOffset>::parse_from_rfc3339(s) {
        return Ok(Value::from(dt.format(&fmt).to_string()));
    }

    // 2) Fallback: plain date "YYYY-MM-DD"
    if let Ok(nd) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Ok(Value::from(nd.format(&fmt).to_string()));
    }

    Err(Error::new(
        ErrorKind::InvalidOperation,
        format!("date: cannot parse '{}' as a date/datetime", s),
    ))
}

//------------------------------------------------------------------------------
// Register all filters
//------------------------------------------------------------------------------

/// Register all custom filters onto the given minijinja `Environment`.
///
/// Call this once after `Environment::new()` / `set_loader()`.
pub fn register_all(env: &mut Environment) {
    env.add_filter("slugify", filter_slugify);
    env.add_filter("date", filter_date);
}
