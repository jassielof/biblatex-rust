/// Tests for issue #94: Support for "and others" in name lists
use biblatex::{Bibliography, NameListEntry};

#[test]
fn test_and_others_basic() {
    let src = r#"@article{test,
    author = {John Doe and Jane Smith and others},
}"#;
    
    let bibliography = Bibliography::parse(src).unwrap();
    let entry = bibliography.get("test").unwrap();
    
    let authors = entry.author_list().unwrap();
    assert_eq!(authors.len(), 3);
    
    // First two should be persons
    assert!(matches!(authors[0], NameListEntry::Person(_)));
    assert!(matches!(authors[1], NameListEntry::Person(_)));
    
    // Third should be "and others"
    assert!(matches!(authors[2], NameListEntry::AndOthers));
    assert!(authors[2].is_and_others());
    
    // Check person details
    if let NameListEntry::Person(p) = &authors[0] {
        assert_eq!(p.name, "Doe");
        assert_eq!(p.given_name, "John");
    } else {
        panic!("Expected Person");
    }
    
    if let NameListEntry::Person(p) = &authors[1] {
        assert_eq!(p.name, "Smith");
        assert_eq!(p.given_name, "Jane");
    } else {
        panic!("Expected Person");
    }
}

#[test]
fn test_and_others_editors() {
    let src = r#"@book{test,
    editor = {Alice Brown and Bob White and others},
}"#;
    
    let bibliography = Bibliography::parse(src).unwrap();
    let entry = bibliography.get("test").unwrap();
    
    let editors = entry.editor_list().unwrap();
    assert_eq!(editors.len(), 3);
    
    assert!(!editors[0].is_and_others());
    assert!(!editors[1].is_and_others());
    assert!(editors[2].is_and_others());
}

#[test]
fn test_and_others_multiple_editor_types() {
    let src = r#"@book{test,
    editor = {Alice Brown and others},
    editora = {Bob White and Charlie Green and others},
    editoratype = {collaborator},
}"#;
    
    let bibliography = Bibliography::parse(src).unwrap();
    let entry = bibliography.get("test").unwrap();
    
    let editors = entry.editor_lists().unwrap();
    assert_eq!(editors.len(), 2);
    
    // First editor field
    assert_eq!(editors[0].0.len(), 2);
    assert!(editors[0].0[1].is_and_others());
    
    // Second editor field (editora)
    assert_eq!(editors[1].0.len(), 3);
    assert!(editors[1].0[2].is_and_others());
}

#[test]
fn test_and_others_case_insensitive() {
    // Test that "others", "Others", "OTHERS" all work
    let test_cases = vec!["others", "Others", "OTHERS"];
    
    for variant in test_cases {
        let src = format!(
            r#"@article{{test,
    author = {{John Doe and {}}},
}}"#,
            variant
        );
        
        let bibliography = Bibliography::parse(&src).unwrap();
        let entry = bibliography.get("test").unwrap();
        
        let authors = entry.author_list().unwrap();
        assert_eq!(authors.len(), 2, "Failed for variant: {}", variant);
        assert!(authors[1].is_and_others(), "Failed for variant: {}", variant);
    }
}

#[test]
fn test_and_others_only() {
    // Edge case: just "and others" with no names
    let src = r#"@article{test,
    author = {others},
}"#;
    
    let bibliography = Bibliography::parse(src).unwrap();
    let entry = bibliography.get("test").unwrap();
    
    let authors = entry.author_list().unwrap();
    assert_eq!(authors.len(), 1);
    assert!(authors[0].is_and_others());
}

#[test]
fn test_and_others_backward_compatibility() {
    // Ensure old author() method still works and treats "others" as a name
    let src = r#"@article{test,
    author = {John Doe and Jane Smith and others},
}"#;
    
    let bibliography = Bibliography::parse(src).unwrap();
    let entry = bibliography.get("test").unwrap();
    
    // Old method should still work - it treats "others" as a regular name
    let authors_old = entry.author().unwrap();
    assert_eq!(authors_old.len(), 3);
    
    // The third entry will be parsed as a person with "others" as the name
    // since the old method doesn't have special handling for "and others"
    assert_eq!(authors_old[2].name, "others");
    assert_eq!(authors_old[2].given_name, "");
}

#[test]
fn test_and_others_roundtrip() {
    let src = r#"@article{test,
    author = {John Doe and Jane Smith and others},
}"#;
    
    let bibliography = Bibliography::parse(src).unwrap();
    let serialized = bibliography.to_biblatex_string();
    
    // Should be able to parse it again
    let bibliography2 = Bibliography::parse(&serialized).unwrap();
    let entry2 = bibliography2.get("test").unwrap();
    
    let authors2 = entry2.author_list().unwrap();
    assert_eq!(authors2.len(), 3);
    assert!(authors2[2].is_and_others());
}

#[test]
fn test_and_others_as_person_methods() {
    let src = r#"@article{test,
    author = {John Doe and others},
}"#;
    
    let bibliography = Bibliography::parse(src).unwrap();
    let entry = bibliography.get("test").unwrap();
    
    let authors = entry.author_list().unwrap();
    
    // Test as_person()
    assert!(authors[0].as_person().is_some());
    assert!(authors[1].as_person().is_none());
    
    // Test into_person()
    let author0 = authors[0].clone().into_person();
    assert!(author0.is_some());
    
    let author1 = authors[1].clone().into_person();
    assert!(author1.is_none());
}

#[test]
fn test_no_and_others() {
    // Test that normal name lists without "and others" still work
    let src = r#"@article{test,
    author = {John Doe and Jane Smith and Bob Jones},
}"#;
    
    let bibliography = Bibliography::parse(src).unwrap();
    let entry = bibliography.get("test").unwrap();
    
    let authors = entry.author_list().unwrap();
    assert_eq!(authors.len(), 3);
    
    // None should be "and others"
    for author in &authors {
        assert!(!author.is_and_others());
        assert!(author.as_person().is_some());
    }
}

#[test]
fn test_and_others_with_complex_names() {
    let src = r#"@article{test,
    author = {de la Fontaine, Jean and van Gogh, Vincent and others},
}"#;
    
    let bibliography = Bibliography::parse(src).unwrap();
    let entry = bibliography.get("test").unwrap();
    
    let authors = entry.author_list().unwrap();
    assert_eq!(authors.len(), 3);
    
    if let NameListEntry::Person(p) = &authors[0] {
        assert_eq!(p.name, "Fontaine");
        assert_eq!(p.prefix, "de la");
        assert_eq!(p.given_name, "Jean");
    } else {
        panic!("Expected Person");
    }
    
    if let NameListEntry::Person(p) = &authors[1] {
        assert_eq!(p.name, "Gogh");
        assert_eq!(p.prefix, "van");
        assert_eq!(p.given_name, "Vincent");
    } else {
        panic!("Expected Person");
    }
    
    assert!(authors[2].is_and_others());
}
