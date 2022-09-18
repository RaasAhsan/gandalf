use gandalf::lf::{
    dsl::{family_abs, family_app, term_app},
    eval::Environment,
    Context, Family, FamilyName, Kind, Term, TermName, VarName,
};

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
            &family_abs(nat_family.clone(), nat_family.clone()),
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
            &family_app(even_family.clone(), z_term.clone()),
        )
        .unwrap();
    let even_s_term = environment
        .declare_term(
            &TermName::new("even_s"),
            &family_abs(
                nat_family,
                family_abs(
                    family_app(even_family.clone(), Term::Var(VarName::new(0))),
                    family_app(
                        even_family.clone(),
                        term_app(
                            s_term.clone(),
                            term_app(s_term.clone(), Term::Var(VarName::new(1))),
                        ),
                    ),
                ),
            ),
        )
        .unwrap();

    let family = environment
        .check_term(
            &Context::new(),
            &term_app(
                term_app(even_s_term.clone(), z_term.clone()),
                even_z_term.clone(),
            ),
        )
        .unwrap();

    assert_eq!(
        family,
        family_app(
            even_family.clone(),
            term_app(s_term.clone(), term_app(s_term.clone(), z_term.clone()))
        )
    );
    println!("{:?}", family);
}
