use biblatex::{Bibliography, ChunksExt, DateValue, PermissiveType};

#[test]
fn test_urldate_with_time_example() {
    // Parse a bibliography with URL access dates including time information
    let src = r#"
@online{online-source-with-time,
    author    = "Donald Knuth",
    title     = "Knuth: Computers and Typesetting",
    url       = "http://www-cs-faculty.stanford.edu/~uno/abcde.html",
    urldate   = {2024-05-09T14:30:00},
}
    "#;

    let bib = Bibliography::parse(src).unwrap();
    let entry = bib.get("online-source-with-time").unwrap();

    assert_eq!(
        entry.title().unwrap().format_sentence(),
        "Knuth: computers and typesetting"
    );
    assert_eq!(
        entry.url().unwrap(),
        "http://www-cs-faculty.stanford.edu/~uno/abcde.html"
    );

    // Access the urldate field with time information
    if let Ok(PermissiveType::Typed(date)) = entry.url_date() {
        if let DateValue::At(datetime) = date.value {
            assert_eq!(datetime.year, 2024);
            assert_eq!(datetime.month, Some(4)); // May is month 4 (0-indexed)
            assert_eq!(datetime.day, Some(8)); // 9th is day 8 (0-indexed)

            assert!(datetime.time.is_some(), "Should have time information");
            let time = datetime.time.unwrap();
            assert_eq!(time.hour, 14);
            assert_eq!(time.minute, 30);
            assert_eq!(time.second, 0);
        } else {
            panic!("Expected DateValue::At");
        }
    } else {
        panic!("Expected PermissiveType::Typed");
    }
}
