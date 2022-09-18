use gandalf::{
    interpreter::Environment,
    lf::{Context, Family, FamilyName, Kind, Term, TermName, VarName},
};

fn main() {
    let mut environment = Environment::new();
    let nat_family = FamilyName::new("nat");
    let z_term = TermName::new("z");
    let s_term = TermName::new("s");

    let even_family = FamilyName::new("even");
    let even_z_term = TermName::new("even_z");
    let even_s_term = TermName::new("even_s");

    // Declare nat type
    environment
        .declare_family(&nat_family, &Kind::Type)
        .unwrap();
    environment
        .declare_term(&z_term, &Family::Const(nat_family.clone()))
        .unwrap();
    environment
        .declare_term(
            &s_term,
            &Family::Abs(
                Box::new(Family::Const(nat_family.clone())),
                Box::new(Family::Const(nat_family.clone())),
            ),
        )
        .unwrap();

    // Declare even judgment
    environment
        .declare_family(
            &even_family,
            &Kind::Abs(Family::Const(nat_family.clone()), Box::new(Kind::Type)),
        )
        .unwrap();
    environment
        .declare_term(
            &even_z_term,
            &Family::App(
                Box::new(Family::Const(even_family.clone())),
                Term::Const(z_term.clone()),
            ),
        )
        .unwrap();
    environment
        .declare_term(
            &even_s_term,
            &Family::Abs(
                Box::new(Family::Const(nat_family.clone())),
                Box::new(Family::Abs(
                    Box::new(Family::App(
                        Box::new(Family::Const(even_family.clone())),
                        Term::Var(VarName::new(0)),
                    )),
                    Box::new(Family::App(
                        Box::new(Family::Const(even_family.clone())),
                        Term::App(
                            Box::new(Term::Const(s_term.clone())),
                            Box::new(Term::App(
                                Box::new(Term::Const(s_term.clone())),
                                Box::new(Term::Const(z_term.clone())),
                            )),
                        ),
                    )),
                )),
            ),
        )
        .unwrap();

    // let family = environment
    //     .check_term(&Context::new(), &Term::Const(z_term.clone()))
    //     .unwrap();
    // println!("{:?}", family);

    // let family = environment
    //     .check_term(
    //         &Context::new(),
    //         &Term::App(
    //             Box::new(Term::Const(s_term.clone())),
    //             Box::new(Term::Const(z_term.clone())),
    //         ),
    //     )
    //     .unwrap();
    // println!("{:?}", family);

    let family = environment
        .check_term(
            &Context::new(),
            &Term::App(
                Box::new(Term::App(
                    Box::new(Term::Const(even_s_term.clone())),
                    Box::new(Term::Const(z_term.clone())),
                )),
                Box::new(Term::Const(even_z_term.clone())),
            ),
        )
        .unwrap();

    assert_eq!(
        family,
        Family::App(
            Box::new(Family::Const(even_family.clone())),
            Term::App(
                Box::new(Term::Const(s_term.clone())),
                Box::new(Term::App(
                    Box::new(Term::Const(s_term.clone())),
                    Box::new(Term::Const(z_term.clone())),
                ))
            )
        )
    );
    println!("{:?}", family);
}
