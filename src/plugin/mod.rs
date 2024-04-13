use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct PluginId(String);

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct SystemId(String);

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ResourceId(String);

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ComponentId(String);

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AssetId(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Plugin {
    name: String,
    description: String,
    author: String,
    // TODO: Do these need to store any meta-info? Probably, but later problem.
    plugins: HashSet<PluginId>,
    systems: HashSet<SystemId>,
    resources: HashSet<ResourceId>,
    components: HashSet<ComponentId>,
    assets: HashSet<AssetId>,
}
