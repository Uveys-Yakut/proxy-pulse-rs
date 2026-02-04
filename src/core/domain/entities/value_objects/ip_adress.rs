use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IpAdress(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Port(pub String);
