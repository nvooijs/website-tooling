use crate::Result;
use oxc_allocator::Allocator;
use oxc_codegen::{Codegen, CodegenOptions, CommentOptions};
use oxc_minifier::Minifier;
use oxc_parser::Parser;
use oxc_span::SourceType;

pub enum JsType {
    JsScript,
    JsModule,
}

pub fn minify(source: &str, js_type: JsType) -> Result<String> {
    let allocator = Allocator::new();
    let source_type = match js_type {
        JsType::JsModule => SourceType::mjs(),
        JsType::JsScript => SourceType::cjs(),
    };
    let parser = Parser::new(&allocator, source, source_type);
    let ast = parser.parse();

    if !ast.errors.is_empty() {
        // TODO: proper error message
        anyhow::bail!("Errors in js source");
    }

    let mut program = ast.program;

    let minifier = Minifier::new(Default::default());
    let minified = minifier.minify(&allocator, &mut program);

    let minified = Codegen::new()
        .with_options(CodegenOptions {
            minify: true,
            comments: CommentOptions {
                legal: oxc_codegen::LegalComment::Inline,
                ..CommentOptions::disabled()
            },
            ..CodegenOptions::default()
        })
        .with_scoping(minified.scoping)
        .build(&program);

    Ok(minified.code)
}
