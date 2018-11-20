extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "xml1_0.pest"]
pub struct XmlParser1_0;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_document() {
        let rule = Rule::document;

        let mut content = r#""#;
        let mut parsed = XmlParser1_0::parse(rule, content);
        assert_eq!(parsed.is_ok(), false);

        content = r#"<?xml version="1.0" encoding="utf-8"?>"#;
        parsed = XmlParser1_0::parse(rule, content);
        assert_eq!(parsed.is_ok(), false);

        content = r#"<?xml version="1.0" encoding="utf-8"?>
<root>
</root>"#;
        parsed = XmlParser1_0::parse(rule, content);
        assert_eq!(parsed.is_ok(), true);
    }
}
