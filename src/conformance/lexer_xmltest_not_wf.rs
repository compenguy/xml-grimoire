/* Conformance tests of not well-formed xml documents.
 * See /tests/xmltest/xmltest.xml for short descriptions
 * of each test document.
 */

#[cfg(test)]
mod tests {
    use lexer::Rule;
    use lexer::XmlLexer1_0;
    use pest::Parser;

    fn conformance_not_wf(id: &str, section: &str, desc: &str, content: &str) {
        println!("[{}] {} (Section {})", id, desc, section);
        let rule = Rule::document;
        let _parsed = match XmlLexer1_0::parse(rule, content) {
            Err(e) => panic!("Lexing the XML content failed: {}", e),
            Ok(p) => p,
        };
    }

    /* <!-- Start:  not-wf/sa --> */
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_001() {
        let id = "not-wf-sa-001";
        let content = include_str!("../../tests/xmltest/not-wf/sa/001.xml");
        let section = "3.1 [41]";
        let desc = "Attribute values must start with attribute names, not '?'";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_002() {
        let id = "not-wf-sa-002";
        let content = include_str!("../../tests/xmltest/not-wf/sa/002.xml");
        let section = "2.3 [4]";
        let desc = "Names may not start with '.'; it's not a Letter";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_003() {
        let id = "not-wf-sa-003";
        let content = include_str!("../../tests/xmltest/not-wf/sa/003.xml");
        let section = "2.6 [16]";
        let desc = "Processing Instruction target name is required";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_004() {
        let id = "not-wf-sa-004";
        let content = include_str!("../../tests/xmltest/not-wf/sa/004.xml");
        let section = "2.6 [16]";
        let desc = "SGML-ism: processing instructions end in '?&gt;' not '&gt;'";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_005() {
        let id = "not-wf-sa-005";
        let content = include_str!("../../tests/xmltest/not-wf/sa/004.xml");
        let section = "2.6 [16]";
        let desc = "Processing instructions end in '?&gt;' not '?'";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_006() {
        let id = "not-wf-sa-006";
        let content = include_str!("../../tests/xmltest/not-wf/sa/006.xml");
        let section = "section 2.5 [16]";
        let desc = "XML comments may not contain '--'";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_007() {
        let id = "not-wf-sa-007";
        let content = include_str!("../../tests/xmltest/not-wf/sa/007.xml");
        let section = "section 4.1 [68]";
        let desc = "General entity references have no whitespace after the entity name and before the semicolon.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_008() {
        let id = "not-wf-sa-008";
        let content = include_str!("../../tests/xmltest/not-wf/sa/008.xml");
        let section = "section 2.3 [5]";
        let desc = "Entity references must include names, which don't begin with '.' (it's not a Letter or other name start character).";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_009() {
        let id = "not-wf-sa-009";
        let content = include_str!("../../tests/xmltest/not-wf/sa/009.xml");
        let section = "section 4.1 [66]";
        let desc = "Character references may have only decimal or numeric strings.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_010() {
        let id = "not-wf-sa-010";
        let content = include_str!("../../tests/xmltest/not-wf/sa/010.xml");
        let section = "section 4.1 [68]";
        let desc = "Ampersand may only appear as part of a general entity reference.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_011() {
        let id = "not-wf-sa-011";
        let content = include_str!("../../tests/xmltest/not-wf/sa/011.xml");
        let section = "section 3.1 [41]";
        let desc = "SGML-ism:  attribute values must be explicitly assigned a value, it can't act as a boolean toggle.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_012() {
        let id = "not-wf-sa-012";
        let content = include_str!("../../tests/xmltest/not-wf/sa/012.xml");
        let section = "section 2.3 [10]";
        let desc = "SGML-ism:  attribute values must be quoted in all cases.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_013() {
        let id = "not-wf-sa-013";
        let content = include_str!("../../tests/xmltest/not-wf/sa/013.xml");
        let section = "section 2.3 [10]";
        let desc = "The quotes on both ends of an attribute value must match.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_014() {
        let id = "not-wf-sa-014";
        let content = include_str!("../../tests/xmltest/not-wf/sa/014.xml");
        let section = "section 2.3 [10]";
        let desc = "Attribute values may not contain literal '&lt;' characters.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_015() {
        let id = "not-wf-sa-015";
        let content = include_str!("../../tests/xmltest/not-wf/sa/015.xml");
        let section = "section 3.1 [41]";
        let desc = "Attribute values need a value, not just an equals sign.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_016() {
        let id = "not-wf-sa-016";
        let content = include_str!("../../tests/xmltest/not-wf/sa/016.xml");
        let section = "section 3.1 [41]";
        let desc = "Attribute values need an associated name.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_017() {
        let id = "not-wf-sa-017";
        let content = include_str!("../../tests/xmltest/not-wf/sa/017.xml");
        let section = "section 2.7 [18]";
        let desc = "CDATA sections need a terminating ']]&gt;'.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_018() {
        let id = "not-wf-sa-018";
        let content = include_str!("../../tests/xmltest/not-wf/sa/018.xml");
        let section = "section 2.7 [19]";
        let desc = "CDATA sections begin with a literal '&lt;![CDATA[', no space.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_019() {
        let id = "not-wf-sa-019";
        let content = include_str!("../../tests/xmltest/not-wf/sa/019.xml");
        let section = "section 3.1 [42]";
        let desc = "End tags may not be abbreviated as '&lt;/&gt;'.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_020() {
        let id = "not-wf-sa-020";
        let content = include_str!("../../tests/xmltest/not-wf/sa/020.xml");
        let section = "section 2.3 [10]";
        let desc = "Attribute values may not contain literal '&amp;' characters except as part of an entity reference.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_021() {
        let id = "not-wf-sa-021";
        let content = include_str!("../../tests/xmltest/not-wf/sa/021.xml");
        let section = "section 2.3 [10]";
        let desc = "Attribute values may not contain literal '&amp;' characters except as part of an entity reference.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_022() {
        let id = "not-wf-sa-022";
        let content = include_str!("../../tests/xmltest/not-wf/sa/022.xml");
        let section = "section 4.1 [66]";
        let desc = "Character references end with semicolons, always!";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_023() {
        let id = "not-wf-sa-023";
        let content = include_str!("../../tests/xmltest/not-wf/sa/023.xml");
        let section = "section 2.3 [5]";
        let desc = "Digits are not valid name start characters.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_024() {
        let id = "not-wf-sa-024";
        let content = include_str!("../../tests/xmltest/not-wf/sa/024.xml");
        let section = "section 2.3 [5]";
        let desc = "Digits are not valid name start characters.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_025() {
        let id = "not-wf-sa-025";
        let content = include_str!("../../tests/xmltest/not-wf/sa/025.xml");
        let section = "section 2.4 [14]";
        let desc = "Text may not contain a literal ']]&gt;' sequence.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_026() {
        let id = "not-wf-sa-026";
        let content = include_str!("../../tests/xmltest/not-wf/sa/026.xml");
        let section = "section 2.4 [14]";
        let desc = "Text may not contain a literal ']]&gt;' sequence.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_027() {
        let id = "not-wf-sa-027";
        let content = include_str!("../../tests/xmltest/not-wf/sa/027.xml");
        let section = "section 2.5 [15]";
        let desc = "Comments must be terminated with '--&gt;'.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_028() {
        let id = "not-wf-sa-028";
        let content = include_str!("../../tests/xmltest/not-wf/sa/028.xml");
        let section = "section 2.6 [16]";
        let desc = "Processing instructions must end with '?&gt;'.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_029() {
        let id = "not-wf-sa-029";
        let content = include_str!("../../tests/xmltest/not-wf/sa/029.xml");
        let section = "section 2.4 [14]";
        let desc = "Text may not contain a literal ']]&gt;' sequence.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_030() {
        let id = "not-wf-sa-030";
        let content = include_str!("../../tests/xmltest/not-wf/sa/030.xml");
        let section = "section 2.2 [2]";
        let desc = "A form feed is not a legal XML character.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_031() {
        let id = "not-wf-sa-031";
        let content = include_str!("../../tests/xmltest/not-wf/sa/031.xml");
        let section = "section 2.2 [2]";
        let desc = "A form feed is not a legal XML character.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_032() {
        let id = "not-wf-sa-032";
        let content = include_str!("../../tests/xmltest/not-wf/sa/032.xml");
        let section = "section 2.2 [2]";
        let desc = "A form feed is not a legal XML character.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_033() {
        let id = "not-wf-sa-033";
        let content = include_str!("../../tests/xmltest/not-wf/sa/033.xml");
        let section = "section 2.2 [2]";
        let desc = "An ESC (octal 033) is not a legal XML character.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_034() {
        let id = "not-wf-sa-034";
        let content = include_str!("../../tests/xmltest/not-wf/sa/034.xml");
        let section = "section 2.2 [2]";
        let desc = "A form feed is not a legal XML character.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_035() {
        let id = "not-wf-sa-035";
        let content = include_str!("../../tests/xmltest/not-wf/sa/035.xml");
        let section = "section 3.1 [43]";
        let desc = "The '&lt;' character is a markup delimiter and must start an element, CDATA section, PI, or comment.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_036() {
        let id = "not-wf-sa-036";
        let content = include_str!("../../tests/xmltest/not-wf/sa/036.xml");
        let section = "section 2.8 [27]";
        let desc = "Text may not appear after the root element.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_037() {
        let id = "not-wf-sa-037";
        let content = include_str!("../../tests/xmltest/not-wf/sa/037.xml");
        let section = "section 2.8 [27]";
        let desc = "Character references may not appear after the root element.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test presumes knowledge of a previously emitted token, which is
    // something we don't track at this level, but should be up to the
    // parser to enforce.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_038() {
        let id = "not-wf-sa-038";
        let content = include_str!("../../tests/xmltest/not-wf/sa/038.xml");
        let section = "section 3.1";
        let desc = "Tests the 'Unique Att Spec' WF constraint by providing multiple values for an attribute.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test presumes knowledge of a previously emitted token, which is
    // something we don't track at this level, but should be up to the
    // parser to enforce.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_039() {
        let id = "not-wf-sa-039";
        let content = include_str!("../../tests/xmltest/not-wf/sa/039.xml");
        let section = "3";
        let desc = "Tests the Element Type Match WFC - end tag name must match start tag name.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_040() {
        let id = "not-wf-sa-040";
        let content = include_str!("../../tests/xmltest/not-wf/sa/040.xml");
        let section = "section 2.8 [27]";
        let desc = "Provides two document elements.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_041() {
        let id = "not-wf-sa-041";
        let content = include_str!("../../tests/xmltest/not-wf/sa/041.xml");
        let section = "section 2.8 [27]";
        let desc = "Provides two document elements.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_042() {
        let id = "not-wf-sa-042";
        let content = include_str!("../../tests/xmltest/not-wf/sa/042.xml");
        let section = "section 3.1 [42]";
        let desc = "Invalid End Tag";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_043() {
        let id = "not-wf-sa-043";
        let content = include_str!("../../tests/xmltest/not-wf/sa/043.xml");
        let section = "section 2.8 [27]";
        let desc = "Provides #PCDATA text after the document element.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_044() {
        let id = "not-wf-sa-044";
        let content = include_str!("../../tests/xmltest/not-wf/sa/044.xml");
        let section = "section 2.8 [27]";
        let desc = "Provides two document elements.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_045() {
        let id = "not-wf-sa-045";
        let content = include_str!("../../tests/xmltest/not-wf/sa/045.xml");
        let section = "section 3.1 [44]";
        let desc = "Invalid Empty Element Tag";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_046() {
        let id = "not-wf-sa-046";
        let content = include_str!("../../tests/xmltest/not-wf/sa/046.xml");
        let section = "section 3.1 [40]";
        let desc = "This start (or empty element) tag was not terminated correctly.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_047() {
        let id = "not-wf-sa-047";
        let content = include_str!("../../tests/xmltest/not-wf/sa/047.xml");
        let section = "section 3.1 [44]";
        let desc = "Invalid empty element tag invalid whitespace";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_048() {
        let id = "not-wf-sa-048";
        let content = include_str!("../../tests/xmltest/not-wf/sa/048.xml");
        let section = "section 2.8 [27]";
        let desc = "Provides a CDATA section after the roor element.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_049() {
        let id = "not-wf-sa-049";
        let content = include_str!("../../tests/xmltest/not-wf/sa/049.xml");
        let section = "section 3.1 [40]";
        let desc = "Missing start tag";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_050() {
        let id = "not-wf-sa-050";
        let content = include_str!("../../tests/xmltest/not-wf/sa/050.xml");
        let section = "section 2.1 [1]";
        let desc = "Empty document, with no root element.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_051() {
        let id = "not-wf-sa-051";
        let content = include_str!("../../tests/xmltest/not-wf/sa/051.xml");
        let section = "section 2.7 [18]";
        let desc = "CDATA is invalid at top level of document.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_052() {
        let id = "not-wf-sa-052";
        let content = include_str!("../../tests/xmltest/not-wf/sa/052.xml");
        let section = "section 4.1 [66]";
        let desc = "Invalid character reference.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test presumes knowledge of a previously emitted token, which is
    // something we don't track at this level, but should be up to the
    // parser to enforce.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_053() {
        let id = "not-wf-sa-053";
        let content = include_str!("../../tests/xmltest/not-wf/sa/053.xml");
        let section = "section 3.1 [42]";
        let desc = "End tag does not match start tag.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_054() {
        let id = "not-wf-sa-054";
        let content = include_str!("../../tests/xmltest/not-wf/sa/054.xml");
        let section = "section 4.2.2 [75]";
        let desc = "PUBLIC requires two literals.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_055() {
        let id = "not-wf-sa-055";
        let content = include_str!("../../tests/xmltest/not-wf/sa/055.xml");
        let section = "section 2.8 [28]";
        let desc = "Invalid Document Type Definition format.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_056() {
        let id = "not-wf-sa-056";
        let content = include_str!("../../tests/xmltest/not-wf/sa/056.xml");
        let section = "section 2.8 [28]";
        let desc = "Invalid Document Type Definition format - misplaced comment.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_057() {
        let id = "not-wf-sa-057";
        let content = include_str!("../../tests/xmltest/not-wf/sa/057.xml");
        let section = "section 3.2 [45]";
        let desc = "This isn't SGML; comments can't exist in declarations.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_058() {
        let id = "not-wf-sa-058";
        let content = include_str!("../../tests/xmltest/not-wf/sa/058.xml");
        let section = "section 3.3.1 [54]";
        let desc = "Invalid character , in ATTLIST enumeration";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_059() {
        let id = "not-wf-sa-059";
        let content = include_str!("../../tests/xmltest/not-wf/sa/059.xml");
        let section = "section 3.3.1 [59]";
        let desc = "String literal must be in quotes.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_060() {
        let id = "not-wf-sa-060";
        let content = include_str!("../../tests/xmltest/not-wf/sa/060.xml");
        let section = "section 3.3.1 [56]";
        let desc = "Invalid type NAME defined in ATTLIST.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_061() {
        let id = "not-wf-sa-061";
        let content = include_str!("../../tests/xmltest/not-wf/sa/061.xml");
        let section = "section 4.2.2 [75]";
        let desc = "External entity declarations require whitespace between public and system IDs.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_062() {
        let id = "not-wf-sa-062";
        let content = include_str!("../../tests/xmltest/not-wf/sa/062.xml");
        let section = "section 4.2 [71]";
        let desc = "Entity declarations need space after the entity name.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_063() {
        let id = "not-wf-sa-063";
        let content = include_str!("../../tests/xmltest/not-wf/sa/063.xml");
        let section = "section 2.8 [29]";
        let desc = "Conditional sections may only appear in the external DTD subset.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_064() {
        let id = "not-wf-sa-064";
        let content = include_str!("../../tests/xmltest/not-wf/sa/064.xml");
        let section = "section 3.3 [53]";
        let desc = "Space is required between attribute type and default values in &lt;!ATTLIST...&gt; declarations.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_065() {
        let id = "not-wf-sa-065";
        let content = include_str!("../../tests/xmltest/not-wf/sa/065.xml");
        let section = "section 3.3 [53]";
        let desc = "Space is required between attribute name and type in &lt;!ATTLIST...&gt; declarations.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_066() {
        let id = "not-wf-sa-066";
        let content = include_str!("../../tests/xmltest/not-wf/sa/066.xml");
        let section = "section 3.3 [52]";
        let desc = "Required whitespace is missing.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_067() {
        let id = "not-wf-sa-067";
        let content = include_str!("../../tests/xmltest/not-wf/sa/067.xml");
        let section = "section 3.3 [53]";
        let desc = "Space is required between attribute type and default values in &lt;!ATTLIST...&gt; declarations.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_068() {
        let id = "not-wf-sa-068";
        let content = include_str!("../../tests/xmltest/not-wf/sa/068.xml");
        let section = "section 3.3.1 [58]";
        let desc = "Space is required between NOTATION keyword and list of enumerated choices in &lt;!ATTLIST...&gt; declarations.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_069() {
        let id = "not-wf-sa-069";
        let content = include_str!("../../tests/xmltest/not-wf/sa/069.xml");
        let section = "section 4.2.2 [76]";
        let desc = "Space is required before an NDATA entity annotation.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_070() {
        let id = "not-wf-sa-070";
        let content = include_str!("../../tests/xmltest/not-wf/sa/070.xml");
        let section = "section 2.5 [16]";
        let desc = "XML comments may not contain '--'";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to semantically verify DOCTYPE
    // definitions, something we don't yet do.  See "Known Parsing Gaps"
    // in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_071() {
        let id = "not-wf-sa-071";
        let content = include_str!("../../tests/xmltest/not-wf/sa/071.xml");
        let section = "section 4.1 [68]";
        let desc = "ENTITY can't reference itself directly or indirectly.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_072() {
        let id = "not-wf-sa-072";
        let content = include_str!("../../tests/xmltest/not-wf/sa/072.xml");
        let section = "section 4.1 [68]";
        let desc = "Undefined ENTITY foo.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_073() {
        let id = "not-wf-sa-073";
        let content = include_str!("../../tests/xmltest/not-wf/sa/073.xml");
        let section = "section 4.1 [68]";
        let desc = "Undefined ENTITY f.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_074() {
        let id = "not-wf-sa-074";
        let content = include_str!("../../tests/xmltest/not-wf/sa/074.xml");
        let section = "section 4.3.2";
        let desc = "Internal general parsed entities are only well formed if they match the 'content' production.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_075() {
        let id = "not-wf-sa-075";
        let content = include_str!("../../tests/xmltest/not-wf/sa/075.xml");
        let section = "section 4.1 [68]";
        let desc = "ENTITY can't reference itself directly or indirectly.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_076() {
        let id = "not-wf-sa-076";
        let content = include_str!("../../tests/xmltest/not-wf/sa/076.xml");
        let section = "section 4.1 [68]";
        let desc = "Undefined ENTITY foo.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_077() {
        let id = "not-wf-sa-077";
        let content = include_str!("../../tests/xmltest/not-wf/sa/077.xml");
        let section = "section 41. [68]";
        let desc = "Undefined ENTITY bar.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_078() {
        let id = "not-wf-sa-078";
        let content = include_str!("../../tests/xmltest/not-wf/sa/078.xml");
        let section = "section 4.1 [68]";
        let desc = "Undefined ENTITY foo.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_079() {
        let id = "not-wf-sa-079";
        let content = include_str!("../../tests/xmltest/not-wf/sa/079.xml");
        let section = "section 4.1 [68]";
        let desc = "ENTITY can't reference itself directly or indirectly.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_080() {
        let id = "not-wf-sa-080";
        let content = include_str!("../../tests/xmltest/not-wf/sa/080.xml");
        let section = "section 4.1 [68]";
        let desc = "ENTITY can't reference itself directly or indirectly.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_081() {
        let id = "not-wf-sa-081";
        let content = include_str!("../../tests/xmltest/not-wf/sa/081.xml");
        let section = "section 3.1";
        let desc = "This tests the No External Entity References WFC, since the entity is referred to within an attribute.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_082() {
        let id = "not-wf-sa-082";
        let content = include_str!("../../tests/xmltest/not-wf/sa/082.xml");
        let section = "section 3.1";
        let desc = "This tests the No External Entity References WFC, since the entity is referred to within an attribute.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_083() {
        let id = "not-wf-sa-083";
        let content = include_str!("../../tests/xmltest/not-wf/sa/083.xml");
        let section = "section 4.2.2 [76]";
        let desc = "Undefined NOTATION n.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_084() {
        let id = "not-wf-sa-084";
        let content = include_str!("../../tests/xmltest/not-wf/sa/084.xml");
        let section = "section 4.1";
        let desc = "Tests the Parsed Entity WFC by referring to an unparsed entity.  (This precedes the error of not declaring that entity's notation, which may be detected any time before the DTD parsing is completed.)";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_085() {
        let id = "not-wf-sa-085";
        let content = include_str!("../../tests/xmltest/not-wf/sa/085.xml");
        let section = "section 2.3 [13]";
        let desc = "Public IDs may not contain '['.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_086() {
        let id = "not-wf-sa-086";
        let content = include_str!("../../tests/xmltest/not-wf/sa/086.xml");
        let section = "section 2.3 [13]";
        let desc = "Public IDs may not contain '['.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_087() {
        let id = "not-wf-sa-087";
        let content = include_str!("../../tests/xmltest/not-wf/sa/087.xml");
        let section = "section 2.3 [13]";
        let desc = "Public IDs may not contain '['.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_088() {
        let id = "not-wf-sa-088";
        let content = include_str!("../../tests/xmltest/not-wf/sa/088.xml");
        let section = "section 2.3 [10]";
        let desc = "Attribute values are terminated by literal quote characters, and any entity expansion is done afterwards.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_089() {
        let id = "not-wf-sa-089";
        let content = include_str!("../../tests/xmltest/not-wf/sa/089.xml");
        let section = "section 4.2 [74]";
        let desc = "Parameter entities *are* always parsed; NDATA annotations are not permitted.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_090() {
        let id = "not-wf-sa-090";
        let content = include_str!("../../tests/xmltest/not-wf/sa/090.xml");
        let section = "section 2.3 [10]";
        let desc = "Attributes may not contain a literal '&lt;' character; this one has one because of reference expansion.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_091() {
        let id = "not-wf-sa-091";
        let content = include_str!("../../tests/xmltest/not-wf/sa/091.xml");
        let section = "section 4.2 [74]";
        let desc = "Parameter entities *are* always parsed; NDATA annotations are not permitted.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_092() {
        let id = "not-wf-sa-092";
        let content = include_str!("../../tests/xmltest/not-wf/sa/092.xml");
        let section = "section 4.5";
        let desc = "The replacement text of this entity has an illegal reference, because the character reference is expanded immediately.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_093() {
        let id = "not-wf-sa-093";
        let content = include_str!("../../tests/xmltest/not-wf/sa/093.xml");
        let section = "section 4.1 [66]";
        let desc = "Hexadecimal character references may not use the uppercase 'X'.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_094() {
        let id = "not-wf-sa-094";
        let content = include_str!("../../tests/xmltest/not-wf/sa/094.xml");
        let section = "section 2.8 [24]";
        let desc = "Prolog VERSION must be lowercase.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_095() {
        let id = "not-wf-sa-095";
        let content = include_str!("../../tests/xmltest/not-wf/sa/095.xml");
        let section = "section 2.8 [23]";
        let desc = "VersionInfo must come before EncodingDecl.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_096() {
        let id = "not-wf-sa-096";
        let content = include_str!("../../tests/xmltest/not-wf/sa/096.xml");
        let section = "section 2.9 [32]";
        let desc = "Space is required before the standalone declaration.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_097() {
        let id = "not-wf-sa-097";
        let content = include_str!("../../tests/xmltest/not-wf/sa/097.xml");
        let section = "section 2.8 [24]";
        let desc = "Both quotes surrounding VersionNum must be the same.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_098() {
        let id = "not-wf-sa-098";
        let content = include_str!("../../tests/xmltest/not-wf/sa/098.xml");
        let section = "section 2.8 [23]";
        let desc = "Only one 'version=...' string may appear in an XML declaration.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_099() {
        let id = "not-wf-sa-099";
        let content = include_str!("../../tests/xmltest/not-wf/sa/099.xml");
        let section = "section 2.8 [23]";
        let desc = "Only three pseudo-attributes are in the XML declaration, and 'valid=...' is not one of them.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_100() {
        let id = "not-wf-sa-100";
        let content = include_str!("../../tests/xmltest/not-wf/sa/100.xml");
        let section = "section 2.9 [32]";
        let desc = "Only 'yes' and 'no' are permitted as values of 'standalone'.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_101() {
        let id = "not-wf-sa-101";
        let content = include_str!("../../tests/xmltest/not-wf/sa/101.xml");
        let section = "section 4.3.3 [81]";
        let desc = "Space is not permitted in an encoding name.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_102() {
        let id = "not-wf-sa-102";
        let content = include_str!("../../tests/xmltest/not-wf/sa/102.xml");
        let section = "section 2.8 [26]";
        let desc = "Provides an illegal XML version number; spaces are illegal.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_103() {
        let id = "not-wf-sa-103";
        let content = include_str!("../../tests/xmltest/not-wf/sa/103.xml");
        let section = "section 4.3.2";
        let desc = "End-tag required for element foo.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_104() {
        let id = "not-wf-sa-104";
        let content = include_str!("../../tests/xmltest/not-wf/sa/104.xml");
        let section = "section 4.3.2";
        let desc = "Internal general parsed entities are only well formed if they match the 'content' production.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_105() {
        let id = "not-wf-sa-105";
        let content = include_str!("../../tests/xmltest/not-wf/sa/105.xml");
        let section = "section 2.7";
        let desc = "Invalid placement of CDATA section.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_106() {
        let id = "not-wf-sa-106";
        let content = include_str!("../../tests/xmltest/not-wf/sa/106.xml");
        let section = "section 4.2";
        let desc = "Invalid placement of entity declaration.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_107() {
        let id = "not-wf-sa-107";
        let content = include_str!("../../tests/xmltest/not-wf/sa/107.xml");
        let section = "section 2.8 [28]";
        let desc = "Invalid document type declaration.  CDATA alone is invalid.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_108() {
        let id = "not-wf-sa-108";
        let content = include_str!("../../tests/xmltest/not-wf/sa/108.xml");
        let section = "section 2.7 [19]";
        let desc = "No space in '&lt;![CDATA['.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_109() {
        let id = "not-wf-sa-109";
        let content = include_str!("../../tests/xmltest/not-wf/sa/109.xml");
        let section = "section 4.2 [70]";
        let desc = "Tags invalid within EntityDecl.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_110() {
        let id = "not-wf-sa-110";
        let content = include_str!("../../tests/xmltest/not-wf/sa/110.xml");
        let section = "section 4.1 [68]";
        let desc = "Entity reference must be in content of element.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_111() {
        let id = "not-wf-sa-111";
        let content = include_str!("../../tests/xmltest/not-wf/sa/111.xml");
        let section = "section 3.1 [43]";
        let desc = "Entiry reference must be in content of element not Start-tag.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_112() {
        let id = "not-wf-sa-112";
        let content = include_str!("../../tests/xmltest/not-wf/sa/112.xml");
        let section = "section 2.7 [19]";
        let desc = "CDATA sections start '&lt;![CDATA[', not '&lt;!cdata['.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_113() {
        let id = "not-wf-sa-113";
        let content = include_str!("../../tests/xmltest/not-wf/sa/113.xml");
        let section = "section 2.3 [9]";
        let desc =
            "Parameter entity values must use valid reference syntax; this reference is malformed.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_114() {
        let id = "not-wf-sa-114";
        let content = include_str!("../../tests/xmltest/not-wf/sa/114.xml");
        let section = "section 2.3 [9]";
        let desc =
            "General entity values must use valid reference syntax; this reference is malformed.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_115() {
        let id = "not-wf-sa-115";
        let content = include_str!("../../tests/xmltest/not-wf/sa/115.xml");
        let section = "section 4.5";
        let desc = "The replacement text of this entity is an illegal character reference, which must be rejected when it is parsed in the context of an attribute value.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_116() {
        let id = "not-wf-sa-116";
        let content = include_str!("../../tests/xmltest/not-wf/sa/116.xml");
        let section = "section 4.3.2";
        let desc = "Internal general parsed entities are only well formed if they match the 'content' production.  This is a partial character reference, not a full one.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_117() {
        let id = "not-wf-sa-117";
        let content = include_str!("../../tests/xmltest/not-wf/sa/117.xml");
        let section = "section 4.3.2";
        let desc = "Internal general parsed entities are only well formed if they match the 'content' production.  This is a partial character reference, not a full one.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_118() {
        let id = "not-wf-sa-118";
        let content = include_str!("../../tests/xmltest/not-wf/sa/118.xml");
        let section = "section 4.1 [68]";
        let desc = "Entity reference expansion is not recursive.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_119() {
        let id = "not-wf-sa-119";
        let content = include_str!("../../tests/xmltest/not-wf/sa/119.xml");
        let section = "section 4.3.2";
        let desc = "Internal general parsed entities are only well formed if they match the 'content' production.  This is a partial character reference, not a full one.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_120() {
        let id = "not-wf-sa-120";
        let content = include_str!("../../tests/xmltest/not-wf/sa/120.xml");
        let section = "section 4.5";
        let desc = "Character references are expanded in the replacement text of an internal entity, which is then parsed as usual.  Accordingly, &amp; must be doubly quoted - encoded either as &amp;amp; or as &amp;#38;#38;.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_121() {
        let id = "not-wf-sa-121";
        let content = include_str!("../../tests/xmltest/not-wf/sa/121.xml");
        let section = "section 4.1 [68]";
        let desc = "A name of an ENTITY was started with an invalid character.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_122() {
        let id = "not-wf-sa-122";
        let content = include_str!("../../tests/xmltest/not-wf/sa/122.xml");
        let section = "section 3.2.1 [47]";
        let desc = "Invalid syntax mixed connectors are used.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_123() {
        let id = "not-wf-sa-123";
        let content = include_str!("../../tests/xmltest/not-wf/sa/123.xml");
        let section = "section 3.2.1 [48]";
        let desc = "Invalid syntax mismatched parenthesis.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_124() {
        let id = "not-wf-sa-124";
        let content = include_str!("../../tests/xmltest/not-wf/sa/124.xml");
        let section = "section 3.2.2 [51]";
        let desc = "Invalid format of Mixed-content declaration.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_125() {
        let id = "not-wf-sa-125";
        let content = include_str!("../../tests/xmltest/not-wf/sa/125.xml");
        let section = "section 3.2.2 [51]";
        let desc = "Invalid syntax extra set of parenthesis not necessary.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_126() {
        let id = "not-wf-sa-126";
        let content = include_str!("../../tests/xmltest/not-wf/sa/126.xml");
        let section = "section 3.2.2 [51]";
        let desc = "Invalid syntax Mixed-content must be defined as zero or more.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_127() {
        let id = "not-wf-sa-127";
        let content = include_str!("../../tests/xmltest/not-wf/sa/127.xml");
        let section = "section 3.2.2 [51]";
        let desc = "Invalid syntax Mixed-content must be defined as zero or more.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_128() {
        let id = "not-wf-sa-128";
        let content = include_str!("../../tests/xmltest/not-wf/sa/128.xml");
        let section = "section 2.7 [18]";
        let desc = "Invalid CDATA syntax.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_129() {
        let id = "not-wf-sa-129";
        let content = include_str!("../../tests/xmltest/not-wf/sa/129.xml");
        let section = "section 3.2 [45]";
        let desc = "Invalid syntax for Element Type Declaration.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_130() {
        let id = "not-wf-sa-130";
        let content = include_str!("../../tests/xmltest/not-wf/sa/130.xml");
        let section = "section 3.2 [45]";
        let desc = "Invalid syntax for Element Type Declaration.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_131() {
        let id = "not-wf-sa-131";
        let content = include_str!("../../tests/xmltest/not-wf/sa/131.xml");
        let section = "section 3.2 [45]";
        let desc = "Invalid syntax for Element Type Declaration.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_132() {
        let id = "not-wf-sa-132";
        let content = include_str!("../../tests/xmltest/not-wf/sa/132.xml");
        let section = "section 3.2.1 [50]";
        let desc = "Invalid syntax mixed connectors used.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_133() {
        let id = "not-wf-sa-133";
        let content = include_str!("../../tests/xmltest/not-wf/sa/133.xml");
        let section = "section 3.2.1";
        let desc = "Illegal whitespace before optional character causes syntax error.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_134() {
        let id = "not-wf-sa-134";
        let content = include_str!("../../tests/xmltest/not-wf/sa/134.xml");
        let section = "section 3.2.1";
        let desc = "Illegal whitespace before optional character causes syntax error.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_135() {
        let id = "not-wf-sa-135";
        let content = include_str!("../../tests/xmltest/not-wf/sa/135.xml");
        let section = "section 3.2.1 [47]";
        let desc = "Invalid character used as connector.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_136() {
        let id = "not-wf-sa-136";
        let content = include_str!("../../tests/xmltest/not-wf/sa/136.xml");
        let section = "section 3.2 [45]";
        let desc = "Tag omission is invalid in XML.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_137() {
        let id = "not-wf-sa-137";
        let content = include_str!("../../tests/xmltest/not-wf/sa/137.xml");
        let section = "section 3.2 [45]";
        let desc = "Space is required before a content model.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_138() {
        let id = "not-wf-sa-138";
        let content = include_str!("../../tests/xmltest/not-wf/sa/138.xml");
        let section = "section 3.2.1 [48]";
        let desc = "Invalid syntax for content particle. ";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_139() {
        let id = "not-wf-sa-139";
        let content = include_str!("../../tests/xmltest/not-wf/sa/139.xml");
        let section = "section 3.2.1 [46]";
        let desc = "The element-content model should not be empty.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_140() {
        let id = "not-wf-sa-140";
        let content = include_str!("../../tests/xmltest/not-wf/sa/140.xml");
        let section = "section 2.3 [4]";
        let desc = "Character '&amp;#x309a;' is a CombiningChar, not a Letter, and so may not begin a name.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_141() {
        let id = "not-wf-sa-141";
        let content = include_str!("../../tests/xmltest/not-wf/sa/141.xml");
        let section = "section 2.3 [5]";
        let desc = "Character #x0E5C is not legal in XML names.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_142() {
        let id = "not-wf-sa-142";
        let content = include_str!("../../tests/xmltest/not-wf/sa/142.xml");
        let section = "section 2.2 [2]";
        let desc = "Character #x0000 is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_143() {
        let id = "not-wf-sa-143";
        let content = include_str!("../../tests/xmltest/not-wf/sa/143.xml");
        let section = "section 2.2 [2]";
        let desc = "Character #x001F is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_144() {
        let id = "not-wf-sa-144";
        let content = include_str!("../../tests/xmltest/not-wf/sa/144.xml");
        let section = "section 2.2 [2]";
        let desc = "Character #xFFFF is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_145() {
        let id = "not-wf-sa-145";
        let content = include_str!("../../tests/xmltest/not-wf/sa/145.xml");
        let section = "section 2.2 [2]";
        let desc = "Character #xD800 is not legal anywhere in an XML document.  (If it appeared in a UTF-16 surrogate pair, it'd represent half of a UCS-4 character and so wouldn't really be in the document.)";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_146() {
        let id = "not-wf-sa-146";
        let content = include_str!("../../tests/xmltest/not-wf/sa/146.xml");
        let section = "section 2.2 [2]";
        let desc = "Character references must also refer to legal XML characters; #x00110000 is one more than the largest legal character.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_147() {
        let id = "not-wf-sa-147";
        let content = include_str!("../../tests/xmltest/not-wf/sa/147.xml");
        let section = "section 2.8 [22]";
        let desc = "XML Declaration may not be preceded by whitespace.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_148() {
        let id = "not-wf-sa-148";
        let content = include_str!("../../tests/xmltest/not-wf/sa/148.xml");
        let section = "section 2.8 [22]";
        let desc = "XML Declaration may not be preceded by comments or whitespace.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_149() {
        let id = "not-wf-sa-149";
        let content = include_str!("../../tests/xmltest/not-wf/sa/149.xml");
        let section = "section 2.8 [28]";
        let desc = "XML Declaration may not be within a DTD.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_150() {
        let id = "not-wf-sa-150";
        let content = include_str!("../../tests/xmltest/not-wf/sa/150.xml");
        let section = "section 3.1 [43]";
        let desc = "XML declarations may not be within element content.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_151() {
        let id = "not-wf-sa-151";
        let content = include_str!("../../tests/xmltest/not-wf/sa/151.xml");
        let section = "section 2.8 [27]";
        let desc = "XML declarations may not follow document content.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_152() {
        let id = "not-wf-sa-152";
        let content = include_str!("../../tests/xmltest/not-wf/sa/152.xml");
        let section = "section 2.8 [22]";
        let desc = "XML declarations must include the 'version=...' string.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_153() {
        let id = "not-wf-sa-153";
        let content = include_str!("../../tests/xmltest/not-wf/sa/153.xml");
        let section = "section 4.3.2";
        let desc = "Text declarations may not begin internal parsed entities; they may only appear at the beginning of external parsed (parameter or general) entities.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_154() {
        let id = "not-wf-sa-154";
        let content = include_str!("../../tests/xmltest/not-wf/sa/154.xml");
        let section = "section 2.8 2.6 [23, 17]";
        let desc = "'&lt;?XML ...?&gt;' is neither an XML declaration nor a legal processing instruction target name.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_155() {
        let id = "not-wf-sa-155";
        let content = include_str!("../../tests/xmltest/not-wf/sa/155.xml");
        let section = "section 2.8 2.6 [23, 17]";
        let desc = "'&lt;?xmL ...?&gt;' is neither an XML declaration nor a legal processing instruction target name.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_156() {
        let id = "not-wf-sa-156";
        let content = include_str!("../../tests/xmltest/not-wf/sa/156.xml");
        let section = "section 2.8 2.6 [23, 17]";
        let desc = "'&lt;?xMl ...?&gt;' is neither an XML declaration nor a legal processing instruction target name.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_157() {
        let id = "not-wf-sa-157";
        let content = include_str!("../../tests/xmltest/not-wf/sa/157.xml");
        let section = "section 2.6 [17]";
        let desc = "'&lt;?xmL ...?&gt;' is not a legal processing instruction target name.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_158() {
        let id = "not-wf-sa-158";
        let content = include_str!("../../tests/xmltest/not-wf/sa/158.xml");
        let section = "section 3.3 [52]";
        let desc = "SGML-ism:  '#NOTATION gif' can't have attributes.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_159() {
        let id = "not-wf-sa-159";
        let content = include_str!("../../tests/xmltest/not-wf/sa/159.xml");
        let section = "section 2.3 [9]";
        let desc = "Uses '&amp;' unquoted in an entity declaration, which is illegal syntax for an entity reference.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_160() {
        let id = "not-wf-sa-160";
        let content = include_str!("../../tests/xmltest/not-wf/sa/160.xml");
        let section = "section 2.8";
        let desc =
            "Violates the PEs in Internal Subset WFC by using a PE reference within a declaration.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_161() {
        let id = "not-wf-sa-161";
        let content = include_str!("../../tests/xmltest/not-wf/sa/161.xml");
        let section = "section 2.8";
        let desc =
            "Violates the PEs in Internal Subset WFC by using a PE reference within a declaration.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_162() {
        let id = "not-wf-sa-162";
        let content = include_str!("../../tests/xmltest/not-wf/sa/162.xml");
        let section = "section 2.8";
        let desc =
            "Violates the PEs in Internal Subset WFC by using a PE reference within a declaration.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_163() {
        let id = "not-wf-sa-163";
        let content = include_str!("../../tests/xmltest/not-wf/sa/163.xml");
        let section = "section 4.1 [69]";
        let desc = "Invalid placement of Parameter entity reference.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_164() {
        let id = "not-wf-sa-164";
        let content = include_str!("../../tests/xmltest/not-wf/sa/164.xml");
        let section = "section 4.1 [69]";
        let desc = "Invalid placement of Parameter entity reference.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_165() {
        let id = "not-wf-sa-165";
        let content = include_str!("../../tests/xmltest/not-wf/sa/165.xml");
        let section = "section 4.2 [72]";
        let desc = "Parameter entity declarations must have a space before the '%'.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_166() {
        let id = "not-wf-sa-166";
        let content = include_str!("../../tests/xmltest/not-wf/sa/166.xml");
        let section = "section 2.2 [2]";
        let desc = "Character FFFF is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_167() {
        let id = "not-wf-sa-167";
        let content = include_str!("../../tests/xmltest/not-wf/sa/167.xml");
        let section = "section 2.2 [2]";
        let desc = "Character FFFE is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    // Test uses invalid utf8 in the input - read as bytes and convert unchecked to &str
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_168() {
        let id = "not-wf-sa-168";
        let content = unsafe {
            std::str::from_utf8_unchecked(include_bytes!("../../tests/xmltest/not-wf/sa/168.xml"))
        };
        let section = "section 2.2 [2]";
        let desc = "An unpaired surrogate (D800) is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    // Test uses invalid utf8 in the input - read as bytes and convert unchecked to &str
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_169() {
        let id = "not-wf-sa-169";
        let content = unsafe {
            std::str::from_utf8_unchecked(include_bytes!("../../tests/xmltest/not-wf/sa/169.xml"))
        };
        let section = "section 2.2 [2]";
        let desc = "An unpaired surrogate (DC00) is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    // Test uses invalid utf8 in the input - read as bytes and convert unchecked to &str
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_170() {
        let id = "not-wf-sa-170";
        let content = unsafe {
            std::str::from_utf8_unchecked(include_bytes!("../../tests/xmltest/not-wf/sa/170.xml"))
        };
        let section = "section 2.2 [2]";
        let desc = "Four byte UTF-8 encodings can encode UCS-4 characters which are beyond the range of legal XML characters (and can't be expressed in Unicode surrogate pairs).  This document holds such a character. ";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_171() {
        let id = "not-wf-sa-171";
        let content = include_str!("../../tests/xmltest/not-wf/sa/171.xml");
        let section = "section 2.2 [2]";
        let desc = "Character FFFF is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_172() {
        let id = "not-wf-sa-172";
        let content = include_str!("../../tests/xmltest/not-wf/sa/172.xml");
        let section = "section 2.2 [2]";
        let desc = "Character FFFF is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_173() {
        let id = "not-wf-sa-173";
        let content = include_str!("../../tests/xmltest/not-wf/sa/173.xml");
        let section = "section 2.2 [2]";
        let desc = "Character FFFF is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_174() {
        let id = "not-wf-sa-174";
        let content = include_str!("../../tests/xmltest/not-wf/sa/174.xml");
        let section = "section 2.2 [2]";
        let desc = "Character FFFF is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_175() {
        let id = "not-wf-sa-175";
        let content = include_str!("../../tests/xmltest/not-wf/sa/175.xml");
        let section = "section 2.2 [2]";
        let desc = "Character FFFF is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_176() {
        let id = "not-wf-sa-176";
        let content = include_str!("../../tests/xmltest/not-wf/sa/176.xml");
        let section = "section 3 [39]";
        let desc = "Start tags must have matching end tags.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_177() {
        let id = "not-wf-sa-177";
        let content = include_str!("../../tests/xmltest/not-wf/sa/177.xml");
        let section = "section 2.2 [2]";
        let desc = "Character FFFF is not legal anywhere in an XML document.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_178() {
        let id = "not-wf-sa-178";
        let content = include_str!("../../tests/xmltest/not-wf/sa/178.xml");
        let section = "section 3.1 [41]";
        let desc = "Invalid syntax matching double quote is missing.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_179() {
        let id = "not-wf-sa-179";
        let content = include_str!("../../tests/xmltest/not-wf/sa/179.xml");
        let section = "section 4.1 [66]";
        let desc = "Invalid syntax matching double quote is missing.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_180() {
        let id = "not-wf-sa-180";
        let content = include_str!("../../tests/xmltest/not-wf/sa/180.xml");
        let section = "section 4.1";
        let desc = "The Entity Declared WFC requires entities to be declared before they are used in an attribute list declaration.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_181() {
        let id = "not-wf-sa-181";
        let content = include_str!("../../tests/xmltest/not-wf/sa/181.xml");
        let section = "section 4.3.2";
        let desc = "Internal parsed entities must match the content production to be well formed.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_182() {
        let id = "not-wf-sa-182";
        let content = include_str!("../../tests/xmltest/not-wf/sa/182.xml");
        let section = "section 4.3.2";
        let desc = "Internal parsed entities must match the content production to be well formed.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_183() {
        let id = "not-wf-sa-183";
        let content = include_str!("../../tests/xmltest/not-wf/sa/183.xml");
        let section = "section 3.2.2 [51]";
        let desc = "Mixed content declarations may not include content particles.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_184() {
        let id = "not-wf-sa-184";
        let content = include_str!("../../tests/xmltest/not-wf/sa/184.xml");
        let section = "section 3.2.2 [51]";
        let desc = "In mixed content models, element names must not be parenthesized.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_185() {
        let id = "not-wf-sa-185";
        let content = include_str!("../../tests/xmltest/not-wf/sa/185.xml");
        let section = "section 4.1";
        let desc = "Tests the Entity Declared WFC.  Note:  a nonvalidating parser is permitted not to report this WFC violation, since it would need to read an external parameter entity to distinguish it from a violation of the Standalone Declaration VC.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_186() {
        let id = "not-wf-sa-186";
        let content = include_str!("../../tests/xmltest/not-wf/sa/186.xml");
        let section = "section 3.1 [44]";
        let desc = "Whitespace is required between attribute/value pairs.";
        conformance_not_wf(id, section, desc, content);
    }

    // <!-- Start:  not-wf/not-sa -->
    // This test requires the parser to resolve external DTDs, something we don't yet do.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_not_sa_001() {
        let id = "not-wf-not-sa-001";
        let content = include_str!("../../tests/xmltest/not-wf/not-sa/001.xml");
        let section = "section 3.4 [62]";
        let desc =
            "Conditional sections must be properly terminated (']&gt;' used instead of ']]&gt;').";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve external DTDs, something we don't yet do.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_not_sa_002() {
        let id = "not-wf-not-sa-002";
        let content = include_str!("../../tests/xmltest/not-wf/not-sa/002.xml");
        let section = "section 2.6 [17]";
        let desc =
            "Processing instruction target names may not be 'XML' in any combination of cases.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve external DTDs, something we don't yet do.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_not_sa_003() {
        let id = "not-wf-not-sa-003";
        let content = include_str!("../../tests/xmltest/not-wf/not-sa/003.xml");
        let section = "section 3.4 [62]";
        let desc = "Conditional sections must be properly terminated (']]&gt;' omitted).";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve external DTDs, something we don't yet do.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_not_sa_004() {
        let id = "not-wf-not-sa-004";
        let content = include_str!("../../tests/xmltest/not-wf/not-sa/004.xml");
        let section = "section 3.4 [62]";
        let desc = "Conditional sections must be properly terminated (']]&gt;' omitted).";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve external DTDs, something we don't yet do.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_not_sa_005() {
        let id = "not-wf-not-sa-005";
        let content = include_str!("../../tests/xmltest/not-wf/not-sa/005.xml");
        let section = "section 4.1";
        let desc = "Tests the Entity Declared VC by referring to an undefined parameter entity within an external entity.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve external DTDs, something we don't yet do.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_not_sa_006() {
        let id = "not-wf-not-sa-006";
        let content = include_str!("../../tests/xmltest/not-wf/not-sa/006.xml");
        let section = "section 3.4 [62]";
        let desc = "Conditional sections need a '[' after the INCLUDE or IGNORE.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve external DTDs, something we don't yet do.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_not_sa_007() {
        let id = "not-wf-not-sa-007";
        let content = include_str!("../../tests/xmltest/not-wf/not-sa/007.xml");
        let section = "section 4.3.2 [79]";
        let desc = "A &lt;!DOCTYPE ...&gt; declaration may not begin any external entity; it's only found once, in the document entity.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve external DTDs, something we don't yet do.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_not_sa_008() {
        let id = "not-wf-not-sa-008";
        let content = include_str!("../../tests/xmltest/not-wf/not-sa/008.xml");
        let section = "section 4.1 [69]";
        let desc = "In DTDs, the '%' character must be part of a parameter entity reference.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve external DTDs, something we don't yet do.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_not_sa_009() {
        let id = "not-wf-not-sa-009";
        let content = include_str!("../../tests/xmltest/not-wf/not-sa/009.xml");
        let section = "section 2.8";
        let desc = "This test violates WFC:PE Between Declarations in Production 28a.  The last character of a markup declaration is not contained in the same parameter-entity text replacement.";
        conformance_not_wf(id, section, desc, content);
    }

    // <!-- Start:  not-wf/ext-sa -->
    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_ext_sa_001() {
        let id = "not-wf-ext-sa-001";
        let content = include_str!("../../tests/xmltest/not-wf/ext-sa/001.xml");
        let section = "section 4.1";
        let desc =
            "Tests the No Recursion WFC by having an external general entity be self-recursive.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_ext_sa_002() {
        let id = "not-wf-ext-sa-002";
        let content = include_str!("../../tests/xmltest/not-wf/ext-sa/002.xml");
        let section = "section 4.3.1 4.3.2 [77, 78]";
        let desc = "External entities have 'text declarations', which do not permit the 'standalone=...' attribute that's allowed in XML declarations.";
        conformance_not_wf(id, section, desc, content);
    }

    // This test requires the parser to resolve, and possibly trigger re-lexing
    // at the site of the entity, in DOCTYPE definitions, something we don't yet
    // do.  See "Known Parsing Gaps" in xml1_0.pest.
    #[ignore]
    #[test]
    #[should_panic]
    fn conformance_not_wf_ext_sa_003() {
        let id = "not-wf-ext-sa-003";
        let content = include_str!("../../tests/xmltest/not-wf/ext-sa/003.xml");
        let section = "section 2.6 [17]";
        let desc = "Only one text declaration is permitted; a second one looks like an illegal processing instruction (target names of 'xml' in any case are not allowed).";
        conformance_not_wf(id, section, desc, content);
    }
}
