#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use std::hash::Hash;
use std::io::Write;

pub use doc::Document;
pub use node::Node;
pub use nodes::Nodes;
pub use ord_map::OrdMap;

use crate::error::Result;

#[macro_use]
pub mod error;

mod doc;
mod node;
mod nodes;
mod ord_map;

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Name {
    pub namespace: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PIData {}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Attribute {
    key: String,
    value: String,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElementData {
    pub namespace: Option<String>,
    pub name: String,
    pub attributes: OrdMap,
    pub nodes: Vec<Node>,
}

const SMALLEST_ELEMENT: usize = 4; // <x/>

impl ElementData {
    fn check(&self) -> Result<()> {
        if self.name.is_empty() {
            return raise!("Empty element name.");
        }
        if let Some(ns) = &self.namespace {
            return raise!("Namespace should not be empty when the option is 'some'.");
        }
        for attribute_key in self.attributes.map().keys() {
            if attribute_key.is_empty() {
                return raise!("Empty attribute name encountered.");
            }
        }
        Ok(())
    }

    pub fn to_writer<W>(&self, writer: &mut W) -> Result<()>
        where W: Write, {
        if let Err(e) = self.check() {
            return wrap!(e);
        }
        if let Err(e) = write!(writer, "<") {
            return wrap!(e);
        }
        if let Some(ns) = &self.namespace {
            if !ns.is_empty() {
                if let Err(e) = write!(writer, "{}:", ns) {
                    return wrap!(e);
                }
            }
        }
        if let Err(e) = write!(writer, "{}", self.name) {
            return wrap!(e);
        }
        // TODO - attributes
        if self.nodes.is_empty() {
            if let Err(e) = write!(writer, "/>") {
                return wrap!(e);
            } else {
                return Ok(());
            }
        } else {
            if let Err(e) = write!(writer, ">") {
                return wrap!(e);
            }
        }

        for node in self.nodes.iter() {
            if let Err(e) = node.write(writer) {
                // TODO - this may explode with recursive wrapping
                return wrap!(e);
            }
        }

        // Closing Tag
        if let Err(e) = write!(writer, "</") {
            return wrap!(e);
        }
        if let Some(ns) = &self.namespace {
            if !ns.is_empty() {
                if let Err(e) = write!(writer, "{}:", ns) {
                    return wrap!(e);
                }
            }
        }
        if let Err(e) = write!(writer, "{}", self.name) {
            return wrap!(e);
        }
        if let Err(e) = write!(writer, ">") {
            return wrap!(e);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn structs_test() {
        let mut doc = Document::new();
        doc.root = Node::Element(ElementData {
            namespace: None,
            name: "root-element".to_string(),
            attributes: Default::default(),
            nodes: vec![],
        });
        let mut c = Cursor::new(Vec::new());
        let result = doc.to_writer(&mut c);
        assert!(result.is_ok());
        let data = c.into_inner();
        let data_str = std::str::from_utf8(data.as_slice()).unwrap();
        assert_eq!("<root-element/>", data_str);
    }
}
