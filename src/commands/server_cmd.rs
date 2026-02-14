// src/commands/server.rs

use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Instant;

use tiny_http::{Server, Response, StatusCode};
use thiserror::Error;

use crate::utils::output;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO error\n  Path: {path}\n  Reason: {source}")]
    Io { path: PathBuf, source: std::io::Error },

    #[error("Blog project not found\n  Path: {path}\n  Expected: blogconfig.yaml")]
    ProjectNotFound{ path: PathBuf},

    #[error("Publishing directory not found\n  Path: {path}\n  Expected: public/")]
    PublishingDirNotFound{ path: PathBuf },

    #[error("Failed to start server\n  Reason: {0}")]
    ServerStartFailed(String),

    #[error("Invalid request path\n  Path: {0}")]
    InvalidPath(String),

    #[error("File not found\n  Path: {path}")]
    FileNotFound{ path: PathBuf },

    #[error("Security violation: path traversal attempt detected")]
    PathTraversal,
}

pub fn run(port: u16, root: &str) -> Result<(), ServerError> {
    let project_path = PathBuf::from(root);
    let config_path = project_path.join("blogconfig.yaml");

    if !config_path.is_file() {
        return Err(ServerError::ProjectNotFound{ path: project_path.clone() });
    }

    let publishing_dir = project_path.join("public");
    if !publishing_dir.is_dir() {
        return Err(ServerError::PublishingDirNotFound{ path: publishing_dir.clone() });
    }

    if port == 0 || port > u16::MAX {
        return Err(ServerError::ServerStartFailed("Invalid port number".to_string()));
    }

    let addr = format!("127.0.0.1:{}", port);
    let server = Server::http(&addr).map_err(|e| {
        ServerError::ServerStartFailed(format!("Could not bind to {}: {}", addr, e))
    })?;

    output::info(&format!("Starting server at http://{}/", addr));
    output::print_path(&publishing_dir.display().to_string());
    output::info("Press Ctrl+C to stop the server");
    eprintln!();

    for request in server.incoming_requests() {
        let publishing_dir = publishing_dir.clone();

        thread::spawn(move || {
            if let Err(e) = handle_request(request, &publishing_dir) {
                output::error(&format!("Request error: {}", e));
            }
        });
    }

    Ok(())
}

fn handle_request(
    request: tiny_http::Request,
    publishing_dir: &std::path::Path,
) -> Result<(), ServerError> {
    let url_path = request.url();
    let method = request.method();
    let start_time = Instant::now();

    // Process the request
    let result = (|| {
        // 1. Validate and resolve path
        let safe_path = validate_and_resolve_path(url_path, publishing_dir)?;

        // 2. Resolve to actual file
        let file_path = resolve_file_path(&safe_path)?;

        // 3. Read file contents
        let contents = fs::read(&file_path).map_err(|e| ServerError::Io {
            path: file_path.clone(),
            source: e,
        })?;

        // 4. Determine Content-Type
        let content_type = get_content_type(&file_path);

        Ok((contents, content_type, file_path))
    })();

    // Handle result and send appropriate response
    match result {
        Ok((contents, content_type, file_path)) => {
            let elapsed = start_time.elapsed();
            output::info(&format!(
                "{} {} - 200 OK ({} bytes, {:.2}ms)",
                method,
                url_path,
                contents.len(),
                elapsed.as_secs_f64() * 1000.0
            ));
            output::print_path(&file_path.display().to_string());

            let response = Response::from_data(contents).with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes())
                    .unwrap(),
            );

            request.respond(response).map_err(|e| {
                ServerError::ServerStartFailed(format!("Failed to send response: {}", e))
            })?;
        }
        Err(ServerError::FileNotFound { .. }) | Err(ServerError::InvalidPath(_)) => {
            output::warning(&format!("{} {} - 404 Not Found", method, url_path));

            let body = b"404 Not Found";
            let response = Response::from_data(&body[..])
                .with_status_code(StatusCode(404))
                .with_header(
                    tiny_http::Header::from_bytes(
                        &b"Content-Type"[..],
                        b"text/plain; charset=utf-8",
                    )
                    .unwrap(),
                );

            request.respond(response).ok();
        }
        Err(ServerError::PathTraversal) => {
            output::error(&format!("{} {} - 403 Forbidden (path traversal)", method, url_path));

            let body = b"403 Forbidden: Path traversal detected";
            let response = Response::from_data(&body[..])
                .with_status_code(StatusCode(403))
                .with_header(
                    tiny_http::Header::from_bytes(
                        &b"Content-Type"[..],
                        b"text/plain; charset=utf-8",
                    )
                    .unwrap(),
                );

            request.respond(response).ok();
        }
        Err(e) => {
            output::error(&format!("Internal server error: {}", e));
            output::error(&format!("{} {} - 500 Internal Server Error", method, url_path));

            let body = b"500 Internal Server Error";
            let response = Response::from_data(&body[..])
                .with_status_code(StatusCode(500))
                .with_header(
                    tiny_http::Header::from_bytes(
                        &b"Content-Type"[..],
                        b"text/plain; charset=utf-8",
                    )
                    .unwrap(),
                );

            request.respond(response).ok();
        }
    }

    Ok(())
}

/// Validates and resolves a request path to a safe file system path
///
/// Security checks:
/// - Path traversal defense (../, ../../, etc.)
/// - Absolute path rejection
/// - Symbolic link validation
/// - Access outside publishing_dir blocked
fn validate_and_resolve_path(
    request_path: &str,
    publishing_dir: &std::path::Path,
) -> Result<PathBuf, ServerError> {
    // 1. URL decode
    let decoded_path = urlencoding::decode(request_path)
        .map_err(|_| ServerError::InvalidPath(request_path.to_string()))?;

    // 2. Remove leading slash
    let decoded_path = decoded_path.trim_start_matches('/');

    // 3. Build candidate path
    let candidate_path = publishing_dir.join(decoded_path);

    // 4. Canonicalize publishing directory
    let canonical_publishing = publishing_dir
        .canonicalize()
        .map_err(|e| ServerError::Io {
            path: publishing_dir.to_path_buf(),
            source: e,
        })?;

    // 5. Try to canonicalize candidate path
    if let Ok(canonical_candidate) = candidate_path.canonicalize() {
        // Path exists - check if it's within publishing_dir
        if !canonical_candidate.starts_with(&canonical_publishing) {
            return Err(ServerError::PathTraversal);
        }
        return Ok(canonical_candidate);
    }

    // 6. Path doesn't exist yet - validate relative path components
    let relative = candidate_path
        .strip_prefix(&canonical_publishing)
        .map_err(|_| ServerError::PathTraversal)?;

    // Check for ".." components
    for component in relative.components() {
        if matches!(component, std::path::Component::ParentDir) {
            return Err(ServerError::PathTraversal);
        }
    }

    Ok(candidate_path)
}

/// Resolves a path to an actual file to serve
///
/// Priority:
/// 1. If path is a file -> return it directly
/// 2. If path is a directory -> look for index.html
/// 3. If no index.html -> return 404
fn resolve_file_path(path: &std::path::Path) -> Result<PathBuf, ServerError> {
    if path.is_file() {
        return Ok(path.to_path_buf());
    }

    if path.is_dir() {
        let index_path = path.join("index.html");
        if index_path.is_file() {
            return Ok(index_path);
        }
    }

    Err(ServerError::FileNotFound {
        path: path.to_path_buf(),
    })
}

/// Determines Content-Type header based on file extension
fn get_content_type(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("xml") => "application/xml; charset=utf-8",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("eot") => "application/vnd.ms-fontobject",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}