use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorDefinition {}

impl From<ErrorDefinition> for AstNode {
    fn from(node: ErrorDefinition) -> Self {
        AstNode::ErrorDefinition(node)
    }
}

impl ErrorDefinition {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse ErrorDefinition")
    }
}
