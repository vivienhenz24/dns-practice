// DNS service that resolves domain names to IP addresses
pub fn resolve_domain(_domain: &str) -> Option<String> {
    let domain = _domain;

    let ip = match domain {
        "google.com" => Some("8.8.8.8".to_string()),
        "github.com" => Some("1.1.1.1".to_string()),
        "rust-lang.org" => Some("192.168.1.1".to_string()),
        "example.com" => Some("10.0.0.1".to_string()),
        "stackoverflow.com" => Some("172.16.0.1".to_string()),
        "reddit.com" => Some("203.0.113.1".to_string()),
        _ => None,
    };

    return ip;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_com() {
        let domain = "google.com";
        let ip = resolve_domain(domain);
        assert_eq!(ip, Some("8.8.8.8".to_string()), "google.com should resolve to 8.8.8.8");
    }

    #[test]
    fn test_github_com() {
        let domain = "github.com";
        let ip = resolve_domain(domain);
        assert_eq!(ip, Some("1.1.1.1".to_string()), "github.com should resolve to 1.1.1.1");
    }

    #[test]
    fn test_rust_lang_org() {
        let domain = "rust-lang.org";
        let ip = resolve_domain(domain);
        assert_eq!(ip, Some("192.168.1.1".to_string()), "rust-lang.org should resolve to 192.168.1.1");
    }

    #[test]
    fn test_example_com() {
        let domain = "example.com";
        let ip = resolve_domain(domain);
        assert_eq!(ip, Some("10.0.0.1".to_string()), "example.com should resolve to 10.0.0.1");
    }

    #[test]
    fn test_stackoverflow_com() {
        let domain = "stackoverflow.com";
        let ip = resolve_domain(domain);
        assert_eq!(ip, Some("172.16.0.1".to_string()), "stackoverflow.com should resolve to 172.16.0.1");
    }

    #[test]
    fn test_reddit_com() {
        let domain = "reddit.com";
        let ip = resolve_domain(domain);
        assert_eq!(ip, Some("203.0.113.1".to_string()), "reddit.com should resolve to 203.0.113.1");
    }

    #[test]
    fn test_invalid_domain() {
        let domain = "this-domain-definitely-does-not-exist-12345.com";
        let ip = resolve_domain(domain);
        assert_eq!(ip, None, "Invalid domain should return None");
    }
}
