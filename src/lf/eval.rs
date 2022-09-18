use crate::lf::{Context, Family, FamilyName, Kind, Signature, Term, TermName};

#[derive(Debug, Default)]
pub struct Environment {
    signature: Signature,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            signature: Signature::new(),
        }
    }

    pub fn declare_term(&mut self, name: &TermName, family: &Family) -> Result<(), Error> {
        if self.signature.get_term(name).is_none() {
            self.check_family(&Context::new(), family)?;
            self.signature.add_term(name.clone(), family.clone());
            Ok(())
        } else {
            Err(Error::TermAlreadyDefined)
        }
    }

    pub fn declare_family(&mut self, name: &FamilyName, kind: &Kind) -> Result<(), Error> {
        if self.signature.get_family(name).is_none() {
            self.check_kind(&Context::new(), kind)?;
            self.signature.add_family(name.clone(), kind.clone());
            Ok(())
        } else {
            Err(Error::FamilyAlreadyDefined)
        }
    }

    pub fn check_kind(&self, ctx: &Context, kind: &Kind) -> Result<(), Error> {
        match kind {
            Kind::Type => Ok(()),
            Kind::Abs(f, k) => {
                self.check_family(ctx, f)?;
                self.check_kind(ctx, k)?;
                Ok(())
            }
        }
    }

    pub fn check_family(&self, ctx: &Context, family: &Family) -> Result<Kind, Error> {
        match family {
            Family::Const(name) => self
                .signature
                .get_family(name)
                .cloned()
                .ok_or(Error::FamilyDoesntExist),
            Family::Abs(t, k) => {
                let next_ctx = ctx.push(t);
                let k = self.check_family(&next_ctx, k)?;
                Ok(k)
            }
            Family::App(f, t) => {
                let fk = self.check_family(ctx, f)?;
                match fk {
                    Kind::Type => Err(Error::ExpectedTypeAbsToApply),
                    Kind::Abs(f, k) => {
                        let tf = self.check_term(ctx, t)?;
                        if f == tf {
                            Ok(*k)
                        } else {
                            Err(Error::UnexpectedTermApp)
                        }
                    }
                }
            }
        }
    }

    pub fn check_term(&self, ctx: &Context, term: &Term) -> Result<Family, Error> {
        match term {
            Term::Var(name) => ctx.get(name).cloned().ok_or(Error::VarNotFound),
            Term::Const(name) => self
                .signature
                .get_term(name)
                .cloned()
                .ok_or(Error::TermDoesntExist),
            Term::App(t1, t2) => {
                let f1 = self.check_term(ctx, t1)?;
                let f2 = self.check_term(ctx, t2)?;
                match f1 {
                    Family::Abs(fi, mut fr) => {
                        if *fi == f2 {
                            fr.substitute_var(0, t2);
                            Ok(*fr)
                        } else {
                            Err(Error::UnexpectedType)
                        }
                    }
                    _ => Err(Error::ExpectedTermAbs),
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    VarNotFound,
    FamilyAlreadyDefined,
    TermAlreadyDefined,
    FamilyDoesntExist,
    TermDoesntExist,
    ExpectedTypeAbsToApply,
    UnexpectedTermApp,
    ExpectedTermAbs,
    UnexpectedType,
}
