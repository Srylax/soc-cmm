use cmm_core::{
    answer::{Answer, DetailedOptional},
    cid::CID,
    control::Control,
    data::SOCData,
    schema::Schema,
};
use indexmap::IndexMap;
use pretty_assertions::assert_eq;

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
    let cmm = SOCData::from(controls, Some("Hello world!".to_string()));
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

#[test]
fn test_multiline_notes() {
    let src = r#"notes = """
# Hello World!

Line breaks **woo**!
"""
"#;
    let parsed_cmm: SOCData = toml::from_str(src).unwrap();
    assert_eq!(src, toml::to_string(&parsed_cmm).unwrap());
}

#[test]
fn test_soc_cmm_2_3_4() {
    serde_json::from_str::<Schema>(include_str!("../../scheme-2.3.4.json")).unwrap();
}
