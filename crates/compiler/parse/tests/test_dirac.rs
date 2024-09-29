// Copyright © 2024 Squared Star. All rights reserved.
#[cfg(test)]
#[cfg(feature = "2ltt")]
mod test_dirac {
    use roc_parse::header::parse_module_defs;
    use bumpalo::collections::vec::Vec;
    use bumpalo::{self, Bump};
    use roc_parse::state::State;
    use roc_parse::ast;

    #[test]
    fn test_implicit_dependent_function_types() {
        let arena = Bump::new();
        let src =
            r"
            meta
            id : for A : Type. A -> A
            id = \ x -> x
            ";

        let state = State::new(&src.as_bytes());
        let parsed = parse_module_defs(&arena, state, ast::Defs::default());
        assert!(parsed.is_ok());

    }
    // id : a -> a
    // id = \ x -> x
    #[test]
    fn test_explicit_dependent_function_types() {
        let arena = Bump::new();
        let src =
            r"
            meta
            id : A : Type -> A -> A
            id = \ a, x -> x
            ";

        let state = State::new(&src.as_bytes());
        let parsed = parse_module_defs(&arena, state, ast::Defs::default());
        assert!(parsed.is_ok());

        
    }
    fn test_meta_def() {
        let arena = Bump::new();
        let src =
            r"
            meta
            and : ^Bool -> ^Bool -> ^Bool
            and = \ a, b -> ^{if ^[a] then ^[b] else false}
            ";
        let state = State::new(&src.as_bytes());
        let parsed = parse_module_defs(&arena, state, ast::Defs::default());
        assert!(parsed.is_ok());
    }
}