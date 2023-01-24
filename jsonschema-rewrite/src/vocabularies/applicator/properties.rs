use crate::{schema::edges::EdgeLabel, vocabularies::Keyword, Schema};
use serde_json::Value;
use std::ops::Range;

#[derive(Debug, Eq, PartialEq)]
pub struct Properties {
    pub(crate) edges: Range<usize>,
}

impl Properties {
    pub(crate) fn build(edges: Range<usize>) -> Keyword {
        Self { edges }.into()
    }
}

impl Properties {
    pub(crate) fn is_valid(&self, schema: &Schema, instance: &Value) -> bool {
        if let Value::Object(item) = instance {
            schema.edges[self.edges.clone()].iter().all(|edge| {
                // TODO. split edges to String / usize to avoid `match` on each one
                if let Some(value) = match &edge.label {
                    EdgeLabel::Key(key) => item.get(&**key),
                    EdgeLabel::Index(_) => None,
                } {
                    schema.keywords[edge.keywords.clone()]
                        .iter()
                        .all(|k| k.is_valid(schema, value))
                } else {
                    true
                }
            })
        } else {
            true
        }
    }
}