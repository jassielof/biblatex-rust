use biblatex::{Bibliography, ChunksExt, EntryType, PermissiveType, Person};
use std::fs;

#[test]
fn test_gral_bib() {
    let contents = fs::read_to_string("tests/fixtures/valid/gral.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();
    assert_eq!(bibliography.len(), 83);
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
