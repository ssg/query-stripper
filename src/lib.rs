pub fn remove_query_string(query_str: &str, key: &str) -> String {
    const SEPARATOR: &str = "&";

    query_str
        .split(SEPARATOR)
        .filter(|p| !p.starts_with(&format!("{}=", key)))
        .collect::<Vec<&str>>()
        .join(SEPARATOR)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("", "")]
    #[test_case("instanceId=1234", "")]
    #[test_case("instanceId=1234&x=5&y=6", "x=5&y=6")]
    #[test_case("a=5&instanceId=1234&x=5&y=6", "a=5&x=5&y=6")]
    #[test_case("a=5&x=5&y=6&instanceId=1234", "a=5&x=5&y=6")]
    #[test_case("a=5&x=5&y=6&binstanceId=1234", "a=5&x=5&y=6&binstanceId=1234")]
    #[test_case("a=5&x=5&y=6&instanceId=1234&instanceId=5678", "a=5&x=5&y=6")]
    #[test_case(
        "instanceId=1234&a=5&instanceId=5678&x=5&instanceId=5678&y=6&instanceId=9abc",
        "a=5&x=5&y=6"
    )]
    fn test_add(input: &str, output: &str) {
        assert_eq!(output, remove_query_string(input, "instanceId"));
    }
}
