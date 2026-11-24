use std::collections::HashSet;

pub fn normalize_tags(tags: Vec<String>) -> String {
    let mut seen = HashSet::new();

    let mut result = Vec::new();

    for tag in tags {
        let clean = tag.trim().to_lowercase();

        if clean.is_empty() {
            continue;
        }

        if seen.insert(clean.clone()) {
            result.push(clean);
        }
    }

    result.sort();

    result.join(",")
}

pub fn parse_tags(tags: &str) -> Vec<String> {
    tags.split(',')
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .map(String::from)
        .collect()
}

pub fn count_code_lines(code: &str) -> usize {
    if code.trim().is_empty() {
        return 0;
    }

    code.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_tags() {
        let tags = vec![
            " Rust ".to_string(),
            "API".to_string(),
            "rust".to_string(),
            "".to_string(),
        ];

        let result = normalize_tags(tags);

        assert_eq!(result, "api,rust");
    }

    #[test]
    fn parses_tags() {
        let result = parse_tags("api,rust,sqlite");

        assert_eq!(
            result,
            vec![
                "api".to_string(),
                "rust".to_string(),
                "sqlite".to_string()
            ]
        );
    }

    #[test]
    fn counts_lines() {
        let code = "fn main() {\n    println!(\"Hello\");\n}";

        assert_eq!(count_code_lines(code), 3);
    }
}
