use crate::{
    lexer::{Token, TokensGroup}, parser::parser::get_identifier, types::{ASTNode, Node, Type}, ParseResult
};

use super::{
    expression::parse_expected_expression,
    parser::{expect_tokens, peek_expect_tokens},
    types::parse_type,
};

pub fn parse_variable(tokens: &mut TokensGroup) -> ParseResult<ASTNode> {
    let mutable = peek_expect_tokens(tokens, vec![Token::Mutable], true)?.is_some();
    let name = get_identifier(tokens)?;
    
    let mut data_type: Option<Type> = None;
    if peek_expect_tokens(tokens, vec![Token::Colon], true)?.is_some() {
        data_type = Some(parse_type(tokens)?);
    }

    expect_tokens(tokens, vec![Token::Equals])?;

    let expression = parse_expected_expression(tokens)?;

    return Ok(tokens.create_ast( Node::DefineVariable {
        mutable,
        name,
        data_type,
        expression: Some(expression),
    }));
}
