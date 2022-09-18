use gandalf::{
    interpreter::Environment,
    lf::{Family, FamilyName, Kind, Signature, TermName},
};

fn main() {
    let mut environment = Environment::new();
    let nat_family = FamilyName::new("nat");
    let z_term = TermName::new("z");
    let s_term = TermName::new("s");
    environment.declare_family(&nat_family, &Kind::Type);
    environment.declare_term(&z_term, Family::Const(nat_family.clone()));
    environment.declare_term(
        &s_term,
        Family::Abs(
            Box::new(Family::Const(nat_family.clone())),
            Box::new(Family::Const(nat_family.clone())),
        ),
    );
}
