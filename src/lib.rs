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
    fn test_document_valid_simple() {
        let rule = Rule::document;
        let content = r#"<?xml version="1.0" encoding="utf-8"?>
<root>
</root>"#;
        let _parsed = match XmlParser1_0::parse(rule, content) {
            Err(e) => panic!("Document unexpectedly failed to parse: {}", e),
            Ok(p) => p,
        };
    }

    #[test]
    fn test_document_valid_planes_xsd() {
        let rule = Rule::document;
        let content = include_str!("../tests/valid/planes.xsd");
        let _parsed = match XmlParser1_0::parse(rule, content) {
            Err(e) => panic!("Document unexpectedly failed to parse: {}", e),
            Ok(p) => p,
        };
    }

    #[test]
    fn test_document_valid_planes_xml() {
        let rule = Rule::document;
        let content = include_str!("../tests/valid/planes.xml");
        let _parsed = match XmlParser1_0::parse(rule, content) {
            Err(e) => panic!("Document unexpectedly failed to parse: {}", e),
            Ok(p) => p,
        };
    }

    #[test]
    fn test_document_valid_planes_xsd_xml() {
        let rule = Rule::document;
        let content = include_str!("../tests/valid/planes.xsd.xml");
        let _parsed = match XmlParser1_0::parse(rule, content) {
            Err(e) => panic!("Document unexpectedly failed to parse: {}", e),
            Ok(p) => p,
        };
    }

    #[test]
    fn test_document_valid_note_in_dtd_xml() {
        let rule = Rule::document;
        let content = include_str!("../tests/valid/note_in_dtd.xml");
        let _parsed = match XmlParser1_0::parse(rule, content) {
            Err(e) => panic!("Document unexpectedly failed to parse: {}", e),
            Ok(p) => p,
        };
    }

    #[test]
    fn test_document_valid_note_ex_dtd_xml() {
        let rule = Rule::document;
        let content = include_str!("../tests/valid/note_ex_dtd.xml");
        let _parsed = match XmlParser1_0::parse(rule, content) {
            Err(e) => panic!("Document unexpectedly failed to parse: {}", e),
            Ok(p) => p,
        };
    }

    #[test]
    #[should_panic]
    fn test_document_invalid_empty() {
        let rule = Rule::document;
        let content = r#""#;
        let _parsed = XmlParser1_0::parse(rule, content).expect("Empty document is not valid.");
    }

    #[test]
    #[should_panic]
    fn test_document_invalid_missing_root() {
        let rule = Rule::document;
        let content = r#"<?xml version="1.0" encoding="utf-8"?>"#;
        let _parsed = XmlParser1_0::parse(rule, content).expect("Document missing a root element is not valid.");
    }

}
