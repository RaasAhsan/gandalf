use std::collections::HashMap;

#[derive(Debug)]
pub enum Kind {
    Type,
    Abs(Family, Box<Kind>),
}

#[derive(Debug)]
pub enum Family {
    Const(FamilyName),
    Dabs(Box<Family>, Box<Family>),
    Abs(Box<Family>, Box<Family>),
    App(Box<Family>, Term),
}

#[derive(Debug)]
pub enum Term {
    Const(TermName),
}

#[derive(Debug, PartialEq)]
pub struct FamilyName(String);

#[derive(Debug, PartialEq)]
pub struct TermName(String);

#[derive(Debug)]
pub struct Signature {
    objects: HashMap<TermName, Family>,
    families: HashMap<FamilyName, Kind>,
}
