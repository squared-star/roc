// Copyright © 2024 Squared Star. All rights reserved.

use bumpalo::Bump;
use roc_region::all::{Loc, Position};

use crate::{
    ast::{Expr, Malformed, TypeAnnotation},
    normalize::Normalize,
    parser::{map, skip_first, specialize_err, two_bytes, EType, Either, Parser, Progress},
    state::State,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceVisibility {
    Implicit,
    Explicit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erasure {
    Erased,
    Normal,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Meta,
    Run,
}

/// Syntax (explicit unlabeled): / x : A -> B x
/// Syntax (implicit unlabeled): / <x : A> -> B x
/// Syntax (implicit unlabeled erased): / <0 x : A> -> B x or B 𝐴
/// Syntax (implicit labeled erased): <0 x : A> -> B x or B 'x or 'A -> A
/// Syntax (explicit erased): /^0 x : A -> B x
/// Syntax (explicit labeled): x : A -> B x
/// Syntax (explicit labeled erased): 0 x : A -> B x
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DependentFunction<'a> {
    pub erasure: Erasure,
    pub visibility: SourceVisibility,
    pub parameter_name: &'a str,
    pub parameter_annotation: &'a Loc<TypeAnnotation<'a>>,
    pub return_type: &'a Loc<TypeAnnotation<'a>>,
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
            ..*self
        }
    }
}

/// The underlying curried call by push value style function type in Dirac.
/// Roc-style functions such as U64, U64 -> U64 will be treated like U64 -> U64 -> Partial U64
///
/// Currying makes working with functions during staging more convenient
pub struct PrimitiveFunction<'a> {
    pub input: &'a Loc<TypeAnnotation<'a>>,
    pub output: &'a Loc<TypeAnnotation<'a>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MetaDef<'a> {
    Annotation(Loc<&'a str>, Loc<TypeAnnotation<'a>>),
    Body(Loc<&'a str>, Loc<Expr<'a>>),
}

impl<'a> Normalize<'a> for MetaDef<'a> {
    fn normalize(&self, arena: &'a Bump) -> Self {
        match self {
            MetaDef::Annotation(name, annotation) => {
                MetaDef::Annotation(name.normalize(arena), annotation.normalize(arena))
            }
            MetaDef::Body(name, body) => {
                MetaDef::Body(name.normalize(arena), body.normalize(arena))
            }
        }
    }
}

use crate::ident::lowercase_ident;
use crate::parser::either;
pub fn implicitly_bound_type_variable_ident<'a>() -> impl Parser<'a, &'a str, ()> {
    map(
        either(
            move |_, state: State<'a>, _| match chomp_italicized(&state.bytes()) {
                Ok(ident) => Ok((Progress::MadeProgress, ident, state.advance(ident.len()))),
                Err(progress) => Err((progress, ())),
            },
            lowercase_ident(),
        ),
        |ident| match ident {
            Either::First(ident) => ident,
            Either::Second(ident) => ident,
        },
    )
}
const ITALICS_UNICODE_START: char = '𝐴';
const ITALICS_UNICODE_END: char = '𝑧';

fn is_italicized_unicode(c: char) -> bool {
    c >= ITALICS_UNICODE_START && c <= ITALICS_UNICODE_END
}

fn chomp_italicized(buffer: &[u8]) -> Result<&str, Progress> {
    use encode_unicode::CharExt;
    let mut chomped = 0;
    if let Ok((character, width)) = char::from_utf8_slice_start(&buffer[chomped..]) {
        if !is_italicized_unicode(character) {
            return Err(Progress::NoProgress);
        }
        chomped += width;
    }

    while let Ok((character, width)) = char::from_utf8_slice_start(&buffer[chomped..]) {
        if is_italicized_unicode(character) {
            chomped += width;
        } else {
            break;
        }
    }

    if chomped == 0 {
        Err(Progress::NoProgress)
    } else {
        Ok(std::str::from_utf8(&buffer[..chomped]).unwrap())
    }
}

enum EDependentFunctionType<'a> {
    Start(Position),
    Colon(Position),
    Arrow(Position),
    ParameterName(Position),
    ParameterAnnotation(EType<'a>, Position),
    ReturnType(EType<'a>, Position),
}

use crate::ident::unqualified_ident;
use crate::parser::Progress::{MadeProgress, NoProgress};
use crate::parser::{allocated, byte_indent, loc, succeed, word};
use crate::type_annotation;
fn parse_dependent_function<'a>(
) -> impl Parser<'a, Loc<DependentFunction<'a>>, EDependentFunctionType<'a>> {
    skip_first(
        byte_indent(b'/', EDependentFunctionType::Start),
        loc(record! {
            DependentFunction {
                erasure: succeed(Erasure::Normal),
                visibility: succeed(SourceVisibility::Explicit),
                parameter_name: specialize_err(|_, position| EDependentFunctionType::ParameterName(position), unqualified_ident()),
                parameter_annotation:
                    skip_first(
                        byte_indent(b':', EDependentFunctionType::Colon),
                        specialize_err(EDependentFunctionType::ParameterAnnotation, allocated(type_annotation::located(false)))
                    ),
                return_type:
                    skip_first(
                        one_of!(two_bytes(b'-', b'>', EDependentFunctionType::Arrow), word("→", EDependentFunctionType::Arrow)),
                        specialize_err(EDependentFunctionType::ReturnType, allocated(type_annotation::located(false)))
                    ),
            }
        }),
    )
}
