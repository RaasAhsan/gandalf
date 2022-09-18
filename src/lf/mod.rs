use std::collections::{HashMap, VecDeque};

pub mod eval;

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Type,
    Abs(Family, Box<Kind>), // TODO: not binding
}

#[derive(Clone, Debug, PartialEq)]
pub enum Family {
    Const(FamilyName),
    Abs(Box<Family>, Box<Family>),
    App(Box<Family>, Term),
}

impl Family {
    pub fn substitute_var(&mut self, index: usize, subst: &Term) {
        match self {
            Family::Abs(f1, f2) => {
                f1.substitute_var(index, subst);
                f2.substitute_var(index + 1, subst)
            }
            Family::App(f, t) => {
                f.substitute_var(index, subst);
                t.substitute_var(index, subst);
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Var(VarName),
    Const(TermName),
    App(Box<Term>, Box<Term>),
}

impl Term {
    pub fn substitute_var(&mut self, index: usize, subst: &Term) {
        match self {
            Term::Var(i) if i.0 == index => *self = subst.clone(),
            Term::App(t1, t2) => {
                t1.substitute_var(index, subst);
                t2.substitute_var(index, subst);
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FamilyName(String);

impl FamilyName {
    pub fn new<A: AsRef<str>>(name: A) -> Self {
        FamilyName(name.as_ref().into())
    }
}

impl<A> From<A> for FamilyName
where
    A: AsRef<str>,
{
    fn from(a: A) -> Self {
        FamilyName::new(a)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct VarName(usize);

impl VarName {
    pub fn new(index: usize) -> Self {
        VarName(index)
    }
}

impl From<usize> for VarName {
    fn from(a: usize) -> Self {
        VarName::new(a)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TermName(String);

impl TermName {
    pub fn new<A: AsRef<str>>(name: A) -> Self {
        TermName(name.as_ref().into())
    }
}

impl<A> From<A> for TermName
where
    A: AsRef<str>,
{
    fn from(a: A) -> Self {
        TermName::new(a)
    }
}

#[derive(Debug, Default)]
pub struct Signature {
    terms: HashMap<TermName, Family>,
    families: HashMap<FamilyName, Kind>,
}

impl Signature {
    pub fn new() -> Self {
        Signature {
            terms: HashMap::new(),
            families: HashMap::new(),
        }
    }

    pub fn get_term(&self, name: &TermName) -> Option<&Family> {
        self.terms.get(name)
    }

    pub fn get_family(&self, name: &FamilyName) -> Option<&Kind> {
        self.families.get(name)
    }

    // TODO: should these validate or we expect it as a precondition?
    pub fn add_term(&mut self, term: TermName, family: Family) {
        self.terms.insert(term, family);
    }

    pub fn add_family(&mut self, family: FamilyName, kind: Kind) {
        self.families.insert(family, kind);
    }
}

/// Maintains the context of bound variables for checking terms and types.
/// Uses a de Bruijn index representation for variables, implemented with a vector.
#[derive(Clone, Debug)]
pub struct Context {
    indexes: VecDeque<Family>, // TODO: lifetime-bounded family reference?
}

impl Context {
    pub fn new() -> Self {
        Context {
            indexes: VecDeque::new(),
        }
    }

    pub fn push(&self, family: &Family) -> Context {
        let mut next = self.clone();
        next.indexes.push_front(family.clone());
        next
    }

    pub fn get(&self, index: &VarName) -> Option<&Family> {
        self.indexes.get(index.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::lf::Family;

    use super::{Term, VarName};

    #[test]
    fn subst_term() {
        let mut term = Term::Var(VarName(0));
        term.substitute_var(0, &Term::Const("x".into()));
        assert_eq!(term, Term::Const("x".into()));

        let mut term = Term::Var(VarName(0));
        term.substitute_var(0, &Term::Const("x".into()));
        assert_eq!(term, Term::Const("x".into()));

        let mut term = Term::App(
            Box::new(Term::Var(VarName(0))),
            Box::new(Term::Var(VarName(1))),
        );
        term.substitute_var(0, &Term::Const("x".into()));
        assert_eq!(
            term,
            Term::App(
                Box::new(Term::Const("x".into())),
                Box::new(Term::Var(VarName(1))),
            )
        );
    }

    #[test]
    fn subst_family() {
        let mut family = Family::Const("nat".into());
        family.substitute_var(0, &Term::Const("x".into()));
        assert_eq!(family, Family::Const("nat".into()));

        let mut family = Family::App(
            Box::new(Family::Const("even".into())),
            Term::Var(VarName::new(0)),
        );
        family.substitute_var(0, &Term::Const("z".into()));
        assert_eq!(
            family,
            Family::App(
                Box::new(Family::Const("even".into())),
                Term::Const("z".into()),
            )
        );

        let mut family = Family::Abs(
            Box::new(Family::Const("nat".into())),
            Box::new(Family::App(
                Box::new(Family::Const("even".into())),
                Term::Var(VarName::new(0)),
            )),
        );
        family.substitute_var(0, &Term::Const("x".into()));
        assert_eq!(
            family,
            Family::Abs(
                Box::new(Family::Const("nat".into())),
                Box::new(Family::App(
                    Box::new(Family::Const("even".into())),
                    Term::Var(VarName::new(0)),
                )),
            )
        );
    }
}
