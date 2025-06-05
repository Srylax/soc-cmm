use cmm_core::{CID, CMM, Domain, answer::Answer, control::SimpleControl};

use indexmap::IndexMap;
use strum::VariantArray;

#[test]
fn test_extend_serialize() {
    let cmm: CMM = serde_json::from_str(include_str!("../../scheme-2.3.4.json")).unwrap();
    for domain in Domain::VARIANTS {
        for aspect in cmm.aspect(domain).unwrap() {
            for (cid, control) in aspect.controls() {
                if control.guidances().is_empty() {
                    match control.answer() {
                        Answer::Satisfaction(_)
                        | Answer::Detailed(_)
                        | Answer::DetailedOptional(_)
                        | Answer::Occurence(_) => {
                            if control.guidances().is_empty() {
                                panic!("{cid}");
                            }
                        }
                        Answer::Bool(_) | Answer::Any(_) | Answer::Title => {
                            if !control.guidances().is_empty() {
                                panic!("{cid}");
                            }
                        }
                    }
                }
            }
        }
    }
}
