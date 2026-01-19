use biblatex::Bibliography;

/// Test for issue #76: Math expressions should be preserved exactly
/// Input: $\mathrm{dg}$
/// Expected: $\mathrm{dg}$ (not $\\mathrm\{dg\}$)
#[test]
fn test_math_preservation() {
    let src = r#"@article{key1,
title = {Explicit homotopy limits of $\mathrm{dg}$-categories and twisted complexes},
}"#;
    let bibliography = Bibliography::parse(src).unwrap();
    let serialized = bibliography.to_biblatex_string();

    // The math expression should be preserved exactly
    assert!(
        serialized.contains("$\\mathrm{dg}$"),
        "Math expression should be preserved: {}",
        serialized
    );
}

/// Test for issue #76: Unicode escapes can be converted to Unicode
/// Input: K\"ahler
/// Accepted: Either K\"ahler or Kähler
#[test]
fn test_unicode_escape_preservation() {
    let src = r#"@article{key2,
title = {Hyper-K\"ahler Fourfolds Fibered by Elliptic Products},
}"#;
    let bibliography = Bibliography::parse(src).unwrap();
    let serialized = bibliography.to_biblatex_string();

    // The escaped character can be converted to Unicode
    assert!(
        serialized.contains("ä") || serialized.contains(r#"\"a"#),
        "Unicode or escaped character expected: {}",
        serialized
    );
}

/// Test for issue #76: TeX control space should not be doubled
/// Input: Dept.\ of
/// Expected: Dept. of (not Dept.\\  of)
#[test]
fn test_tex_space_control() {
    let src = r#"@techreport{r:86,
institution = "Dept.\ of Computer Science, Stanford Univ.",
}"#;
    let bibliography = Bibliography::parse(src).unwrap();
    let serialized = bibliography.to_biblatex_string();

    // The space control should not be doubled
    assert!(
        !serialized.contains(r"\\  "),
        "Space control should not be doubled: {}",
        serialized
    );
}

/// Test that simple entries can roundtrip correctly
#[test]
fn test_roundtrip_exact() {
    let src = r#"@article{simple,
title = {A Simple Title},
author = {John Doe},
}"#;
    let bibliography = Bibliography::parse(src).unwrap();
    let serialized = bibliography.to_biblatex_string();

    // Parse again to ensure validity
    let bibliography2 = Bibliography::parse(&serialized).unwrap();
    assert_eq!(bibliography.len(), bibliography2.len());

    let entry1 = bibliography.get("simple").unwrap();
    let entry2 = bibliography2.get("simple").unwrap();

    // Compare the parsed content, not the exact string representation
    let title1 = entry1.title().unwrap();
    let title2 = entry2.title().unwrap();

    assert_eq!(title1.len(), title2.len());
    for (c1, c2) in title1.iter().zip(title2.iter()) {
        assert_eq!(c1.v, c2.v, "Chunk content mismatch");
    }

    assert_eq!(entry1.author(), entry2.author());
}
