//! Desktop shell. The UI lives in `../src` (React); this crate hosts it in a
//! native window and exposes a few commands the UI can call with `invoke`.

const DEFAULT_API_URL: &str = "http://localhost:8080";

/// Base URL of the SIS API. Set `SIS_API_URL` to point at another server.
fn resolve_api_url(env_value: Option<String>) -> String {
    env_value
        .map(|v| v.trim().trim_end_matches('/').to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| DEFAULT_API_URL.to_string())
}

#[tauri::command]
fn api_base_url() -> String {
    resolve_api_url(std::env::var("SIS_API_URL").ok())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api_base_url])
        .run(tauri::generate_context!())
        .expect("error while running the desktop application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_localhost() {
        assert_eq!(resolve_api_url(None), DEFAULT_API_URL);
        assert_eq!(resolve_api_url(Some("  ".into())), DEFAULT_API_URL);
    }

    #[test]
    fn uses_env_value_without_trailing_slash() {
        assert_eq!(
            resolve_api_url(Some("https://sis.example.edu/".into())),
            "https://sis.example.edu"
        );
    }
}
