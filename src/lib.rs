extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "xml1_0.pest"]
pub struct XmlParser1_0;

#[cfg(test)]
mod tests {
    #[test]
    fn simple_docs() {
        assert!(XmlParser1_0::parse(Rule::doc, r#""#).is_err());
        assert!(
            XmlParser1_0::parse(Rule::doc, r#"<?xml version="1.0" encoding="utf-8"?>"#).is_err()
        );
        assert!(
            XmlParser1_0::parse(
                Rule::doc,
                r#"<?xml version="1.0" encoding="utf-8"?>\n<root/>"#
            ).is_ok()
        );
    }
}
