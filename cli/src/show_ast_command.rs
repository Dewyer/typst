use std::fs;
use std::path::PathBuf;
use typst::diag::StrResult;
use typst::syntax::ast::{AstNode, Markup};

/// Show the AST of a .typ file
pub struct ShowAstCommand {
    pub input: PathBuf,
}

pub fn show_ast(command: ShowAstCommand) -> StrResult<()> {
    let input_content = fs::read_to_string(command.input)
        .map_err(|err| format!("failed to read input file: {}", err))?;
    let parsed = typst::syntax::parse(&input_content);

    let markup = Markup::from_untyped(&parsed).ok_or("input file is not valid markup")?;

    println!("{:#?}", markup);

    Ok(())
}
