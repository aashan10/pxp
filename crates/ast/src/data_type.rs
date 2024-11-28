use pxp_span::Span;
use pxp_type::Type;

use crate::{DataType, Name, NodeId};

impl DataType {
    pub fn new(id: NodeId, kind: Type<Name>, span: Span) -> Self {
        Self { id, kind, span }
    }

    pub fn get_type(&self) -> &Type<Name> {
        &self.kind
    }

    pub fn get_span(&self) -> Span {
        self.span
    }

    pub fn standalone(&self) -> bool {
        self.kind.standalone()
    }

    pub fn nullable(&self) -> bool {
        self.kind.nullable()
    }

    pub fn includes_callable(&self) -> bool {
        self.kind.includes_callable()
    }

    pub fn is_bottom(&self) -> bool {
        self.kind.is_bottom()
    }
}
