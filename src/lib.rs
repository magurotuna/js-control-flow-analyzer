mod parse;

use std::collections::BTreeSet;
use std::fmt;
use swc_common::Span;
use swc_ecmascript::ast::Program;
use typed_arena::Arena;

// TODO(magurotuna): define original error enum
pub type CfaResult<T> = Result<T, &'static str>;

pub struct ControlFlow<'a> {
    code_paths: BTreeSet<CodePath<'a>>,
    segment_arena: Arena<CodePathSegment<'a>>,
}

impl<'a> ControlFlow<'a> {
    fn new() -> Self {
        Self {
            code_paths: BTreeSet::new(),
            segment_arena: Arena::new(),
        }
    }
}

impl<'a> Default for ControlFlow<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> fmt::Debug for ControlFlow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ControlFlow")
            .field("code_paths", &self.code_paths)
            .finish()
    }
}

#[derive(Debug)]
struct CodePath<'a> {
    span: Span,
    kind: CodePathKind,
    initial_segment: &'a CodePathSegment<'a>,
}

impl<'a> PartialEq for CodePath<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.span.eq(&other.span)
    }
}

impl<'a> Eq for CodePath<'a> {}

impl<'a> PartialOrd for CodePath<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.span.cmp(&other.span))
    }
}

impl<'a> Ord for CodePath<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.span.cmp(&other.span)
    }
}

#[derive(Debug)]
enum CodePathKind {
    Global,
    Function,
}

#[derive(Debug)]
struct CodePathSegment<'a> {
    span: Span,
    next_segments: Vec<&'a CodePathSegment<'a>>,
    prev_segments: Vec<&'a CodePathSegment<'a>>,
    reachable: bool,
}

pub fn analyze<'a>(source_code: impl Into<String>) -> CfaResult<ControlFlow<'a>> {
    let program = parse::parse_as_program(source_code.into())?;
    analyze_from_program(program)
}

pub fn analyze_from_program<'a>(program: Program) -> CfaResult<ControlFlow<'a>> {
    dbg!(program);

    // TODO
    Ok(ControlFlow::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_from_program() {
        let src = "if (true) { console.log(42); }";
        let result = analyze(src);
        assert!(result.is_ok());
    }
}
