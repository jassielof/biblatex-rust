use biblatex::{Bibliography, ParseErrorKind, RetrievalError, TypeErrorKind};

#[test]
fn test_gral_rep_key() {
    let contents =
        std::fs::read_to_string("tests/fixtures/valid/gral_rep_key.bib").unwrap();
    let bibliography = Bibliography::parse(&contents);
    match bibliography {
        Ok(_) => panic!("Should return Err"),
        Err(s) => {
            assert_eq!(s.kind, ParseErrorKind::DuplicateKey("ishihara2012".into()));
        }
    };
}

#[test]
fn test_incorrect_syntax() {
    let contents = std::fs::read_to_string("tests/fixtures/invalid/incorrect_syntax.bib")
        .unwrap()
        .replace("\r\n", "\n");

    let bibliography = Bibliography::parse(&contents);
    match bibliography {
        Ok(_) => {
            panic!("Should return Err")
        }
        Err(s) => {
            assert_eq!(s.span, 369..369);
            assert_eq!(s.kind, ParseErrorKind::Expected(biblatex::Token::Equals));
        }
    };
}

#[test]
fn test_incorrect_data() {
    let contents = std::fs::read_to_string("tests/fixtures/invalid/incorrect_data.bib")
        .unwrap()
        .replace("\r\n", "\n");

    let bibliography = Bibliography::parse(&contents).unwrap();
    let rashid = bibliography.get("rashid2016").unwrap();
    match rashid.pagination() {
        Err(RetrievalError::TypeError(s)) => {
            assert_eq!(s.span, 352..359);
            assert_eq!(s.kind, TypeErrorKind::UnknownPagination);
        }
        _ => {
            panic!()
        }
    };
}
