use crate::error::ParseResult;
use crate::expect_token;
use crate::expected_token_err;
use crate::expressions::create;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use pxp_ast::comments::CommentGroup;
use pxp_ast::identifiers::Identifier;
use pxp_ast::literals::Literal;
use pxp_ast::literals::LiteralInteger;
use pxp_ast::literals::LiteralString;
use pxp_ast::literals::LiteralStringKind;
use pxp_ast::operators::ArithmeticOperationExpression;
use pxp_ast::variables::Variable;
use pxp_ast::Expression;
use pxp_ast::ExpressionStringPart;
use pxp_ast::LiteralStringPart;
use pxp_ast::StringPart;
use pxp_ast::{
    ArrayIndexExpression, ExpressionKind, HeredocExpression, InterpolatedStringExpression,
    NowdocExpression, NullsafePropertyFetchExpression, PropertyFetchExpression,
    ShellExecExpression,
};
use pxp_lexer::error::SyntaxError;
use pxp_span::Span;
use pxp_token::DocStringIndentationKind;
use pxp_token::TokenKind;

#[inline(always)]
pub fn interpolated(state: &mut State) -> ParseResult<Expression> {
    let start_span = state.stream.current().span;
    let mut parts = Vec::new();

    while state.stream.current().kind != TokenKind::DoubleQuote {
        if let Some(part) = part(state)? {
            parts.push(part);
        }
    }

    state.stream.next();

    let end_span = state.stream.current().span;

    Ok(Expression::new(
        ExpressionKind::InterpolatedString(InterpolatedStringExpression { parts }),
        Span::new(start_span.start, end_span.end),
        CommentGroup::default(),
    ))
}

#[inline(always)]
pub fn shell_exec(state: &mut State) -> ParseResult<Expression> {
    let start_span = state.stream.current().span;
    state.stream.next();

    let mut parts = Vec::new();

    while state.stream.current().kind != TokenKind::Backtick {
        if let Some(part) = part(state)? {
            parts.push(part);
        }
    }

    state.stream.next();

    let end_span = state.stream.current().span;

    Ok(Expression::new(
        ExpressionKind::ShellExec(ShellExecExpression { parts }),
        Span::new(start_span.start, end_span.end),
        CommentGroup::default(),
    ))
}

#[inline(always)]
pub fn heredoc(state: &mut State) -> ParseResult<Expression> {
    let span = state.stream.current().span;
    let label = state.stream.current().value.clone();
    state.stream.next();

    let mut parts = Vec::new();

    while !matches!(state.stream.current().kind, TokenKind::EndDocString(_, _)) {
        if let Some(part) = part(state)? {
            parts.push(part);
        }
    }

    let (indentation_type, indentation_amount) = match &state.stream.current().kind {
        TokenKind::EndDocString(indentation_type, indentation_amount) => {
            (indentation_type.clone(), *indentation_amount)
        }
        _ => unreachable!(),
    };

    state.stream.next();

    let mut new_line = true;
    if indentation_type != DocStringIndentationKind::None {
        let indentation_char: u8 = indentation_type.into();

        for part in parts.iter_mut() {
            // We only need to strip and validate indentation
            // for individual lines, so we can skip checks if
            // we know we're not on a new line.
            if !new_line {
                continue;
            }

            match part {
                StringPart::Literal(LiteralStringPart { value: bytes }) => {
                    // 1. If this line doesn't start with any whitespace,
                    //    we can return an error early because we know
                    //    the label was indented.
                    if !bytes.starts_with(&[b' ']) && !bytes.starts_with(&[b'\t']) {
                        return Err(SyntaxError::InvalidDocBodyIndentationLevel(
                            indentation_amount,
                            span,
                        )
                        .into());
                    }

                    // 2. If this line doesn't start with the correct
                    //    type of whitespace, we can also return an error.
                    if !bytes.starts_with(&[indentation_char]) {
                        return Err(SyntaxError::InvalidDocIndentation(span).into());
                    }

                    // 3. We now know that the whitespace at the start of
                    //    this line is correct, so we need to check that the
                    //    amount of whitespace is correct too. In this case,
                    //    the amount of whitespace just needs to be at least
                    //    the same, so we can create a vector containing the
                    //    minimum and check using `starts_with()`.
                    let expected_whitespace_buffer = vec![indentation_char; indentation_amount];
                    if !bytes.starts_with(&expected_whitespace_buffer) {
                        return Err(SyntaxError::InvalidDocBodyIndentationLevel(
                            indentation_amount,
                            span,
                        )
                        .into());
                    }

                    // 4. All of the above checks have passed, so we know
                    //    there are no more possible errors. Let's now
                    //    strip the leading whitespace accordingly.
                    *bytes = bytes
                        .strip_prefix(&expected_whitespace_buffer[..])
                        .unwrap()
                        .into();
                    new_line = bytes.ends_with(&[b'\n']);
                }
                _ => continue,
            }
        }
    }

    let end_span = state.stream.previous().span;

    Ok(Expression::new(
        ExpressionKind::Heredoc(HeredocExpression { label, parts }),
        Span::new(span.start, end_span.end),
        CommentGroup::default(),
    ))
}

#[inline(always)]
pub fn nowdoc(state: &mut State) -> ParseResult<Expression> {
    let span = state.stream.current().span;
    let label = state.stream.current().value.clone();

    state.stream.next();

    let mut string_part = state.stream.current().value.clone();
    expect_token!([TokenKind::StringPart => ()], state, "constant string");

    let (indentation_type, indentation_amount) = match &state.stream.current().kind {
        TokenKind::EndDocString(indentation_type, indentation_amount) => {
            (indentation_type.clone(), *indentation_amount)
        }
        _ => unreachable!(),
    };

    state.stream.next();

    if indentation_type != DocStringIndentationKind::None {
        let indentation_char: u8 = indentation_type.into();

        let mut lines = string_part
            .split(|b| *b == b'\n')
            .map(|s| s.to_vec())
            .collect::<Vec<Vec<u8>>>();

        for line in lines.iter_mut() {
            if line.is_empty() {
                continue;
            }

            // 1. If this line doesn't start with any whitespace,
            //    we can return an error early because we know
            //    the label was indented.
            if !line.starts_with(&[b' ']) && !line.starts_with(&[b'\t']) {
                return Err(
                    SyntaxError::InvalidDocBodyIndentationLevel(indentation_amount, span).into(),
                );
            }

            // 2. If this line doesn't start with the correct
            //    type of whitespace, we can also return an error.
            if !line.starts_with(&[indentation_char]) {
                return Err(SyntaxError::InvalidDocIndentation(span).into());
            }

            // 3. We now know that the whitespace at the start of
            //    this line is correct, so we need to check that the
            //    amount of whitespace is correct too. In this case,
            //    the amount of whitespace just needs to be at least
            //    the same, so we can create a vector containing the
            //    minimum and check using `starts_with()`.
            let expected_whitespace_buffer = vec![indentation_char; indentation_amount];
            if !line.starts_with(&expected_whitespace_buffer) {
                return Err(
                    SyntaxError::InvalidDocBodyIndentationLevel(indentation_amount, span).into(),
                );
            }

            // 4. All of the above checks have passed, so we know
            //    there are no more possible errors. Let's now
            //    strip the leading whitespace accordingly.
            *line = line
                .strip_prefix(&expected_whitespace_buffer[..])
                .unwrap()
                .into();
        }

        let mut bytes = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            bytes.extend(line);
            if i < lines.len() - 1 {
                bytes.push(b'\n');
            }
        }
        string_part = bytes.into();
    }

    let end_span = state.stream.previous().span;

    Ok(Expression::new(
        ExpressionKind::Nowdoc(NowdocExpression {
            label,
            value: string_part,
        }),
        Span::new(span.start, end_span.end),
        CommentGroup::default(),
    ))
}

fn part(state: &mut State) -> ParseResult<Option<StringPart>> {
    Ok(match &state.stream.current().kind {
        TokenKind::StringPart => {
            let s = state.stream.current().value.clone();
            let part = if s.len() > 0 {
                Some(StringPart::Literal(LiteralStringPart { value: s }))
            } else {
                None
            };

            state.stream.next();
            part
        }
        TokenKind::DollarLeftBrace => {
            let start_span = state.stream.current().span;
            let variable = variables::dynamic_variable(state)?;
            let expression = Expression::new(
                ExpressionKind::Variable(variable),
                Span::new(start_span.start, state.stream.previous().span.end),
                CommentGroup::default(),
            );

            Some(StringPart::Expression(ExpressionStringPart {
                expression: Box::new(expression),
            }))
        }
        TokenKind::LeftBrace => {
            // "{$expr}"
            state.stream.next();
            let e = create(state)?;
            utils::skip_right_brace(state)?;
            Some(StringPart::Expression(ExpressionStringPart {
                expression: Box::new(e),
            }))
        }
        TokenKind::Variable => {
            // "$expr", "$expr[0]", "$expr[name]", "$expr->a"
            let variable_span = state.stream.current().span;
            let variable = ExpressionKind::Variable(variables::dynamic_variable(state)?);
            let variable = Expression::new(variable, variable_span, CommentGroup::default());

            let current = state.stream.current();
            let e = match &current.kind {
                TokenKind::LeftBracket => {
                    let left_bracket = utils::skip_left_bracket(state)?;

                    let current = state.stream.current();
                    let index_start_span = current.span;
                    // Full expression syntax is not allowed here,
                    // so we can't call expression.
                    let index = match &current.kind {
                        TokenKind::LiteralInteger => {
                            state.stream.next();

                            ExpressionKind::Literal(Literal::Integer(LiteralInteger {
                                span: current.span,
                                value: current.value.clone(),
                            }))
                        }
                        TokenKind::Minus => {
                            state.stream.next();
                            let literal = state.stream.current();
                            if let TokenKind::LiteralInteger = &literal.kind {
                                let span = state.stream.current().span;
                                state.stream.next();
                                let kind =
                                    ExpressionKind::Literal(Literal::Integer(LiteralInteger {
                                        span: literal.span,
                                        value: literal.value.clone(),
                                    }));
                                let expression =
                                    Expression::new(kind, span, CommentGroup::default());

                                ExpressionKind::ArithmeticOperation(
                                    ArithmeticOperationExpression::Negative {
                                        minus: span,
                                        right: Box::new(expression),
                                    },
                                )
                            } else {
                                return expected_token_err!("an integer", state);
                            }
                        }
                        TokenKind::Identifier => {
                            state.stream.next();

                            ExpressionKind::Literal(Literal::String(LiteralString {
                                span: current.span,
                                value: current.value.clone(),
                                kind: LiteralStringKind::SingleQuoted,
                            }))
                        }
                        TokenKind::Variable => ExpressionKind::Variable(Variable::SimpleVariable(
                            variables::simple_variable(state)?,
                        )),
                        _ => {
                            return expected_token_err!(
                                ["`-`", "an integer", "an identifier", "a variable"],
                                state
                            );
                        }
                    };
                    let index_end_span = state.stream.previous().span;
                    let index = Expression::new(
                        index,
                        Span::new(index_start_span.start, index_end_span.end),
                        CommentGroup::default(),
                    );

                    let right_bracket = utils::skip_right_bracket(state)?;

                    ExpressionKind::ArrayIndex(ArrayIndexExpression {
                        array: Box::new(variable),
                        left_bracket,
                        index: Some(Box::new(index)),
                        right_bracket,
                    })
                }
                TokenKind::Arrow => {
                    let span = current.span;

                    state.stream.next();

                    let identifier = identifiers::identifier_maybe_reserved(state)?;
                    let id_span = identifier.span;
                    let kind = ExpressionKind::Identifier(Identifier::SimpleIdentifier(identifier));
                    let identifier_expression =
                        Expression::new(kind, id_span, CommentGroup::default());

                    ExpressionKind::PropertyFetch(PropertyFetchExpression {
                        target: Box::new(variable),
                        arrow: span,
                        property: Box::new(identifier_expression),
                    })
                }
                TokenKind::QuestionArrow => {
                    let span = current.span;
                    state.stream.next();

                    let ident = identifiers::identifier_maybe_reserved(state)?;
                    let kind = ExpressionKind::Identifier(Identifier::SimpleIdentifier(ident));

                    ExpressionKind::NullsafePropertyFetch(NullsafePropertyFetchExpression {
                        target: Box::new(variable),
                        question_arrow: span,
                        property: Box::new(Expression::new(kind, span, CommentGroup::default())),
                    })
                }
                // FIXME: This is hacky and bad for performance & memory, but works for now.
                _ => variable.kind.clone(),
            };

            Some(StringPart::Expression(ExpressionStringPart {
                expression: Box::new(Expression::new(
                    e,
                    Span::new(variable_span.start, state.stream.previous().span.end),
                    CommentGroup::default(),
                )),
            }))
        }
        _ => {
            return expected_token_err!(["`${`", "`{$", "`\"`", "a variable"], state);
        }
    })
}