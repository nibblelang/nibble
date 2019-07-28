use crate::ast::*;
use super::context::Context;
use std::io;
use std::io::Write;
use super::Generator;

impl Generator for Expression {
    fn generate(&self, stream: &mut impl Write, ctx: &mut Context) -> io::Result<()> {
        match self {
            Expression::Int(val) => {
                writeln!(stream, "mov rax, {}", val)?;
            }
            Expression::Bool(val) => {
                writeln!(stream, "mov rax, {}", if *val { 1 } else { 0 })?;
            }
            Expression::Conditional(cond, exp, alt) => {
                let alt_label = ctx.unique_label();
                let post_conditional = ctx.unique_label();

                cond.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "cmp rax, 0\n\
                     je {}",
                    alt_label
                )?;
                exp.generate(stream, ctx)?;
                writeln!(
                    stream,
                    "jmp {}\n\
                     {}:",
                    post_conditional, alt_label
                )?;
                alt.generate(stream, ctx)?;
                writeln!(stream, "{}:", post_conditional)?;
            }
            Expression::BinaryOp(op, a, b) => match op {
                BinaryOp::Multiply => {
                    a.generate(stream, ctx)?;
                    writeln!(stream, "push rax")?;
                    b.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "pop rcx\n\
                         imul rax, rcx"
                    )?;
                }
                BinaryOp::Divide => {
                    a.generate(stream, ctx)?;
                    writeln!(stream, "push rax")?;
                    b.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "mov rdx, 0\n\
                         mov rcx, rax\n\
                         pop rax\n\
                         idiv rcx"
                    )?;
                }
                BinaryOp::Add => {
                    a.generate(stream, ctx)?;
                    writeln!(stream, "push rax")?;
                    b.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "pop rcx\n\
                         add rax, rcx"
                    )?;
                }
                BinaryOp::Sub => {
                    b.generate(stream, ctx)?;
                    writeln!(stream, "push rax")?;
                    a.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "pop rcx\n\
                         sub rax, rcx"
                    )?;
                }
                BinaryOp::LessThan => {
                    a.generate(stream, ctx)?;
                    writeln!(stream, "push rax")?;
                    b.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "pop rcx\n\
                         cmp rcx, rax\n\
                         mov rax, 0\n\
                         setl al"
                    )?;
                }
                BinaryOp::LessThanOrEqual => {
                    a.generate(stream, ctx)?;
                    writeln!(stream, "push rax")?;
                    b.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "pop rcx\n\
                         cmp rcx, rax\n\
                         mov rax, 0\n\
                         setle al"
                    )?;
                }
                BinaryOp::GreaterThan => {
                    a.generate(stream, ctx)?;
                    writeln!(stream, "push rax")?;
                    b.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "pop rcx\n\
                         cmp rcx, rax\n\
                         mov rax, 0\n\
                         setg al"
                    )?;
                }
                BinaryOp::GreaterThanOrEqual => {
                    a.generate(stream, ctx)?;
                    writeln!(stream, "push rax")?;
                    b.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "pop rcx\n\
                         cmp rcx, rax\n\
                         mov rax, 0\n\
                         setge al"
                    )?;
                }
                BinaryOp::Equal => {
                    a.generate(stream, ctx)?;
                    writeln!(stream, "push rax")?;
                    b.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "pop rcx\n\
                         cmp rcx, rax\n\
                         mov rax, 0\n\
                         sete al"
                    )?;
                }
                BinaryOp::NotEqual => {
                    a.generate(stream, ctx)?;
                    writeln!(stream, "push rax")?;
                    b.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "pop rcx\n\
                         cmp rcx, rax\n\
                         mov rax, 0\n\
                         setne al"
                    )?;
                }
                BinaryOp::And => {
                    let end = ctx.unique_label();
                    let second_clause = ctx.unique_label();
                    a.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "cmp rax, 0\n\
                         jne {}\n\
                         jmp {}\n\
                         {}:",
                        second_clause, end, second_clause
                    )?;
                    b.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "cmp rax, 0\n\
                         mov rax, 0\n\
                         setne al\n\
                         {}:",
                        end
                    )?;
                }
                BinaryOp::Or => {
                    let end = ctx.unique_label();
                    let second_clause = ctx.unique_label();
                    a.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "cmp rax, 0\n\
                         je {}\n\
                         mov rax, 1\n\
                         jmp {}\n\
                         {}:",
                        second_clause, end, second_clause
                    )?;
                    b.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "cmp rax, 0\n\
                         mov rax, 0\n\
                         setne al\n\
                         {}:",
                        end
                    )?;
                }
            },
            Expression::UnaryOp(op, a) => match op {
                UnaryOp::Minus => {
                    a.generate(stream, ctx)?;
                    writeln!(stream, "neg rax")?;
                }
                UnaryOp::Not => {
                    a.generate(stream, ctx)?;
                    writeln!(
                        stream,
                        "cmp rax, 0\n\
                         mov rax, 0\n\
                         sete al"
                    )?;
                }
            },
        }
        Ok(())
    }
}
