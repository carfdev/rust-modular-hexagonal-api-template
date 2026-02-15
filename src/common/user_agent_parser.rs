/// Parses a raw User-Agent string into a human-readable device name.
///
/// Examples:
/// - `"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 ... Chrome/120.0"` → `"Chrome on Windows"`
/// - `"PostmanRuntime/7.36.0"` → `"PostmanRuntime"`
/// - `"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) ... Firefox/121.0"` → `"Firefox on macOS"`
pub fn parse_device_name(user_agent: &str) -> String {
    let browser = detect_browser(user_agent);
    let os = detect_os(user_agent);

    match (browser, os) {
        (Some(b), Some(o)) => format!("{} on {}", b, o),
        (Some(b), None) => b.to_string(),
        (None, Some(o)) => format!("Unknown browser on {}", o),
        (None, None) => "Unknown device".to_string(),
    }
}

fn detect_browser(ua: &str) -> Option<&'static str> {
    // Order matters: check more specific strings first
    if ua.contains("PostmanRuntime") {
        return Some("Postman");
    }
    if ua.contains("Insomnia") {
        return Some("Insomnia");
    }
    if ua.contains("curl") {
        return Some("curl");
    }
    if ua.contains("Thunder Client") {
        return Some("Thunder Client");
    }
    // Edge must come before Chrome (Edge UA contains "Chrome")
    if ua.contains("Edg/") || ua.contains("Edge/") {
        return Some("Edge");
    }
    // Opera/OPR must come before Chrome
    if ua.contains("OPR/") || ua.contains("Opera") {
        return Some("Opera");
    }
    // Brave embeds "Chrome" too, check for Brave first
    if ua.contains("Brave") {
        return Some("Brave");
    }
    // Vivaldi
    if ua.contains("Vivaldi") {
        return Some("Vivaldi");
    }
    // Samsung Internet
    if ua.contains("SamsungBrowser") {
        return Some("Samsung Internet");
    }
    // Chrome must come before Safari (Safari UA doesn't contain "Chrome")
    if ua.contains("Chrome/") || ua.contains("CriOS/") {
        return Some("Chrome");
    }
    // Safari (must be after Chrome/Edge/Opera checks)
    if ua.contains("Safari/") {
        return Some("Safari");
    }
    if ua.contains("Firefox/") || ua.contains("FxiOS/") {
        return Some("Firefox");
    }

    None
}

fn detect_os(ua: &str) -> Option<&'static str> {
    if ua.contains("Windows") {
        return Some("Windows");
    }
    // iOS must come before macOS (iOS UA contains "Mac OS X")
    if ua.contains("iPhone") || ua.contains("iPad") || ua.contains("iPod") {
        return Some("iOS");
    }
    if ua.contains("Macintosh") || ua.contains("Mac OS X") {
        return Some("macOS");
    }
    // Android must come before Linux (Android UA contains "Linux")
    if ua.contains("Android") {
        return Some("Android");
    }
    if ua.contains("CrOS") {
        return Some("ChromeOS");
    }
    if ua.contains("Linux") {
        return Some("Linux");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chrome_windows() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
        assert_eq!(parse_device_name(ua), "Chrome on Windows");
    }

    #[test]
    fn test_firefox_macos() {
        let ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0";
        assert_eq!(parse_device_name(ua), "Firefox on macOS");
    }

    #[test]
    fn test_safari_macos() {
        let ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15";
        assert_eq!(parse_device_name(ua), "Safari on macOS");
    }

    #[test]
    fn test_edge_windows() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0";
        assert_eq!(parse_device_name(ua), "Edge on Windows");
    }

    #[test]
    fn test_postman() {
        let ua = "PostmanRuntime/7.36.0";
        assert_eq!(parse_device_name(ua), "Postman");
    }

    #[test]
    fn test_chrome_android() {
        let ua = "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36";
        assert_eq!(parse_device_name(ua), "Chrome on Android");
    }

    #[test]
    fn test_safari_ios() {
        let ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1";
        assert_eq!(parse_device_name(ua), "Safari on iOS");
    }

    #[test]
    fn test_unknown() {
        let ua = "some-random-client/1.0";
        assert_eq!(parse_device_name(ua), "Unknown device");
    }
}
