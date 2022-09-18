use super::{Family, Term};

pub fn family_abs(f1: Family, f2: Family) -> Family {
    Family::Abs(Box::new(f1), Box::new(f2))
}

pub fn family_app(f: Family, t: Term) -> Family {
    Family::App(Box::new(f), t)
}

pub fn term_app(t1: Term, t2: Term) -> Term {
    Term::App(Box::new(t1), Box::new(t2))
}
