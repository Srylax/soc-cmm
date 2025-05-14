// use std::collections::HashMap;

// use cmm_core::{Answer, Aspect, Control, Domain, Occurence, Satisfaction};

// fn main() {
//     let caps = [
//         ("1".into(), Control::new(Answer::Float(5.8))),
//         ("2".into(), Control::new(Answer::Bool(false))),
//         ("2.1".into(), Control::new(Answer::Float(0.0))),
//         (
//             "3".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::Mostly)),
//         ),
//         (
//             "4".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::Fully)),
//         ),
//         (
//             "5".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::Mostly)),
//         ),
//         (
//             "6".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::Mostly)),
//         ),
//         (
//             "7".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::Fully)),
//         ),
//         (
//             "8".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::Averagely)),
//         ),
//         (
//             "9".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::No)),
//         ),
//         (
//             "10".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::Fully)),
//         ),
//     ];
//     let soc_employees = Aspect::new(Domain::People, 1, HashMap::from(caps));
//     println!("factor: {}", soc_employees.factor());
//     println!("total score: {}", soc_employees.total_score());
//     println!("max score: {}", soc_employees.max_score());
//     println!("final score: {}", soc_employees.final_score());

//     let caps = [
//         (
//             "1".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::Averagely)),
//         ),
//         ("2".into(), Control::new(Answer::None)),
//         ("2.1".into(), Control::new(Answer::Bool(false))),
//         ("2.2".into(), Control::new(Answer::Bool(false))),
//         ("2.3".into(), Control::new(Answer::Bool(false))),
//         ("2.4".into(), Control::new(Answer::Bool(false))),
//         ("2.5".into(), Control::new(Answer::Bool(false))),
//         ("2.6".into(), Control::new(Answer::Bool(false))),
//         ("2.7".into(), Control::new(Answer::Bool(false))),
//         ("2.8".into(), Control::new(Answer::Bool(false))),
//         ("2.9".into(), Control::new(Answer::Bool(false))),
//         ("2.10".into(), Control::new(Answer::Bool(false))),
//         ("2.11".into(), Control::new(Answer::Bool(false))),
//         ("2.12".into(), Control::new(Answer::Bool(false))),
//         ("2.13".into(), Control::new(Answer::Bool(false))),
//         (
//             "2.14".into(),
//             Control::new(Answer::Text("CISO, Security Team".to_owned())),
//         ),
//         (
//             "3".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::No)),
//         ),
//         (
//             "4".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::Fully)),
//         ),
//         (
//             "5".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::No)),
//         ),
//         (
//             "6".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::No)),
//         ),
//         ("7".into(), Control::new(Answer::None)),
//         ("7.1".into(), Control::new(Answer::Bool(false))),
//         ("7.2".into(), Control::new(Answer::Bool(false))),
//         ("7.3".into(), Control::new(Answer::Bool(false))),
//         ("7.4".into(), Control::new(Answer::Bool(false))),
//         ("7.5".into(), Control::new(Answer::Bool(false))),
//         ("7.6".into(), Control::new(Answer::Bool(false))),
//         ("7.7".into(), Control::new(Answer::Bool(false))),
//         ("7.8".into(), Control::new(Answer::Bool(false))),
//         (
//             "8".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::Mostly)),
//         ),
//         (
//             "9".into(),
//             Control::new(Answer::Satisfaction(Satisfaction::No)),
//         ),
//         (
//             "10".into(),
//             Control::new(Answer::Occurence(Occurence::Always)),
//         ),
//     ];
//     let soc_employees = Aspect::new(Domain::People, 2, HashMap::from(caps));
//     println!("factor: {}", soc_employees.factor());
//     println!("total score: {}", soc_employees.total_score());
//     println!("max score: {}", soc_employees.max_score());
//     println!("final score: {}", soc_employees.final_score());
// }
