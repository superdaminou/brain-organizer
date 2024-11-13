use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize,Debug, PartialEq, Eq, Clone, Default, Hash, PartialOrd, Ord)]
pub struct Tag(pub String);
