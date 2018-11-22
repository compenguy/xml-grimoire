/* Conformance tests of well-formed, valid xml documents.
 * See /tests/xmltest/xmltest.xml for short descriptions
 * of each test document.
 */

#[cfg(test)]
mod tests {
    use lexer::Rule;
    use lexer::XmlLexer1_0;
    use pest::Parser;

    fn conformance_valid(id: &str, section: &str, desc: &str, content: &str) {
        println!("[{}] {} (Section {})", id, desc, section);
        let rule = Rule::document;
        let _parsed = match XmlLexer1_0::parse(rule, content) {
            Err(e) => panic!("Lexing the XML content failed: {}", e),
            Ok(p) => p,
        };
    }

    // <!-- Start:  valid/sa -->
    #[test]
    fn conformance_valid_sa_001() {
        let id = "valid-sa-001";
        let content = include_str!("../tests/xmltest/valid/sa/001.xml");
        let section = "3.2.2 [51]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/001.xml");
        let desc = "Test demonstrates an Element Type Declaration with Mixed Content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_002() {
        let id = "valid-sa-002";
        let content = include_str!("../tests/xmltest/valid/sa/002.xml");
        let section = "3.1 [40]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/002.xml");
        let desc =
            "Test demonstrates that whitespace is permitted after the tag name in a Start-tag.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_003() {
        let id = "valid-sa-003";
        let content = include_str!("../tests/xmltest/valid/sa/003.xml");
        let section = "3.1 [42]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/003.xml");
        let desc =
            "Test demonstrates that whitespace is permitted after the tag name in an End-tag.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_004() {
        let id = "valid-sa-004";
        let content = include_str!("../tests/xmltest/valid/sa/004.xml");
        let section = "3.1 [41]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/004.xml");
        let desc = "Test demonstrates a valid attribute specification within a Start-tag.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_005() {
        let id = "valid-sa-005";
        let content = include_str!("../tests/xmltest/valid/sa/005.xml");
        let section = "3.1 [40]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/005.xml");
        let desc = "Test demonstrates a valid attribute specification within a Start-tag that contains whitespace on both sides of the equal sign.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_006() {
        let id = "valid-sa-006";
        let content = include_str!("../tests/xmltest/valid/sa/006.xml");
        let section = "3.1 [41]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/006.xml");
        let desc = "Test demonstrates that the AttValue within a Start-tag can use a single quote as a delimter.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_007() {
        let id = "valid-sa-007";
        let content = include_str!("../tests/xmltest/valid/sa/007.xml");
        let section = "3.1 4.6 [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/007.xml");
        let desc =
            "Test demonstrates numeric character references can be used for element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_008() {
        let id = "valid-sa-008";
        let content = include_str!("../tests/xmltest/valid/sa/008.xml");
        let section = "2.4 3.1 [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/008.xml");
        let desc = "Test demonstrates character references can be used for element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_009() {
        let id = "valid-sa-009";
        let content = include_str!("../tests/xmltest/valid/sa/009.xml");
        let section = "2.3 3.1 [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/009.xml");
        let desc = "Test demonstrates that PubidChar can be used for element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_010() {
        let id = "valid-sa-010";
        let content = include_str!("../tests/xmltest/valid/sa/010.xml");
        let section = "3.1 [40]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/010.xml");
        let desc = "Test demonstrates that whitespace is valid after the Attribute in a Start-tag.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_011() {
        let id = "valid-sa-011";
        let content = include_str!("../tests/xmltest/valid/sa/011.xml");
        let section = "3.1 [40]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/011.xml");
        let desc = "Test demonstrates mutliple Attibutes within the Start-tag.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_012() {
        let id = "valid-sa-012";
        let content = include_str!("../tests/xmltest/valid/sa/012.xml");
        let section = "2.3 [4]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/012.xml");
        let desc = "Uses a legal XML 1.0 name consisting of a single colon character (disallowed by the latest XML Namespaces draft).";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_013() {
        let id = "valid-sa-013";
        let content = include_str!("../tests/xmltest/valid/sa/013.xml");
        let section = "2.3 3.1 [13] [40]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/013.xml");
        let desc = "Test demonstrates that the Attribute in a Start-tag can consist of numerals along with special characters.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_014() {
        let id = "valid-sa-014";
        let content = include_str!("../tests/xmltest/valid/sa/014.xml");
        let section = "2.3 3.1 [13] [40]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/014.xml");
        let desc = "Test demonstrates that all lower case letters are valid for the Attribute in a Start-tag.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_015() {
        let id = "valid-sa-015";
        let content = include_str!("../tests/xmltest/valid/sa/015.xml");
        let section = "2.3 3.1 [13] [40]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/015.xml");
        let desc = "Test demonstrates that all upper case letters are valid for the Attribute in a Start-tag.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_016() {
        let id = "valid-sa-016";
        let content = include_str!("../tests/xmltest/valid/sa/016.xml");
        let section = "2.6 3.1 [16] [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/016.xml");
        let desc = "Test demonstrates that Processing Instructions are valid element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_017() {
        let id = "valid-sa-017";
        let content = include_str!("../tests/xmltest/valid/sa/017.xml");
        let section = "2.6 3.1 [16] [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/017.xml");
        let desc = "Test demonstrates that Processing Instructions are valid element content and there can be more than one.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_018() {
        let id = "valid-sa-018";
        let content = include_str!("../tests/xmltest/valid/sa/018.xml");
        let section = "2.7 3.1 [18] [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/018.xml");
        let desc = "Test demonstrates that CDATA sections are valid element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_019() {
        let id = "valid-sa-019";
        let content = include_str!("../tests/xmltest/valid/sa/019.xml");
        let section = "2.7 3.1 [18] [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/019.xml");
        let desc = "Test demonstrates that CDATA sections are valid element content and that ampersands may occur in their literal form.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_020() {
        let id = "valid-sa-020";
        let content = include_str!("../tests/xmltest/valid/sa/020.xml");
        let section = "2.7 3.1 [18] [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/020.xml");
        let desc = "Test demonstractes that CDATA sections are valid element content and that everyting between the CDStart and CDEnd is recognized as character data not markup.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_021() {
        let id = "valid-sa-021";
        let content = include_str!("../tests/xmltest/valid/sa/021.xml");
        let section = "2.5 3.1 [15] [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/021.xml");
        let desc = "Test demonstrates that comments are valid element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_022() {
        let id = "valid-sa-022";
        let content = include_str!("../tests/xmltest/valid/sa/022.xml");
        let section = "2.5 3.1 [15] [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/022.xml");
        let desc = "Test demonstrates that comments are valid element content and that all characters before the double-hypen right angle combination are considered part of thecomment.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_023() {
        let id = "valid-sa-023";
        let content = include_str!("../tests/xmltest/valid/sa/023.xml");
        let section = "3.1 [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/023.xml");
        let desc = "Test demonstrates that Entity References are valid element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_024() {
        let id = "valid-sa-024";
        let content = include_str!("../tests/xmltest/valid/sa/024.xml");
        let section = "3.1 4.1 [43] [66]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/024.xml");
        let desc = "Test demonstrates that Entity References are valid element content and also demonstrates a valid Entity Declaration.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_025() {
        let id = "valid-sa-025";
        let content = include_str!("../tests/xmltest/valid/sa/025.xml");
        let section = "3.2 [46]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/025.xml");
        let desc = "Test demonstrates an Element Type Declaration and that the contentspec can be of mixed content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_026() {
        let id = "valid-sa-026";
        let content = include_str!("../tests/xmltest/valid/sa/026.xml");
        let section = "3.2 [46]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/026.xml");
        let desc =
            "Test demonstrates an Element Type Declaration and that EMPTY is a valid contentspec.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_027() {
        let id = "valid-sa-027";
        let content = include_str!("../tests/xmltest/valid/sa/027.xml");
        let section = "3.2 [46]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/027.xml");
        let desc =
            "Test demonstrates an Element Type Declaration and that ANY is a valid contenspec.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_028() {
        let id = "valid-sa-028";
        let content = include_str!("../tests/xmltest/valid/sa/028.xml");
        let section = "2.8 [24]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/028.xml");
        let desc = "Test demonstrates a valid prolog that uses double quotes as delimeters around the VersionNum.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_029() {
        let id = "valid-sa-029";
        let content = include_str!("../tests/xmltest/valid/sa/029.xml");
        let section = "2.8 [24]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/029.xml");
        let desc = "Test demonstrates a valid prolog that uses single quotes as delimters around the VersionNum.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_030() {
        let id = "valid-sa-030";
        let content = include_str!("../tests/xmltest/valid/sa/030.xml");
        let section = "2.8 [25]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/030.xml");
        let desc = "Test demonstrates a valid prolog that contains whitespace on both sides of the equal sign in the VersionInfo.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_031() {
        let id = "valid-sa-031";
        let content = include_str!("../tests/xmltest/valid/sa/031.xml");
        let section = "4.3.3 [80]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/031.xml");
        let desc = "Test demonstrates a valid EncodingDecl within the prolog.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_032() {
        let id = "valid-sa-032";
        let content = include_str!("../tests/xmltest/valid/sa/032.xml");
        let section = "2.9 [32]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/032.xml");
        let desc = "Test demonstrates a valid SDDecl within the prolog.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_033() {
        let id = "valid-sa-033";
        let content = include_str!("../tests/xmltest/valid/sa/033.xml");
        let section = "2.8 [23]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/033.xml");
        let desc =
            "Test demonstrates that both a EncodingDecl and SDDecl are valid within the prolog.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_034() {
        let id = "valid-sa-034";
        let content = include_str!("../tests/xmltest/valid/sa/034.xml");
        let section = "3.1 [44]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/034.xml");
        let desc = "Test demonstrates the correct syntax for an Empty element tag.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_035() {
        let id = "valid-sa-035";
        let content = include_str!("../tests/xmltest/valid/sa/035.xml");
        let section = "3.1 [44]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/035.xml");
        let desc = "Test demonstrates that whitespace is permissible after the name in an Empty element tag.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_036() {
        let id = "valid-sa-036";
        let content = include_str!("../tests/xmltest/valid/sa/036.xml");
        let section = "2.6 [16]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/036.xml");
        let desc = "Test demonstrates a valid processing instruction.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_037() {
        let id = "valid-sa-037";
        let content = include_str!("../tests/xmltest/valid/sa/037.xml");
        let section = "2.6 [15]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/037.xml");
        let desc = "Test demonstrates a valid comment and that it may appear anywhere in the document including at the end.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_038() {
        let id = "valid-sa-038";
        let content = include_str!("../tests/xmltest/valid/sa/038.xml");
        let section = "2.6 [15]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/038.xml");
        let desc = "Test demonstrates a valid comment and that it may appear anywhere in the document including the beginning.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_039() {
        let id = "valid-sa-039";
        let content = include_str!("../tests/xmltest/valid/sa/039.xml");
        let section = "2.6 [16]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/039.xml");
        let desc = "Test demonstrates a valid processing instruction and that it may appear at the beginning of the document.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_040() {
        let id = "valid-sa-040";
        let content = include_str!("../tests/xmltest/valid/sa/040.xml");
        let section = "3.3 3.3.1 [52] [54]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/040.xml");
        let desc = "Test demonstrates an Attribute List declaration that uses a StringType as the AttType.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_041() {
        let id = "valid-sa-041";
        let content = include_str!("../tests/xmltest/valid/sa/041.xml");
        let section = "3.3.1 4.1 [54] [66]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/041.xml");
        let desc = "Test demonstrates an Attribute List declaration that uses a StringType as the AttType and also expands the CDATA attribute with a character reference.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_042() {
        let id = "valid-sa-042";
        let content = include_str!("../tests/xmltest/valid/sa/042.xml");
        let section = "3.3.1 4.1 [54] [66]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/042.xml");
        let desc = "Test demonstrates an Attribute List declaration that uses a StringType as the AttType and also expands the CDATA attribute with a character reference.  The test also shows that the leading zeros in the character reference are ignored.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_043() {
        let id = "valid-sa-043";
        let content = include_str!("../tests/xmltest/valid/sa/043.xml");
        let section = "3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/043.xml");
        let desc = "An element's attributes may be declared before its content model; and attribute values may contain newlines.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_044() {
        let id = "valid-sa-044";
        let content = include_str!("../tests/xmltest/valid/sa/044.xml");
        let section = "3.1 [44]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/044.xml");
        let desc = "Test demonstrates that the empty-element tag must be use for an elements that are declared EMPTY.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_045() {
        let id = "valid-sa-045";
        let content = include_str!("../tests/xmltest/valid/sa/045.xml");
        let section = "3.3 [52]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/045.xml");
        let desc = "Tests whether more than one definition can be provided for the same attribute of a given element type with the first declaration being binding.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_046() {
        let id = "valid-sa-046";
        let content = include_str!("../tests/xmltest/valid/sa/046.xml");
        let section = "3.3 [52]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/046.xml");
        let desc = "Test demonstrates that when more than one AttlistDecl is provided for a given element type, the contents of all those provided are merged.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_047() {
        let id = "valid-sa-047";
        let content = include_str!("../tests/xmltest/valid/sa/047.xml");
        let section = "3.1 [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/047.xml");
        let desc =
            "Test demonstrates that extra whitespace is normalized into single space character.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_048() {
        let id = "valid-sa-048";
        let content = include_str!("../tests/xmltest/valid/sa/048.xml");
        let section = "2.4 3.1 [14] [43]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/048.xml");
        let desc = "Test demonstrates that character data is valid element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_049() {
        let id = "valid-sa-049";
        let content = unsafe {
            std::str::from_utf8_unchecked(include_bytes!("../tests/xmltest/valid/sa/049.xml"))
        };
        let section = "2.2 [2]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/049.xml");
        let desc = "Test demonstrates that characters outside of normal ascii range can be used as element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_050() {
        let id = "valid-sa-050";
        let content = unsafe {
            std::str::from_utf8_unchecked(include_bytes!("../tests/xmltest/valid/sa/050.xml"))
        };
        let section = "2.2 [2]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/050.xml");
        let desc = "Test demonstrates that characters outside of normal ascii range can be used as element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_051() {
        let id = "valid-sa-051";
        let content = unsafe {
            std::str::from_utf8_unchecked(include_bytes!("../tests/xmltest/valid/sa/051.xml"))
        };
        let section = "2.2 [2]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/051.xml");
        let desc = "The document is encoded in UTF-16 and uses some name characters well outside of the normal ASCII range.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_052() {
        let id = "valid-sa-052";
        let content = include_str!("../tests/xmltest/valid/sa/052.xml");
        let section = "2.2 [2]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/052.xml");
        let desc = "The document is encoded in UTF-8 and the text inside the root element uses two non-ASCII characters, encoded in UTF-8 and each of which expands to a Unicode surrogate pair.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_053() {
        let id = "valid-sa-053";
        let content = include_str!("../tests/xmltest/valid/sa/053.xml");
        let section = "4.4.2";
        let _output = include_str!("../tests/xmltest/valid/sa/out/053.xml");
        let desc = "Tests inclusion of a well-formed internal entity, which holds an element required by the content model.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_054() {
        let id = "valid-sa-054";
        let content = include_str!("../tests/xmltest/valid/sa/054.xml");
        let section = "3.1 [40] [42]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/054.xml");
        let desc = "Test demonstrates that extra whitespace within Start-tags and End-tags are nomalized into single spaces.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_055() {
        let id = "valid-sa-055";
        let content = include_str!("../tests/xmltest/valid/sa/055.xml");
        let section = "2.6 2.10 [16]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/055.xml");
        let desc = "Test demonstrates that extra whitespace within a processing instruction willnormalized into s single space character.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_056() {
        let id = "valid-sa-056";
        let content = include_str!("../tests/xmltest/valid/sa/056.xml");
        let section = "3.3.1 4.1 [54] [66]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/056.xml");
        let desc = "Test demonstrates an Attribute List declaration that uses a StringType as the AttType and also expands the CDATA attribute with a character reference.  The test also shows that the leading zeros in the character reference are ignored.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_057() {
        let id = "valid-sa-057";
        let content = include_str!("../tests/xmltest/valid/sa/057.xml");
        let section = "3.2.1 [47]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/057.xml");
        let desc = "Test demonstrates an element content model whose element can occur zero or more times.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_058() {
        let id = "valid-sa-058";
        let content = include_str!("../tests/xmltest/valid/sa/058.xml");
        let section = "3.3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/058.xml");
        let desc = "Test demonstrates that extra whitespace be normalized into a single space character in an attribute of type NMTOKENS.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_059() {
        let id = "valid-sa-059";
        let content = include_str!("../tests/xmltest/valid/sa/059.xml");
        let section = "3.2 3.3 [46] [53]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/059.xml");
        let desc = "Test demonstrates an Element Type Declaration that uses the contentspec of EMPTY.  The element cannot have any contents and must always appear as an empty element in the document.  The test also shows an Attribute-list declaration with multiple AttDef's.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_060() {
        let id = "valid-sa-060";
        let content = include_str!("../tests/xmltest/valid/sa/060.xml");
        let section = "4.1 [66]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/060.xml");
        let desc =
            "Test demonstrates the use of decimal Character References within element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_061() {
        let id = "valid-sa-061";
        let content = include_str!("../tests/xmltest/valid/sa/061.xml");
        let section = "4.1 [66]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/061.xml");
        let desc =
            "Test demonstrates the use of decimal Character References within element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_062() {
        let id = "valid-sa-062";
        let content = include_str!("../tests/xmltest/valid/sa/062.xml");
        let section = "4.1 [66]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/062.xml");
        let desc = "Test demonstrates the use of hexadecimal Character References within element.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_063() {
        let id = "valid-sa-063";
        let content = include_str!("../tests/xmltest/valid/sa/063.xml");
        let section = "2.3 [5]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/063.xml");
        let desc = "The document is encoded in UTF-8 and the name of the root element type uses non-ASCII characters.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_064() {
        let id = "valid-sa-064";
        let content = include_str!("../tests/xmltest/valid/sa/064.xml");
        let section = "4.1 [66]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/064.xml");
        let desc = "Tests in-line handling of two legal character references, which each expand to a Unicode surrogate pair.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_065() {
        let id = "valid-sa-065";
        let content = include_str!("../tests/xmltest/valid/sa/065.xml");
        let section = "4.5";
        let _output = include_str!("../tests/xmltest/valid/sa/out/065.xml");
        let desc = "Tests ability to define an internal entity which can't legally be expanded (contains an unquoted <B>&lt;</B>).";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_066() {
        let id = "valid-sa-066";
        let content = include_str!("../tests/xmltest/valid/sa/066.xml");
        let section = "4.1 [66]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/066.xml");
        let desc = "Expands a CDATA attribute with a character reference.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_067() {
        let id = "valid-sa-067";
        let content = include_str!("../tests/xmltest/valid/sa/067.xml");
        let section = "4.1 [66]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/067.xml");
        let desc =
            "Test demonstrates the use of decimal character references within element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_068() {
        let id = "valid-sa-068";
        let content = include_str!("../tests/xmltest/valid/sa/068.xml");
        let section = "2.11, 4.5";
        let _output = include_str!("../tests/xmltest/valid/sa/out/068.xml");
        let desc = "Tests definition of an internal entity holding a carriage return character reference, which must not be normalized before reporting to the application.  Line break normalization only occurs when parsing external parsed entities.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_069() {
        let id = "valid-sa-069";
        let content = include_str!("../tests/xmltest/valid/sa/069.xml");
        let section = "4.7";
        let _output = include_str!("../tests/xmltest/valid/sa/out/069.xml");
        let desc = "Verifies that an XML parser will parse a NOTATION declaration; the output phase of this test ensures that it's reported to the application.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_070() {
        let id = "valid-sa-070";
        let content = include_str!("../tests/xmltest/valid/sa/070.xml");
        let section = "4.4.8";
        let _output = include_str!("../tests/xmltest/valid/sa/out/070.xml");
        let desc = "Verifies that internal parameter entities are correctly expanded within the internal subset.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_071() {
        let id = "valid-sa-071";
        let content = include_str!("../tests/xmltest/valid/sa/071.xml");
        let section = "3.3 3.3.1 [52] [56]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/071.xml");
        let desc = "Test demonstrates that an AttlistDecl can use ID as the TokenizedType within the Attribute type.  The test also shows that IMPLIED is a valid DefaultDecl.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_072() {
        let id = "valid-sa-072";
        let content = include_str!("../tests/xmltest/valid/sa/072.xml");
        let section = "3.3 3.3.1 [52] [56]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/072.xml");
        let desc = "Test demonstrates that an AttlistDecl can use IDREF as the TokenizedType within the Attribute type.  The test also shows that IMPLIED is a valid DefaultDecl.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_073() {
        let id = "valid-sa-073";
        let content = include_str!("../tests/xmltest/valid/sa/073.xml");
        let section = "3.3 3.3.1 [52] [56]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/073.xml");
        let desc = "Test demonstrates that an AttlistDecl can use IDREFS as the TokenizedType within the Attribute type.  The test also shows that IMPLIED is a valid DefaultDecl.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_074() {
        let id = "valid-sa-074";
        let content = include_str!("../tests/xmltest/valid/sa/074.xml");
        let section = "3.3 3.3.1 [52] [56]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/074.xml");
        let desc = "Test demonstrates that an AttlistDecl can use ENTITY as the TokenizedType within the Attribute type.  The test also shows that IMPLIED is a valid DefaultDecl.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_075() {
        let id = "valid-sa-075";
        let content = include_str!("../tests/xmltest/valid/sa/075.xml");
        let section = "3.3 3.3.1 [52] [56]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/075.xml");
        let desc = "Test demonstrates that an AttlistDecl can use ENTITIES as the TokenizedType within the Attribute type.  The test also shows that IMPLIED is a valid DefaultDecl.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_076() {
        let id = "valid-sa-076";
        let content = include_str!("../tests/xmltest/valid/sa/076.xml");
        let section = "3.3.1";
        let _output = include_str!("../tests/xmltest/valid/sa/out/076.xml");
        let desc = "Verifies that an XML parser will parse a NOTATION attribute; the output phase of this test ensures that both notations are reported to the application.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_077() {
        let id = "valid-sa-077";
        let content = include_str!("../tests/xmltest/valid/sa/077.xml");
        let section = "3.3 3.3.1 [52] [54]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/077.xml");
        let desc = "Test demonstrates that an AttlistDecl can use an EnumeratedType within the Attribute type.  The test also shows that IMPLIED is a valid DefaultDecl.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_078() {
        let id = "valid-sa-078";
        let content = include_str!("../tests/xmltest/valid/sa/078.xml");
        let section = "3.3 3.3.1 [52] [54]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/078.xml");
        let desc = "Test demonstrates that an AttlistDecl can use an StringType of CDATA within the Attribute type.  The test also shows that REQUIRED is a valid DefaultDecl.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_079() {
        let id = "valid-sa-079";
        let content = include_str!("../tests/xmltest/valid/sa/079.xml");
        let section = "3.3 3.3.2 [52] [60]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/079.xml");
        let desc = "Test demonstrates that an AttlistDecl can use an StringType of CDATA within the Attribute type.  The test also shows that FIXED is a valid DefaultDecl and that a value can be given to the attribute in the Start-tag as well as the AttListDecl.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_080() {
        let id = "valid-sa-080";
        let content = include_str!("../tests/xmltest/valid/sa/080.xml");
        let section = "3.3 3.3.2 [52] [60]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/080.xml");
        let desc = "Test demonstrates that an AttlistDecl can use an StringType of CDATA within the Attribute type.  The test also shows that FIXED is a valid DefaultDecl and that an value can be given to the attribute.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_081() {
        let id = "valid-sa-081";
        let content = include_str!("../tests/xmltest/valid/sa/081.xml");
        let section = "3.2.1 [50]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/081.xml");
        let desc = "Test demonstrates the use of the optional character following a name or list  to govern the number of times an element or content particles in the list occur.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_082() {
        let id = "valid-sa-082";
        let content = include_str!("../tests/xmltest/valid/sa/082.xml");
        let section = "4.2 [72]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/082.xml");
        let desc = "Tests that an external PE may be defined (but not referenced).";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_083() {
        let id = "valid-sa-083";
        let content = include_str!("../tests/xmltest/valid/sa/083.xml");
        let section = "4.2 [72]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/083.xml");
        let desc = "Tests that an external PE may be defined (but not referenced).";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_084() {
        let id = "valid-sa-084";
        let content = include_str!("../tests/xmltest/valid/sa/084.xml");
        let section = "2.10";
        let _output = include_str!("../tests/xmltest/valid/sa/out/084.xml");
        let desc = "Test demonstrates that although whitespace can be used to set apart markup for greater readability it is not necessary.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_085() {
        let id = "valid-sa-085";
        let content = include_str!("../tests/xmltest/valid/sa/085.xml");
        let section = "4";
        let _output = include_str!("../tests/xmltest/valid/sa/out/085.xml");
        let desc = "Parameter and General entities use different namespaces, so there can be an entity of each type with a given name.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_086() {
        let id = "valid-sa-086";
        let content = include_str!("../tests/xmltest/valid/sa/086.xml");
        let section = "4.2";
        let _output = include_str!("../tests/xmltest/valid/sa/out/086.xml");
        let desc = "Tests whether entities may be declared more than once, with the first declaration being the binding one.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_087() {
        let id = "valid-sa-087";
        let content = include_str!("../tests/xmltest/valid/sa/087.xml");
        let section = "4.5";
        let _output = include_str!("../tests/xmltest/valid/sa/out/087.xml");
        let desc = "Tests whether character references in internal entities are expanded early enough, by relying on correct handling to make the entity be well formed.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_088() {
        let id = "valid-sa-088";
        let content = include_str!("../tests/xmltest/valid/sa/088.xml");
        let section = "4.5";
        let _output = include_str!("../tests/xmltest/valid/sa/out/088.xml");
        let desc = "Tests whether entity references in internal entities are expanded late enough, by relying on correct handling to make the expanded text be valid.  (If it's expanded too early, the entity will parse as an element that's not valid in that context.)";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_089() {
        let id = "valid-sa-089";
        let content = include_str!("../tests/xmltest/valid/sa/089.xml");
        let section = "4.1 [66]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/089.xml");
        let desc = "Tests entity expansion of three legal character references, which each expand to a Unicode surrogate pair.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_090() {
        let id = "valid-sa-090";
        let content = include_str!("../tests/xmltest/valid/sa/090.xml");
        let section = "3.3.1";
        let _output = include_str!("../tests/xmltest/valid/sa/out/090.xml");
        let desc = "Verifies that an XML parser will parse a NOTATION attribute; the output phase of this test ensures that the notation is reported to the application.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_091() {
        let id = "valid-sa-091";
        let content = include_str!("../tests/xmltest/valid/sa/091.xml");
        let section = "3.3.1";
        let _output = include_str!("../tests/xmltest/valid/sa/out/091.xml");
        let desc = "Verifies that an XML parser will parse an ENTITY attribute; the output phase of this test ensures that the notation is reported to the application, and for validating parsers it further tests that the entity is so reported.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_092() {
        let id = "valid-sa-092";
        let content = include_str!("../tests/xmltest/valid/sa/092.xml");
        let section = "2.3 2.10";
        let _output = include_str!("../tests/xmltest/valid/sa/out/092.xml");
        let desc =
            "Test demostrates that extra whitespace is normalized into a single space character.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_093() {
        let id = "valid-sa-093";
        let content = include_str!("../tests/xmltest/valid/sa/093.xml");
        let section = "2.10";
        let _output = include_str!("../tests/xmltest/valid/sa/out/093.xml");
        let desc = "Test demonstrates that extra whitespace is not intended for inclusion in the delivered version of the document.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_094() {
        let id = "valid-sa-094";
        let content = include_str!("../tests/xmltest/valid/sa/094.xml");
        let section = "2.8";
        let _output = include_str!("../tests/xmltest/valid/sa/out/094.xml");
        let desc = "Attribute defaults with a DTD have special parsing rules, different from other strings.  That means that characters found there may look like an undefined parameter entity reference 'within a markup declaration', but they aren't ... so they can't be violating the <EM>PEs in Internal Subset</EM> WFC.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_095() {
        let id = "valid-sa-095";
        let content = include_str!("../tests/xmltest/valid/sa/095.xml");
        let section = "3.3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/095.xml");
        let desc = "Basically an output test, this requires extra whitespace to be normalized into a single space character in an attribute of type NMTOKENS.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_096() {
        let id = "valid-sa-096";
        let content = include_str!("../tests/xmltest/valid/sa/096.xml");
        let section = "3.3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/096.xml");
        let desc = "Test demonstrates that extra whitespace is normalized into a single space character in an attribute of type NMTOKENS.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_097() {
        let id = "valid-sa-097";
        let content = include_str!("../tests/xmltest/valid/sa/097.xml");
        let section = "3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/097.xml");
        let desc = "Basically an output test, this tests whether an externally defined attribute declaration (with a default) takes proper precedence over a subsequent internal declaration.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_098() {
        let id = "valid-sa-098";
        let content = include_str!("../tests/xmltest/valid/sa/098.xml");
        let section = "2.6 2.10 [16]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/098.xml");
        let desc = "Test demonstrates that extra whitespace within a processing instruction is converted into a single space character.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_099() {
        let id = "valid-sa-099";
        let content = include_str!("../tests/xmltest/valid/sa/099.xml");
        let section = "4.3.3 [81]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/099.xml");
        let desc =
            "Test demonstrates the name of the encoding can be composed of lowercase characters.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_100() {
        let id = "valid-sa-100";
        let content = include_str!("../tests/xmltest/valid/sa/100.xml");
        let section = "2.3 [12]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/100.xml");
        let desc = "Makes sure that PUBLIC identifiers may have some strange characters.  <EM>NOTE:  The XML editors have said that the XML specification errata will specify that parameter entity expansion does not occur in PUBLIC identifiers, so that the '%' character will not flag a malformed parameter entity reference.</EM>";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_101() {
        let id = "valid-sa-101";
        let content = include_str!("../tests/xmltest/valid/sa/101.xml");
        let section = "4.5";
        let _output = include_str!("../tests/xmltest/valid/sa/out/101.xml");
        let desc = "This tests whether entity expansion is (incorrectly) done while processing entity declarations; if it is, the entity value literal will terminate prematurely.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_102() {
        let id = "valid-sa-102";
        let content = include_str!("../tests/xmltest/valid/sa/102.xml");
        let section = "3.3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/102.xml");
        let desc = "Test demonstrates that a CDATA attribute can pass a double quote as its value.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_103() {
        let id = "valid-sa-103";
        let content = include_str!("../tests/xmltest/valid/sa/103.xml");
        let section = "3.3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/103.xml");
        let desc = "Test demonstrates that an attribute can pass a less than sign as its value.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_104() {
        let id = "valid-sa-104";
        let content = include_str!("../tests/xmltest/valid/sa/104.xml");
        let section = "3.1 [40]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/104.xml");
        let desc = "Test demonstrates that extra whitespace within an Attribute of a Start-tag is normalized to a single space character.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_105() {
        let id = "valid-sa-105";
        let content = include_str!("../tests/xmltest/valid/sa/105.xml");
        let section = "3.3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/105.xml");
        let desc = "Basically an output test, this requires a CDATA attribute with a tab character to be passed through as one space.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_106() {
        let id = "valid-sa-106";
        let content = include_str!("../tests/xmltest/valid/sa/106.xml");
        let section = "3.3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/106.xml");
        let desc = "Basically an output test, this requires a CDATA attribute with a newline character to be passed through as one space.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_107() {
        let id = "valid-sa-107";
        let content = include_str!("../tests/xmltest/valid/sa/107.xml");
        let section = "3.3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/107.xml");
        let desc = "Basically an output test, this requires a CDATA attribute with a return character to be passed through as one space.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_108() {
        let id = "valid-sa-108";
        let content = include_str!("../tests/xmltest/valid/sa/108.xml");
        let section = "2.11, 3.3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/108.xml");
        let desc = "This tests normalization of end-of-line characters (CRLF) within entities to LF, primarily as an output test.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_109() {
        let id = "valid-sa-109";
        let content = include_str!("../tests/xmltest/valid/sa/109.xml");
        let section = "2.3 3.1 [10][40][41]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/109.xml");
        let desc = "Test demonstrates that an attribute can have a null value.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_110() {
        let id = "valid-sa-110";
        let content = include_str!("../tests/xmltest/valid/sa/110.xml");
        let section = "3.3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/110.xml");
        let desc = "Basically an output test, this requires that a CDATA attribute with a CRLF be normalized to one space.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_111() {
        let id = "valid-sa-111";
        let content = include_str!("../tests/xmltest/valid/sa/111.xml");
        let section = "3.3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/111.xml");
        let desc =
            "Character references expanding to spaces doesn't affect treatment of attributes.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_112() {
        let id = "valid-sa-112";
        let content = include_str!("../tests/xmltest/valid/sa/112.xml");
        let section = "3.2.1 [48][49]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/112.xml");
        let desc =
            "Test demonstrates shows the use of content particles within the element content.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_113() {
        let id = "valid-sa-113";
        let content = include_str!("../tests/xmltest/valid/sa/113.xml");
        let section = "3.3 [52][53]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/113.xml");
        let desc = "Test demonstrates that it is not an error to have attributes declared for an element not itself declared.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_114() {
        let id = "valid-sa-114";
        let content = include_str!("../tests/xmltest/valid/sa/114.xml");
        let section = "2.7 [20]";
        let _output = include_str!("../tests/xmltest/valid/sa/out/114.xml");
        let desc = "Test demonstrates that all text within a valid CDATA section is considered text and not recognized as markup.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_115() {
        let id = "valid-sa-115";
        let content = include_str!("../tests/xmltest/valid/sa/115.xml");
        let section = "3.3.3";
        let _output = include_str!("../tests/xmltest/valid/sa/out/115.xml");
        let desc = "Test demonstrates that an entity reference is processed by recursively processing the replacement text of the entity.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_116() {
        let id = "valid-sa-116";
        let content = include_str!("../tests/xmltest/valid/sa/116.xml");
        let section = "2.11";
        let _output = include_str!("../tests/xmltest/valid/sa/out/116.xml");
        let desc = "Test demonstrates that a line break within CDATA will be normalized.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_117() {
        let id = "valid-sa-117";
        let content = include_str!("../tests/xmltest/valid/sa/117.xml");
        let section = "4.5";
        let _output = include_str!("../tests/xmltest/valid/sa/out/117.xml");
        let desc =
            "Test demonstrates that entity expansion is done while processing entity declarations.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_118() {
        let id = "valid-sa-118";
        let content = include_str!("../tests/xmltest/valid/sa/118.xml");
        let section = "4.5";
        let _output = include_str!("../tests/xmltest/valid/sa/out/118.xml");
        let desc =
            "Test demonstrates that entity expansion is done while processing entity declarations.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_sa_119() {
        let id = "valid-sa-119";
        let content = include_str!("../tests/xmltest/valid/sa/119.xml");
        let section = "2.5";
        let _output = include_str!("../tests/xmltest/valid/sa/out/119.xml");
        let desc =
            "Comments may contain any legal XML characters; only the string '--' is disallowed.";
        conformance_valid(id, section, desc, content);
    }

    // <!-- Start:  valid/not-sa -->
    #[test]
    fn conformance_valid_not_sa_001() {
        let id = "valid-not-sa-001";
        let content = include_str!("../tests/xmltest/valid/not-sa/001.xml");
        let section = "4.2.2 [75]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/001.xml");
        let desc = "Test demonstrates the use of an ExternalID within a document type definition.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_002() {
        let id = "valid-not-sa-002";
        let content = include_str!("../tests/xmltest/valid/not-sa/002.xml");
        let section = "4.2.2 [75]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/002.xml");
        let desc = "Test demonstrates the use of an ExternalID within a document type definition.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_003() {
        let id = "valid-not-sa-003";
        let content = include_str!("../tests/xmltest/valid/not-sa/003.xml");
        let section = "4.1 [69]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/003.xml");
        let desc = "Test demonstrates the expansion of an external parameter entity that declares an attribute.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_004() {
        let id = "valid-not-sa-004";
        let content = include_str!("../tests/xmltest/valid/not-sa/004.xml");
        let section = "4.1 [69]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/004.xml");
        let desc = "Expands an external parameter entity in two different ways, with one of them declaring an attribute.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_005() {
        let id = "valid-not-sa-005";
        let content = include_str!("../tests/xmltest/valid/not-sa/005.xml");
        let section = "4.1 [69]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/005.xml");
        let desc = "Test demonstrates the expansion of an external parameter entity that declares an attribute.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_006() {
        let id = "valid-not-sa-006";
        let content = include_str!("../tests/xmltest/valid/not-sa/006.xml");
        let section = "3.3 [52]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/006.xml");
        let desc = "Test demonstrates that when more than one definition is provided for the same attribute of a given element type only the first declaration is binding.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_007() {
        let id = "valid-not-sa-007";
        let content = include_str!("../tests/xmltest/valid/not-sa/007.xml");
        let section = "3.3 [52]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/007.xml");
        let desc =
            "Test demonstrates the use of an Attribute list declaration within an external entity.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_008() {
        let id = "valid-not-sa-008";
        let content = include_str!("../tests/xmltest/valid/not-sa/008.xml");
        let section = "4.2.2 [75]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/008.xml");
        let desc = "Test demonstrates that an external identifier may include a public identifier.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_009() {
        let id = "valid-not-sa-009";
        let content = include_str!("../tests/xmltest/valid/not-sa/009.xml");
        let section = "4.2.2 [75]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/009.xml");
        let desc = "Test demonstrates that an external identifier may include a public identifier.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_010() {
        let id = "valid-not-sa-010";
        let content = include_str!("../tests/xmltest/valid/not-sa/010.xml");
        let section = "3.3 [52]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/010.xml");
        let desc = "Test demonstrates that when more that one definition is provided for the same attribute of a given element type only the first declaration is binding.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_011() {
        let id = "valid-not-sa-011";
        let content = include_str!("../tests/xmltest/valid/not-sa/011.xml");
        let section = "4.2 4.2.1 [72] [75]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/011.xml");
        let desc = "Test demonstrates a parameter entity declaration whose parameter entity definition is an ExternalID.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_012() {
        let id = "valid-not-sa-012";
        let content = include_str!("../tests/xmltest/valid/not-sa/012.xml");
        let section = "4.3.1 [77]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/012.xml");
        let desc =
            "Test demonstrates an enternal parsed entity that begins with a text declaration.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_013() {
        let id = "valid-not-sa-013";
        let content = include_str!("../tests/xmltest/valid/not-sa/013.xml");
        let section = "3.4 [62]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/013.xml");
        let desc = "Test demonstrates the use of the conditional section INCLUDE that will include its contents as part of the DTD.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_014() {
        let id = "valid-not-sa-014";
        let content = include_str!("../tests/xmltest/valid/not-sa/014.xml");
        let section = "3.4 [62]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/014.xml");
        let desc = "Test demonstrates the use of the conditional section INCLUDE that will include its contents as part of the DTD.  The keyword is a parameter-entity reference.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_015() {
        let id = "valid-not-sa-015";
        let content = include_str!("../tests/xmltest/valid/not-sa/015.xml");
        let section = "3.4 [63]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/015.xml");
        let desc = "Test demonstrates the use of the conditonal section IGNORE the will ignore its content from being part of the DTD.  The keyword is a parameter-entity reference.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_016() {
        let id = "valid-not-sa-016";
        let content = include_str!("../tests/xmltest/valid/not-sa/016.xml");
        let section = "3.4 [62]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/016.xml");
        let desc = "Test demonstrates the use of the conditional section INCLUDE that will include its contents as part of the DTD.  The keyword is a parameter-entity reference.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_017() {
        let id = "valid-not-sa-017";
        let content = include_str!("../tests/xmltest/valid/not-sa/017.xml");
        let section = "4.2 [72]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/017.xml");
        let desc = "Test demonstrates a parameter entity declaration that contains an attribute list declaration.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_018() {
        let id = "valid-not-sa-018";
        let content = include_str!("../tests/xmltest/valid/not-sa/018.xml");
        let section = "4.2.2 [75]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/018.xml");
        let desc = "Test demonstrates an EnternalID whose contents contain an parameter entity declaration and a attribute list definition.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_019() {
        let id = "valid-not-sa-019";
        let content = include_str!("../tests/xmltest/valid/not-sa/019.xml");
        let section = "4.4.8";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/019.xml");
        let desc = "Test demonstrates that a parameter entity will be expanded with spaces on either side.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_020() {
        let id = "valid-not-sa-020";
        let content = include_str!("../tests/xmltest/valid/not-sa/020.xml");
        let section = "4.4.8";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/020.xml");
        let desc = "Parameter entities expand with spaces on either side.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_021() {
        let id = "valid-not-sa-021";
        let content = include_str!("../tests/xmltest/valid/not-sa/021.xml");
        let section = "4.2 [72]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/021.xml");
        let desc = "Test demonstrates a parameter entity declaration that contains a partial attribute list declaration.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_023() {
        let id = "valid-not-sa-023";
        let content = include_str!("../tests/xmltest/valid/not-sa/023.xml");
        let section = "2.3 4.1 [10] [69]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/023.xml");
        let desc = "Test demonstrates the use of a parameter entity reference within an attribute list declaration.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_024() {
        let id = "valid-not-sa-024";
        let content = include_str!("../tests/xmltest/valid/not-sa/024.xml");
        let section = "2.8, 4.1 [69]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/024.xml");
        let desc = "Constructs an &lt;!ATTLIST...&gt; declaration from several PEs.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_025() {
        let id = "valid-not-sa-025";
        let content = include_str!("../tests/xmltest/valid/not-sa/025.xml");
        let section = "4.2";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/025.xml");
        let desc = "Test demonstrates that when more that one definition is provided for the same entity only the first declaration is binding.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_026() {
        let id = "valid-not-sa-026";
        let content = include_str!("../tests/xmltest/valid/not-sa/026.xml");
        let section = "3.3 [52]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/026.xml");
        let desc = "Test demonstrates that when more that one definition is provided for the same attribute of a given element type only the first declaration is binding.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_027() {
        let id = "valid-not-sa-027";
        let content = include_str!("../tests/xmltest/valid/not-sa/027.xml");
        let section = "4.1 [69]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/027.xml");
        let desc = "Test demonstrates a parameter entity reference whose value is NULL.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_028() {
        let id = "valid-not-sa-028";
        let content = include_str!("../tests/xmltest/valid/not-sa/028.xml");
        let section = "3.4 [62]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/028.xml");
        let desc = "Test demonstrates the use of the conditional section INCLUDE that will include its contents.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_029() {
        let id = "valid-not-sa-029";
        let content = include_str!("../tests/xmltest/valid/not-sa/029.xml");
        let section = "3.4 [62]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/029.xml");
        let desc = "Test demonstrates the use of the conditonal section IGNORE the will ignore its content from being used.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_030() {
        let id = "valid-not-sa-030";
        let content = include_str!("../tests/xmltest/valid/not-sa/030.xml");
        let section = "3.4 [62]";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/030.xml");
        let desc = "Test demonstrates the use of the conditonal section IGNORE the will ignore its content from being used.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_not_sa_031() {
        let id = "valid-not-sa-031";
        let content = include_str!("../tests/xmltest/valid/not-sa/031.xml");
        let section = "2.7";
        let _output = include_str!("../tests/xmltest/valid/not-sa/out/031.xml");
        let desc = "Expands a general entity which contains a CDATA section with what looks like a markup declaration (but is just text since it's in a CDATA section).";
        conformance_valid(id, section, desc, content);
    }

    // <!-- Start:  valid/ext-sa -->
    #[test]
    fn conformance_valid_ext_sa_001() {
        let id = "valid-ext-sa-001";
        let content = include_str!("../tests/xmltest/valid/ext-sa/001.xml");
        let section = "2.11";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/001.xml");
        let desc = "A combination of carriage return line feed in an external entity must be normalized to a single newline.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_ext_sa_002() {
        let id = "valid-ext-sa-002";
        let content = include_str!("../tests/xmltest/valid/ext-sa/002.xml");
        let section = "2.11";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/002.xml");
        let desc = "A carriage return (also CRLF) in an external entity must be normalized to a single newline.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_ext_sa_003() {
        let id = "valid-ext-sa-003";
        let content = include_str!("../tests/xmltest/valid/ext-sa/003.xml");
        let section = "3.1 4.1 [43] [68]";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/003.xml");
        let desc = "Test demonstrates that the content of an element can be empty. In this case the external entity is an empty file.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_ext_sa_004() {
        let id = "valid-ext-sa-004";
        let content = include_str!("../tests/xmltest/valid/ext-sa/004.xml");
        let section = "2.11";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/004.xml");
        let desc = "A carriage return (also CRLF) in an external entity must be normalized to a single newline.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_ext_sa_005() {
        let id = "valid-ext-sa-005";
        let content = include_str!("../tests/xmltest/valid/ext-sa/005.xml");
        let section = "3.2.1 4.2.2 [48] [75]";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/005.xml");
        let desc = "Test demonstrates the use of optional character and content particles within an element content.  The test also show the use of external entity.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_ext_sa_006() {
        let id = "valid-ext-sa-006";
        let content = include_str!("../tests/xmltest/valid/ext-sa/006.xml");
        let section = "2.11 3.2.1 3.2.2 4.2.2 [48] [51] [75]";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/006.xml");
        let desc = "Test demonstrates the use of optional character and content particles within mixed element content.  The test also shows the use of an external entity and that a carriage control line feed in an external entity must be normalized to a single newline.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_ext_sa_007() {
        let id = "valid-ext-sa-007";
        let content = include_str!("../tests/xmltest/valid/ext-sa/007.xml");
        let section = "4.2.2 4.4.3 [75]";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/007.xml");
        let desc = "Test demonstrates the use of external entity and how replacement text is retrieved and processed.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_ext_sa_008() {
        let id = "valid-ext-sa-008";
        let content = include_str!("../tests/xmltest/valid/ext-sa/008.xml");
        let section = "4.2.2 4.3.3. 4.4.3 [75] [80]";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/008.xml");
        let desc = "Test demonstrates the use of external entity and how replacement text is retrieved and processed.  Also tests the use of an EncodingDecl of UTF-16.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_ext_sa_009() {
        let id = "valid-ext-sa-009";
        let content = include_str!("../tests/xmltest/valid/ext-sa/009.xml");
        let section = "2.11";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/009.xml");
        let desc = "A carriage return (also CRLF) in an external entity must be normalized to a single newline.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_ext_sa_011() {
        let id = "valid-ext-sa-011";
        let content = include_str!("../tests/xmltest/valid/ext-sa/011.xml");
        let section = "2.11 4.2.2 [75]";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/011.xml");
        let desc = "Test demonstrates the use of a public identifier with and external entity.  The test also show that a carriage control line feed combination in an external entity must be normalized to a single newline.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_ext_sa_012() {
        let id = "valid-ext-sa-012";
        let content = include_str!("../tests/xmltest/valid/ext-sa/012.xml");
        let section = "4.2.1 4.2.2";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/012.xml");
        let desc = "Test demonstrates both internal and external entities and that processing of entity references may be required to produce the correct replacement text.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_ext_sa_013() {
        let id = "valid-ext-sa-013";
        let content = include_str!("../tests/xmltest/valid/ext-sa/013.xml");
        let section = "3.3.3";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/013.xml");
        let desc = "Test demonstrates that whitespace is handled by adding a single whitespace to the normalized value in the attribute list.";
        conformance_valid(id, section, desc, content);
    }

    #[test]
    fn conformance_valid_ext_sa_014() {
        let id = "valid-ext-sa-014";
        let content = include_str!("../tests/xmltest/valid/ext-sa/014.xml");
        let section = "4.1 4.4.3 [68]";
        let _output = include_str!("../tests/xmltest/valid/ext-sa/out/014.xml");
        let desc = "Test demonstrates use of characters outside of normal ASCII range.";
        conformance_valid(id, section, desc, content);
    }
}
