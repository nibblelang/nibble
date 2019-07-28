mod context;
mod expression;

use crate::ast::*;
use context::Context;
use std::io;
use std::io::Write;

pub fn codegen(ast: &Expression, stream: &mut impl Write) -> io::Result<()> {
    writeln!(stream, ".intel_syntax noprefix")?;
    writeln!(
        stream,
        ".globl main\n\
         .globl _main\n\
         main:\n\
         _main:"
    )?;
    ast.generate(stream, &mut Context::empty())?;
    writeln!(stream, "ret")?;
    Ok(())
}

trait Generator {
    fn generate(&self, stream: &mut impl Write, ctx: &mut Context) -> io::Result<()>;
}
