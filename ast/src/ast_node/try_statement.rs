use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct TryStatement {}

impl From<TryStatement> for AstNode {
    fn from(node: TryStatement) -> Self {
        AstNode::TryStatement(node)
    }
}

impl TryStatement {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse TryStatement")
    }
}
