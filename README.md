# gandalf

Gandalf is a proof assistant written in Rust based on the LF logical framework, an extension of the simply typed lambda calculus with dependent types. In LF, an object logic is encoded as an LF signature. Syntax is embedded as LF types and term constructors. Judgments in the object logic are represented as dependent LF types. Proofs or derivations of those judgments correspond to inhabitants of those types.

Using Gandalf, a user may describe a formal logic and write proofs concerning that logic. When a Gandalf program type-checks, the proofs encoded in the program are sound.

## Examples

The following example demonstrates the embedding of arithmetic over natural numbers via Rust code.

```rust
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
                family_app(even_family.clone(), term_var(0)),
                family_app(
                    even_family.clone(),
                    term_app(s_term.clone(), term_app(s_term.clone(), term_var(1))),
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
```

## Building

Clone the repository and run `cargo build` to compile the project.
