use gandalf::lf::{eval::Environment, Context, Family, FamilyName, Kind, Term, TermName, VarName};

fn main() {
    let mut environment = Environment::new();

    // Declare nat type
    let nat_family = environment
        .declare_family(&FamilyName::new("nat"), &Kind::Type)
        .unwrap();
    let z_term = environment
        .declare_term(&TermName::new("z"), &nat_family)
        .unwrap();
    let s_term = environment
        .declare_term(
            &TermName::new("s"),
            &Family::Abs(Box::new(nat_family.clone()), Box::new(nat_family.clone())),
        )
        .unwrap();

    // Declare even judgment
    let even_family = environment
        .declare_family(
            &FamilyName::new("even"),
            &Kind::Abs(nat_family.clone(), Box::new(Kind::Type)),
        )
        .unwrap();
    let even_z_term = environment
        .declare_term(
            &TermName::new("even_z"),
            &Family::App(Box::new(even_family.clone()), z_term.clone()),
        )
        .unwrap();
    let even_s_term = environment
        .declare_term(
            &TermName::new("even_s"),
            &Family::Abs(
                Box::new(nat_family),
                Box::new(Family::Abs(
                    Box::new(Family::App(
                        Box::new(even_family.clone()),
                        Term::Var(VarName::new(0)),
                    )),
                    Box::new(Family::App(
                        Box::new(even_family.clone()),
                        Term::App(
                            Box::new(s_term.clone()),
                            Box::new(Term::App(
                                Box::new(s_term.clone()),
                                Box::new(z_term.clone()),
                            )),
                        ),
                    )),
                )),
            ),
        )
        .unwrap();

    let family = environment
        .check_term(
            &Context::new(),
            &Term::App(
                Box::new(Term::App(
                    Box::new(even_s_term.clone()),
                    Box::new(z_term.clone()),
                )),
                Box::new(even_z_term.clone()),
            ),
        )
        .unwrap();

    assert_eq!(
        family,
        Family::App(
            Box::new(even_family.clone()),
            Term::App(
                Box::new(s_term.clone()),
                Box::new(Term::App(
                    Box::new(s_term.clone()),
                    Box::new(z_term.clone()),
                ))
            )
        )
    );
    println!("{:?}", family);
}
