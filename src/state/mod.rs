use crate::plugin;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// The exported state of an engine, as purely data.
///
/// This is enough to save and load a currently active game. A new game is created
/// by some external party creating the initial state of the game.
#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    plugins: Vec<PluginId>,
    systems: Vec<System>,
    resources: Vec<Resource>,
    entities: Vec<Entity>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct EntityId(Uuid);

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub struct PluginId(plugin::PluginId);

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub struct SystemId(plugin::PluginId, plugin::SystemId);

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub struct ComponentId(plugin::PluginId, plugin::ComponentId);

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub struct ResourceId(plugin::PluginId, plugin::ResourceId);

/// The exported state of a system.
///
/// The code of this system is located elsewhere and is instantiated with the given state
/// as part of the construction of the dispatcher for the game.
#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    id: SystemId,
    state: Value,
}

/// The exported state of an entity.
///
/// While during the game the "entity" itself is only its ID, it is saved alongside all of
/// its own data in the state format.
#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    id: EntityId,
    components: Vec<Component>,
}

/// The exported state of a component.
///
/// This component gets instantiated as part of the associated entity during game initialization.
#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    id: ComponentId,
    state: Value,
}

/// The exported state of a resource.
///
/// This resource gets instantiated during game initialization.
#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    id: ResourceId,
    state: Value,
}
