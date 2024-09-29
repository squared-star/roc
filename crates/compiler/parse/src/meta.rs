// Copyright © 2024 Squared Star. All rights reserved.

use bumpalo::Bump;
use roc_region::all::Loc;

use crate::{ast::{Expr, Malformed, TypeAnnotation}, normalize::Normalize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceVisibility {Implicit, Explicit}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erasure {Erased, Normal}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Stage {Meta, Run}



#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DependentFunction<'a> {
    pub erasure: Erasure,
    pub visibility: SourceVisibility,
    pub parameter_name: &'a str,
    pub parameter_annotation: &'a Loc<TypeAnnotation<'a>>,
    pub return_type: &'a Loc<TypeAnnotation<'a>>

}

impl<'a> Malformed for DependentFunction<'a> {
    fn is_malformed(&self) -> bool {
        self.parameter_annotation.is_malformed() || self.return_type.is_malformed()
    }
}


impl<'a> Normalize<'a> for DependentFunction<'a> {
    fn normalize(&self, arena: &'a Bump) -> Self {
        
        DependentFunction {
            parameter_annotation: (&self.parameter_annotation).normalize(arena),
            return_type: (&self.return_type).normalize(arena),
            .. *self
        }
    }
}

/// The underlying curried call by push value style function type in Dirac.
/// Roc-style functions such as U64, U64 -> U64 will be treated like U64 -> U64 -> Partial U64
/// 
/// Currying makes working with functions during staging more convenient
pub struct PrimitiveFunction<'a> {
    pub input: &'a Loc<TypeAnnotation<'a>>,
    pub output: &'a Loc<TypeAnnotation<'a>>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MetaDef<'a> {
    Annotation(Loc<&'a str>, Loc<TypeAnnotation<'a>>),
    Body(Loc<&'a str>, Loc<Expr<'a>>),
}

impl<'a> Normalize<'a> for MetaDef<'a> {
    fn normalize(&self, arena: &'a Bump) -> Self {
        match self {
            MetaDef::Annotation(name, annotation) => MetaDef::Annotation(name.normalize(arena), annotation.normalize(arena)),
            MetaDef::Body(name, body) => MetaDef::Body(name.normalize(arena), body.normalize(arena))
        }
    }
}