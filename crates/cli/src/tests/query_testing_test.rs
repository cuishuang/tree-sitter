use std::fs;

use tempfile::NamedTempFile;
use tree_sitter::Parser;

use super::helpers::fixtures::get_test_fixture_language;
use crate::query_testing::{CaptureInfo, Utf8Point, assert_expected_captures};

#[test]
fn test_query_assertion_after_multiline_capture_does_not_match() {
    let language = get_test_fixture_language("readme_grammar");
    let source_file = NamedTempFile::with_suffix(".txt").unwrap();
    fs::write(source_file.path(), "a\nb\nc\n# ^ foo\n").unwrap();

    let captures = [CaptureInfo {
        name: "foo".to_string(),
        start: Utf8Point::new(0, 0),
        end: Utf8Point::new(1, 10),
    }];

    let result =
        assert_expected_captures(&captures, source_file.path(), &mut Parser::new(), &language);

    assert!(result.is_err());
}
