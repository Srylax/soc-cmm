use cmm_core::{
    CMM,
    answer::{Answer, DetailedOptional},
    cid::CID,
    control::Control,
};
use indexmap::IndexMap;

/// Test if the format is as defined
#[test]
fn test_toml_format() {
    let mut controls: IndexMap<CID, Control> = IndexMap::new();
    controls.insert(
        "Business.1.2".parse().unwrap(),
        Control::new(
            Answer::DetailedOptional(DetailedOptional::Fully),
            Some(String::from("Comment!")),
        ),
    );
    let cmm = CMM::from(controls, Some("Hello world!".to_string()));
    assert_eq!(
        r#"notes = "Hello world!"

["Business.1.2"]
comment = "Comment!"
type = "DetailedOptional"
answer = "Fully"
"#,
        toml::to_string(&cmm).unwrap(),
    );
}
