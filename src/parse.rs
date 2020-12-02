use crate::CfaResult;
use swc_common::comments::SingleThreadedComments;
use swc_common::{FileName, Globals, Mark, SourceMap};
use swc_ecmascript::ast::Program;
use swc_ecmascript::parser::lexer::Lexer;
use swc_ecmascript::parser::{JscTarget, Parser, StringInput, Syntax, TsConfig};
use swc_ecmascript::transforms::resolver::ts_resolver;
use swc_ecmascript::visit::FoldWith;

const DUMMY_FILE_NAME: &str = "DUMMY.ts";

pub(crate) fn parse_as_program(source_code: String) -> CfaResult<Program> {
    let syntax = get_default_ts_config();
    let source_map = SourceMap::default();
    let swc_source_file =
        source_map.new_source_file(FileName::Custom(DUMMY_FILE_NAME.to_string()), source_code);
    let comments = SingleThreadedComments::default();

    let lexer = Lexer::new(
        syntax,
        JscTarget::Es2019,
        StringInput::from(&*swc_source_file),
        Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);
    let globals = Globals::new();
    let top_level_mark = swc_common::GLOBALS.set(&globals, || Mark::fresh(Mark::root()));

    match parser.parse_program() {
        Err(_) => Err("parse error"),
        Ok(program) => {
            let program = swc_common::GLOBALS.set(&globals, || {
                program.fold_with(&mut ts_resolver(top_level_mark))
            });
            Ok(program)
        }
    }
}

fn get_default_ts_config() -> Syntax {
    let mut ts_config = TsConfig::default();
    ts_config.dynamic_import = true;
    ts_config.decorators = true;
    Syntax::Typescript(ts_config)
}
