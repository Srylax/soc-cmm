use cmm_core::{Answer, Aspect, CAP, Domain, Satisfaction};

fn main() {
    let caps = vec![
        CAP::new(Answer::Float(5.8)),
        CAP::new(Answer::Bool(false)),
        CAP::new(Answer::Maturity(Satisfaction::Mostly)),
        CAP::new(Answer::Maturity(Satisfaction::Fully)),
        CAP::new(Answer::Maturity(Satisfaction::Mostly)),
        CAP::new(Answer::Maturity(Satisfaction::Mostly)),
        CAP::new(Answer::Maturity(Satisfaction::Fully)),
        CAP::new(Answer::Maturity(Satisfaction::Averagely)),
        CAP::new(Answer::Maturity(Satisfaction::No)),
        CAP::new(Answer::Maturity(Satisfaction::Fully)),
    ];
    let soc_employees = Aspect::new(Domain::People, 1, caps);
    println!("factor: {}", soc_employees.factor());
    println!("total score: {}", soc_employees.total_score());
    println!("max score: {}", soc_employees.max_score());
    println!("final score: {}", soc_employees.final_score());
}
