use cmm_core::{CID, CMM, Domain, control::SimpleControl};
use indexmap::IndexMap;

#[test]
fn test_extend_serialize() {
    let mut cmm: CMM = serde_json::from_str(include_str!("../../scheme-2.3.4.json")).unwrap();
    let simple: IndexMap<Domain, IndexMap<CID, SimpleControl>> =
        toml::from_str(include_str!("<INSERT PATH>")).unwrap();
    cmm.extend_with_simple(simple.clone()).unwrap();
    let new_simple = cmm.as_simple();
    assert_eq!(simple, new_simple)
}
