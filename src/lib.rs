mod parse;

use swc_ecmascript::ast::Program;

// TODO(magurotuna): define original error enum
pub type CfaResult<T> = Result<T, &'static str>;

#[derive(Debug, Default)]
pub struct ControlFlow {}

pub fn analyze(source_code: impl Into<String>) -> CfaResult<ControlFlow> {
    let program = parse::parse_as_program(source_code.into())?;
    analyze_from_program(program)
}

pub fn analyze_from_program(program: Program) -> CfaResult<ControlFlow> {
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
