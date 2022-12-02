// use super::*;
use serde::{Serialize, Deserialize};

use crate::documents::BuildXML;
// use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McFallback {}

impl McFallback {
    pub fn new() -> McFallback {
        Default::default()
    }
}

impl Default for McFallback {
    fn default() -> Self {
        McFallback {}
    }
}

impl BuildXML for McFallback {
    fn build(&self) -> Vec<u8> {
        //  Ignore for now
        vec![]
    }
}
