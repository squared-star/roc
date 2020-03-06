#![warn(clippy::all, clippy::dbg_macro)]
// I'm skeptical that clippy:large_enum_variant is a good lint to have globally enabled.
//
// It warns about a performance problem where the only quick remediation is
// to allocate more on the heap, which has lots of tradeoffs - including making it
// long-term unclear which allocations *need* to happen for compilation's sake
// (e.g. recursive structures) versus those which were only added to appease clippy.
//
// Effectively optimizing data struture memory layout isn't a quick fix,
// and encouraging shortcuts here creates bad incentives. I would rather temporarily
// re-enable this when working on performance optimizations than have it block PRs.
#![allow(clippy::large_enum_variant)]

pub mod uniqueness;

pub mod string;

pub mod builtins;
pub mod unique_builtins;

pub mod constrain;
pub mod crane;
pub mod fmt;
pub mod llvm;
pub mod load;
pub mod mono;
pub mod pretty_print_types;
pub mod reporting;
pub mod solve;
pub mod unify;
