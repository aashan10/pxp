use crate::expressions;
use crate::internal::utils;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::variables::BracedVariableVariable;
use pxp_ast::variables::SimpleVariable;
use pxp_ast::variables::Variable;
use pxp_ast::variables::VariableVariable;

use pxp_diagnostics::Severity;
use pxp_token::TokenKind;

pub fn simple_variable(state: &mut State) -> SimpleVariable {
    let current = state.stream.current();

    match &current.kind {
        TokenKind::Variable => {
            state.stream.next();

            SimpleVariable { symbol: current.symbol.unwrap(), span: current.span }
        }
        TokenKind::Dollar => {
            state.stream.next();

            state.diagnostic(
                ParserDiagnostic::DynamicVariableNotAllowed,
                Severity::Error,
                current.span,
            );

            SimpleVariable { symbol: current.symbol.unwrap(), span: current.span }
        }
        _ => {
            state.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: vec![TokenKind::Variable],
                    found: *current,
                },
                Severity::Error,
                current.span,
            );

            SimpleVariable::missing(current.span)
        }
    }
}

pub fn dynamic_variable(state: &mut State) -> Variable {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Variable => {
            state.stream.next();

            Variable::SimpleVariable(SimpleVariable { symbol: current.symbol.unwrap(), span: current.span })
        }
        TokenKind::DollarLeftBrace => {
            let start = current.span;
            state.stream.next();

            let expr = expressions::create(state);

            let end = utils::skip_right_brace(state);

            Variable::BracedVariableVariable(BracedVariableVariable {
                start,
                variable: Box::new(expr),
                end,
            })
        }
        TokenKind::Dollar if state.stream.peek().kind == TokenKind::LeftBrace => {
            let start = current.span;
            state.stream.next();
            state.stream.next();

            let expr = expressions::create(state);

            let end = utils::skip_right_brace(state);

            Variable::BracedVariableVariable(BracedVariableVariable {
                start,
                variable: Box::new(expr),
                end,
            })
        }
        TokenKind::Dollar => {
            let span = current.span;
            state.stream.next();

            match state.stream.current().kind {
                TokenKind::Dollar | TokenKind::Variable => {
                    let variable = dynamic_variable(state);

                    Variable::VariableVariable(VariableVariable {
                        span,
                        variable: Box::new(variable),
                    })
                }
                // This allows us to handle standalone $ tokens, i.e. incomplete variables.
                _ => {
                    state.diagnostic(
                        ParserDiagnostic::ExpectedToken {
                            expected: vec![TokenKind::Variable],
                            found: *current,
                        },
                        Severity::Error,
                        current.span,
                    );

                    Variable::SimpleVariable(SimpleVariable::missing(current.span))
                }
            }
        }
        _ => {
            state.diagnostic(
                ParserDiagnostic::ExpectedToken {
                    expected: vec![TokenKind::Variable],
                    found: *current,
                },
                Severity::Error,
                current.span,
            );

            Variable::SimpleVariable(SimpleVariable::missing(current.span))
        }
    }
}
