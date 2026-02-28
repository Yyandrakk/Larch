#[cfg(test)]
mod tests {
    use crate::TaigaClient;
    use url::Url;

    #[test]
    fn test_is_managed_url() {
        let base_url = Url::parse("https://api.taiga.io/").unwrap();
        let client = TaigaClient::new(base_url);

        // Exact match
        assert!(client.is_managed_url("https://api.taiga.io/some/resource"));

        // Subdomain match
        assert!(client.is_managed_url("https://media.api.taiga.io/attachment.png"));

        // Different domain
        assert!(!client.is_managed_url("https://evil.com/exploit"));

        // Similar domain but not subdomain (suffix match attempt)
        assert!(!client.is_managed_url("https://fakeapi.taiga.io.evil.com/resource"));

        // Invalid URL
        assert!(!client.is_managed_url("not-a-url"));
    }
}
