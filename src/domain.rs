/// Normalize a domain to Pi-hole-compatible form
pub fn normalize_domain(domain: &str) -> Option<Box<str>> {
    let d = domain
        .trim()
        .trim_end_matches('.')
        .to_ascii_lowercase();

    if d.is_empty() || !d.contains('.') {
        return None;
    }

    Some(d.into_boxed_str())
}

/// Parse a single line using Pi-hole gravity rules
pub fn parse_gravity_line(line: &str) -> Option<Box<str>> {
    let mut s = line.trim();

    // Skip empty lines and full-line comments
    if s.is_empty() || s.starts_with('#') {
        return None;
    }

    // Strip inline comments
    if let Some(idx) = s.find('#') {
        s = &s[..idx];
    }

    let parts: Vec<&str> = s.split_whitespace().collect();

    let domain = match parts.as_slice() {
        // example.com
        [d] => *d,

        // 0.0.0.0 example.com
        [_ip, d] => *d,

        _ => return None,
    };

    // Reject unsupported syntax
    if domain.contains('*')
        || domain.contains('/')
        || domain.contains('|')
    {
        return None;
    }

    normalize_domain(domain)
}

/// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    // normalize_domain
    #[test]
    fn normalize_basic_domain() {
        assert_eq!(
            normalize_domain("Example.COM"),
            Some("example.com".into())
        );
    }

    #[test]
    fn normalize_trailing_dot() {
        assert_eq!(
            normalize_domain("example.com."),
            Some("example.com".into())
        );
    }

    #[test]
    fn normalize_whitespace() {
        assert_eq!(
            normalize_domain("  example.com  "),
            Some("example.com".into())
        );
    }

    #[test]
    fn normalize_reject_empty() {
        assert_eq!(normalize_domain(""), None);
        assert_eq!(normalize_domain("   "), None);
    }

    #[test]
    fn normalize_reject_no_dot() {
        assert_eq!(normalize_domain("localhost"), None);
        assert_eq!(normalize_domain("printer"), None);
    }

    /// parse_gravity_line: valid cases
    #[test]
    fn parse_plain_domain() {
        assert_eq!(
            parse_gravity_line("example.com"),
            Some("example.com".into())
        );
    }

    #[test]
    fn parse_hosts_ipv4_entry() {
        assert_eq!(
            parse_gravity_line("0.0.0.0 example.com"),
            Some("example.com".into())
        );
    }

    #[test]
    fn parse_hosts_localhost_entry() {
        assert_eq!(
            parse_gravity_line("127.0.0.1 example.com"),
            Some("example.com".into())
        );
    }

    #[test]
    fn parse_inline_comment() {
        assert_eq!(
            parse_gravity_line("example.com  # tracker"),
            Some("example.com".into())
        );
    }

    #[test]
    fn parse_uppercase_domain() {
        assert_eq!(
            parse_gravity_line("EXAMPLE.COM"),
            Some("example.com".into())
        );
    }

    /// parse_gravity_line: rejected cases
    #[test]
    fn reject_full_line_comment() {
        assert_eq!(parse_gravity_line("# example.com"), None);
    }

    #[test]
    fn reject_empty_line() {
        assert_eq!(parse_gravity_line(""), None);
        assert_eq!(parse_gravity_line("   "), None);
    }

    #[test]
    fn reject_wildcards() {
        assert_eq!(parse_gravity_line("*.example.com"), None);
    }

    #[test]
    fn reject_adblock_syntax() {
        assert_eq!(parse_gravity_line("||example.com^"), None);
    }

    #[test]
    fn reject_regex_like_entries() {
        assert_eq!(parse_gravity_line("/example\\.com/"), None);
    }

    #[test]
    fn reject_multiple_domains_on_line() {
        assert_eq!(
            parse_gravity_line("0.0.0.0 example.com another.com"),
            None
        );
    }

    #[test]
    fn reject_ipv6_hosts_entries() {
        assert_eq!(
            parse_gravity_line("::1 example.com"),
            None
        );
    }

    #[test]
    fn reject_domain_without_dot() {
        assert_eq!(
            parse_gravity_line("0.0.0.0 localhost"),
            None
        );
    }
}
