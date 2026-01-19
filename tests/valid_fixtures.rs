use biblatex::{Bibliography, ChunksExt, EntryType, PermissiveType, Person};
use std::fs;

#[test]
fn test_gral_bib() {
    let contents = fs::read_to_string("tests/fixtures/valid/gral.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();
    assert_eq!(bibliography.len(), 83);
}

#[test]
fn comprehensive() {
    let contents = fs::read_to_string("comprehensive_test.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();
    // back and forth check
    let serialized = bibliography.to_biblatex_string();
    let bibliography2 = Bibliography::parse(&serialized).unwrap();
    println!("bib1 len = {}", bibliography.len());
    println!("bib2 len = {}", bibliography2.len());

    assert_eq!(bibliography.len(), bibliography2.len());

    // check keys
    for key in bibliography.keys() {
        println!("Checking key: {}", key);
        assert!(bibliography2.get(key).is_some());
    }
}

#[test]
fn test_self_referential() {
    let contents =
        fs::read_to_string("tests/fixtures/valid/self_referential.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();
    assert_eq!(bibliography.len(), 1);
    let entry = bibliography.get("Hartman2022").unwrap();
    assert_eq!(entry.entry_type, EntryType::InCollection);
}

#[test]
fn test_circular_crossref() {
    let contents =
        fs::read_to_string("tests/fixtures/valid/circular_crossref.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();
    assert_eq!(bibliography.len(), 3);

    // All entries should parse successfully without stack overflow
    let entry_a = bibliography.get("entry_a").unwrap();
    let entry_b = bibliography.get("entry_b").unwrap();
    let entry_c = bibliography.get("entry_c").unwrap();

    assert_eq!(entry_a.title().unwrap().format_sentence(), "Entry a");
    assert_eq!(entry_b.title().unwrap().format_sentence(), "Entry b");
    assert_eq!(entry_c.title().unwrap().format_sentence(), "Entry c");
}

#[test]
fn test_ds_bib() {
    let contents = fs::read_to_string("tests/fixtures/valid/ds.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();
    println!("{:#?}", bibliography);
}

#[test]
fn test_libra_bib() {
    let contents = fs::read_to_string("tests/fixtures/valid/libra.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();

    println!("{}", bibliography.to_biblatex_string());

    for x in bibliography.iter() {
        let authors = x.author().unwrap_or_default();
        for a in &authors {
            print!("{}, ", a);
        }
        println!("\"{}\".", x.title().unwrap().format_sentence());
    }

    // Import an entry/field with escaped colons
    let e = bibliography.get("dierksmeierJustHODLMoral2018").unwrap();
    assert_eq!(e.doi().unwrap(), "10.1007/s41463-018-0036-z");
    assert_eq!(
        e.file().unwrap(),
        "C:\\Users\\mhaug\\Zotero\\storage\\DTPR7TES\\Dierksmeier - 2018 - Just HODL On the Moral Claims of Bitcoin and Ripp.pdf"
    );

    // Import an entry/field with unescaped colons
    let e = bibliography.get("LibraAssociationIndependent").unwrap();
    assert_eq!(e.url().unwrap(), "https://libra.org/association/");

    // Test export of entry (not escaping colons)
    let e = bibliography.get("finextraFedGovernorChallenges2019").unwrap();
    assert_eq!(
        e.to_biblatex_string(),
        "@online{finextraFedGovernorChallenges2019,\nauthor = {FinExtra},\ndate = {2019-12-18},\nfile = {C:\\\\Users\\\\mhaug\\\\Zotero\\\\storage\\\\VY9LAKFE\\\\fed-governor-challenges-facebooks-libra-project.html},\ntitle = {Fed {Governor} Challenges {Facebook}'s {Libra} Project},\nurl = {https://www.finextra.com/newsarticle/34986/fed-governor-challenges-facebooks-libra-project},\nurldate = {2020-08-22},\n}"
    );

    // Test URLs with math and backslashes
    let e = bibliography.get("weirdUrl2023").unwrap();
    assert_eq!(e.url().unwrap(), r#"example.com?A=$B\%\{}"#);
    assert_eq!(e.doi().unwrap(), r#"example.com?A=$B\%\{}"#);
}

#[test]
fn test_rass_bib() {
    let contents = fs::read_to_string("tests/fixtures/valid/rass.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();

    println!("{}", bibliography.to_biblatex_string());

    for x in bibliography {
        let authors = x.author().unwrap_or_default();
        for a in authors {
            print!("{}, ", a);
        }
        println!("\"{}\".", x.title().unwrap().format_sentence());
    }
}

#[test]
fn test_polaritons_bib() {
    let contents = fs::read_to_string("tests/fixtures/valid/polaritons.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();

    println!("{}", bibliography.to_biblatex_string());

    for x in bibliography.iter() {
        let authors = x.author().unwrap_or_default();
        for a in authors {
            print!("{}, ", a);
        }
        println!("\"{}\".", x.title().unwrap().format_sentence());
    }
}

#[test]
fn test_comments_bib() {
    let contents = fs::read_to_string("tests/fixtures/valid/comments.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();

    assert_eq!(
        bibliography.keys().collect::<Vec<_>>(),
        &[
            "mcelreath2007mathematical",
            "fischer2022equivalence",
            "roes2003belief",
            "wong2016null",
        ]
    );

    assert_eq!(
        bibliography
            .get("wong2016null")
            .unwrap()
            .title()
            .unwrap()
            .format_verbatim(),
        "Null hypothesis testing (I)-5% significance level"
    );
}

#[test]
fn test_extended_name_format_bib() {
    let contents =
        fs::read_to_string("tests/fixtures/valid/extended_name_format.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();

    println!("{}", bibliography.to_biblatex_string());

    for x in bibliography.iter() {
        let authors = x.author().unwrap_or_default();
        for a in authors {
            print!("{}, ", a);
        }
        println!("\"{}\".", x.title().unwrap().format_sentence());
    }
}

#[test]
fn test_cross_bib() {
    let contents = fs::read_to_string("tests/fixtures/valid/cross.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();

    let e = bibliography.get("macmillan").unwrap();
    assert_eq!(e.publisher().unwrap()[0].format_verbatim(), "Macmillan");
    assert_eq!(e.location().unwrap().format_verbatim(), "New York and London");

    let book = bibliography.get("recursive").unwrap();
    assert_eq!(book.publisher().unwrap()[0].format_verbatim(), "Macmillan");
    assert_eq!(book.location().unwrap().format_verbatim(), "New York and London");
    assert_eq!(
        book.title().unwrap().format_verbatim(),
        "Recursive shennenigans and other important stuff"
    );

    assert_eq!(
        bibliography.get("arrgh").unwrap().parents().unwrap(),
        vec!["polecon".to_string()]
    );
    let arrgh = bibliography.get("arrgh").unwrap();
    assert_eq!(arrgh.entry_type, EntryType::Article);
    assert_eq!(arrgh.volume().unwrap(), PermissiveType::Typed(115));
    assert_eq!(arrgh.editors().unwrap()[0].0[0].name, "Uhlig");
    assert_eq!(arrgh.number().unwrap().format_verbatim(), "6");
    assert_eq!(
        arrgh.journal().unwrap().format_verbatim(),
        "Journal of Political Economy"
    );
    assert_eq!(
        arrgh.title().unwrap().format_verbatim(),
        "An‐arrgh‐chy: The Law and Economics of Pirate Organization"
    );
}

#[test]
fn test_cross_alias() {
    let contents = fs::read_to_string("tests/fixtures/valid/cross.bib").unwrap();
    let mut bibliography = Bibliography::parse(&contents).unwrap();

    assert_eq!(bibliography.get("issue201"), bibliography.get("github"));
    bibliography.alias("issue201", "crap");
    assert_eq!(bibliography.get("crap"), bibliography.get("unstable"));
    bibliography.remove("crap").unwrap();

    let entry = bibliography.get("cannonfodder").unwrap();
    assert_eq!(entry.key, "cannonfodder");
    assert_eq!(entry.entry_type, EntryType::Misc);
}

#[test]
fn test_cross_bibtex_conversion() {
    let contents = fs::read_to_string("tests/fixtures/valid/cross.bib").unwrap();
    let mut bibliography = Bibliography::parse(&contents).unwrap();

    let biblatex = bibliography.get_mut("haug2019").unwrap().to_biblatex_string();
    assert!(biblatex.contains("institution = {Technische Universität Berlin},"));

    let bibtex = bibliography.get_mut("haug2019").unwrap().to_bibtex_string().unwrap();
    assert!(bibtex.contains("school = {Technische Universität Berlin},"));
    assert!(bibtex.contains("year = {2019},"));
    assert!(bibtex.contains("month = {10},"));
    assert!(!bibtex.contains("institution"));
    assert!(!bibtex.contains("date"));
}

#[test]
fn test_cross_verify() {
    let contents = fs::read_to_string("tests/fixtures/valid/cross.bib").unwrap();
    let mut bibliography = Bibliography::parse(&contents).unwrap();

    assert!(bibliography.get_mut("haug2019").unwrap().verify().is_ok());
    assert!(bibliography.get_mut("cannonfodder").unwrap().verify().is_ok());

    let ill = bibliography.get("ill-defined").unwrap();
    let report = ill.verify();
    assert_eq!(report.missing.len(), 3);
    assert_eq!(report.superfluous.len(), 3);
    assert_eq!(report.malformed.len(), 1);
    assert!(report.missing.contains(&"title"));
    assert!(report.missing.contains(&"year"));
    assert!(report.missing.contains(&"editor"));
    assert!(report.superfluous.contains(&"maintitle"));
    assert!(report.superfluous.contains(&"author"));
    assert!(report.superfluous.contains(&"chapter"));
    assert_eq!(report.malformed[0].0.as_str(), "gender");
}

#[test]
fn test_gral_verify() {
    let contents = fs::read_to_string("tests/fixtures/valid/gral.bib").unwrap();
    let mut bibliography = Bibliography::parse(&contents).unwrap();
    assert!(bibliography.get_mut("lin_sida:_2007").unwrap().verify().is_ok());
}

#[test]
fn test_editortypes_bib() {
    let contents = fs::read_to_string("tests/fixtures/valid/editortypes.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();

    assert_eq!(
        bibliography.keys().collect::<Vec<_>>(),
        &["acerolaThisDifferenceGaussians2022", "mozart_KV183_1773", "Smith2018"]
    );

    let video = bibliography.get("acerolaThisDifferenceGaussians2022").unwrap();
    assert_eq!(
        video.editors(),
        Ok(vec![(
            vec![Person {
                name: "Acerola".into(),
                given_name: "".into(),
                prefix: "".into(),
                suffix: "".into()
            }],
            biblatex::EditorType::Director
        )])
    );

    let music = bibliography.get("mozart_KV183_1773").unwrap();
    assert_eq!(
        music.editors(),
        Ok(vec![(
            vec![Person {
                name: "Mozart".into(),
                given_name: "Wolfgang Amadeus".into(),
                prefix: "".into(),
                suffix: "".into()
            }],
            biblatex::EditorType::Unknown("pianist".into()),
        )])
    );

    let audio = bibliography.get("Smith2018").unwrap();
    assert_eq!(
        audio.editors(),
        Ok(vec![
            (
                vec![Person {
                    name: "Smith".into(),
                    given_name: "Stacey Vanek".into(),
                    prefix: "".into(),
                    suffix: "".into()
                }],
                biblatex::EditorType::Unknown("host".into()),
            ),
            (
                vec![Person {
                    name: "Plotkin".into(),
                    given_name: "Stanley".into(),
                    prefix: "".into(),
                    suffix: "".into()
                }],
                biblatex::EditorType::Unknown("participant".into()),
            )
        ])
    );
}

#[test]
fn test_case_bib() {
    let contents = fs::read_to_string("tests/fixtures/valid/case.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();

    let entry = bibliography.get("biblatex2023").unwrap();
    let author = entry.author();

    match author {
        Ok(a) => assert_eq!(a[0].name, "Kime"),
        Err(biblatex::RetrievalError::Missing(_)) => {
            panic!("Tags should be case insensitive.");
        }
        _ => panic!(),
    }
}

#[test]
fn test_non_standard_types() {
    let contents =
        fs::read_to_string("tests/fixtures/valid/non_standard_types.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();
    assert_eq!(bibliography.len(), 15);

    // Test artwork type
    let artwork = bibliography.get("picasso1937").unwrap();
    assert_eq!(artwork.entry_type, EntryType::Artwork);
    assert_eq!(artwork.title().unwrap().format_sentence(), "Guernica");

    // Test audio type
    let audio = bibliography.get("beatles1969").unwrap();
    assert_eq!(audio.entry_type, EntryType::Audio);

    // Test video type
    let video = bibliography.get("nolan2010").unwrap();
    assert_eq!(video.entry_type, EntryType::Video);

    // Test movie type
    let movie = bibliography.get("kubrick1968").unwrap();
    assert_eq!(movie.entry_type, EntryType::Movie);

    // Test music type
    let music = bibliography.get("mozart1791").unwrap();
    assert_eq!(music.entry_type, EntryType::Music);

    // Test review type (should behave like Article)
    let review = bibliography.get("ebert1999").unwrap();
    assert_eq!(review.entry_type, EntryType::Review);
    assert!(review.journal_title().is_ok());

    // Test performance type
    let performance = bibliography.get("shakespeare1599").unwrap();
    assert_eq!(performance.entry_type, EntryType::Performance);

    // Test standard type
    let standard = bibliography.get("iso8601").unwrap();
    assert_eq!(standard.entry_type, EntryType::Standard);

    // Test image type
    let image = bibliography.get("adams1941").unwrap();
    assert_eq!(image.entry_type, EntryType::Image);

    // Test letter type
    let letter = bibliography.get("einstein1939").unwrap();
    assert_eq!(letter.entry_type, EntryType::Letter);

    // Test commentary type
    let commentary = bibliography.get("blackstone1765").unwrap();
    assert_eq!(commentary.entry_type, EntryType::Commentary);

    // Test jurisdiction type
    let jurisdiction = bibliography.get("brownboard1954").unwrap();
    assert_eq!(jurisdiction.entry_type, EntryType::Jurisdiction);

    // Test legislation type
    let legislation = bibliography.get("civilrights1964").unwrap();
    assert_eq!(legislation.entry_type, EntryType::Legislation);

    // Test legal type
    let legal = bibliography.get("magna1215").unwrap();
    assert_eq!(legal.entry_type, EntryType::Legal);

    // Test bibnote type
    let bibnote = bibliography.get("note1").unwrap();
    assert_eq!(bibnote.entry_type, EntryType::BibNote);

    // Test BibTeX conversion (all non-standard types should convert to Misc except Review -> Article)
    assert_eq!(artwork.entry_type.to_bibtex(), EntryType::Misc);
    assert_eq!(review.entry_type.to_bibtex(), EntryType::Article);
}

#[test]
fn test_at_sign_fields() {
    let contents = fs::read_to_string("tests/fixtures/valid/at_sign_fields.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();
    assert_eq!(bibliography.len(), 4);

    // First entry should have title and year, but not @location
    let entry1 = bibliography.get("foo").unwrap();
    assert_eq!(entry1.title().unwrap().format_sentence(), "Bar");
    // Check that the year field exists
    assert!(entry1.get("year").is_some());
    // The @location field should be ignored
    assert!(entry1.get("location").is_none());

    // Second entry should have title, author, and year, but not the @commented fields
    let entry2 = bibliography.get("test_at_comment").unwrap();
    assert_eq!(entry2.title().unwrap().format_sentence(), "Test article");
    assert_eq!(entry2.author().unwrap()[0].name, "Doe");
    // Check that the year field exists
    assert!(entry2.get("year").is_some());
    // The @commented_field and @another_comment should be ignored
    assert!(entry2.get("commented_field").is_none());
    assert!(entry2.get("another_comment").is_none());

    // Third entry tests complex nested braces in @-commented field
    let entry3 = bibliography.get("test_at_with_braces").unwrap();
    assert_eq!(entry3.title().unwrap().format_sentence(), "Book with nested braces");
    assert_eq!(entry3.author().unwrap()[0].name, "Smith");
    assert!(entry3.publisher().is_ok());
    // The @complex_field should be ignored even with nested braces
    assert!(entry3.get("complex_field").is_none());

    // Fourth entry tests multiple @ comments interspersed with real fields
    let entry4 = bibliography.get("test_at_multiple").unwrap();
    assert_eq!(entry4.title().unwrap().format_sentence(), "Conference paper");
    assert_eq!(entry4.author().unwrap()[0].name, "Jones");
    assert!(entry4.get("year").is_some());
    // All three @comment fields should be ignored
    assert!(entry4.get("comment1").is_none());
    assert!(entry4.get("comment2").is_none());
    assert!(entry4.get("comment3").is_none());
}

#[test]
fn test_urldate_with_time() {
    let contents =
        fs::read_to_string("tests/fixtures/valid/urldate_with_time.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();
    assert_eq!(bibliography.len(), 3);

    // Test first entry with full datetime
    let entry1 = bibliography.get("online-source-with-time").unwrap();
    assert_eq!(entry1.title().unwrap().format_sentence(), "Knuth: computers and typesetting");
    assert_eq!(entry1.author().unwrap()[0].name, "Knuth");
    
    let urldate1 = entry1.url_date().unwrap();
    if let PermissiveType::Typed(date) = urldate1 {
        if let biblatex::DateValue::At(datetime) = date.value {
            assert_eq!(datetime.year, 2024);
            assert_eq!(datetime.month, Some(4)); // May is month 4 (0-indexed)
            assert_eq!(datetime.day, Some(8)); // 9th day is 8 (0-indexed)
            assert!(datetime.time.is_some());
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

    // Test second entry with seconds
    let entry2 = bibliography.get("online-source-with-seconds").unwrap();
    let urldate2 = entry2.url_date().unwrap();
    if let PermissiveType::Typed(date) = urldate2 {
        if let biblatex::DateValue::At(datetime) = date.value {
            assert_eq!(datetime.year, 2024);
            assert_eq!(datetime.month, Some(0)); // January is month 0
            assert_eq!(datetime.day, Some(14)); // 15th day is 14 (0-indexed)
            assert!(datetime.time.is_some());
            let time = datetime.time.unwrap();
            assert_eq!(time.hour, 9);
            assert_eq!(time.minute, 45);
            assert_eq!(time.second, 30);
        } else {
            panic!("Expected DateValue::At");
        }
    } else {
        panic!("Expected PermissiveType::Typed");
    }

    // Test third entry without time (backward compatibility)
    let entry3 = bibliography.get("online-source-date-only").unwrap();
    let urldate3 = entry3.url_date().unwrap();
    if let PermissiveType::Typed(date) = urldate3 {
        if let biblatex::DateValue::At(datetime) = date.value {
            assert_eq!(datetime.year, 2024);
            assert_eq!(datetime.month, Some(2)); // March is month 2
            assert_eq!(datetime.day, Some(19)); // 20th day is 19 (0-indexed)
            assert!(datetime.time.is_none()); // No time should be present
        } else {
            panic!("Expected DateValue::At");
        }
    } else {
        panic!("Expected PermissiveType::Typed");
    }
}
