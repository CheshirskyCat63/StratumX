use engine_core::{ComponentTypeId, EngineCoreError, EngineCoreResult};
use engine_identity::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryModel {
    entity_set: BTreeSet<EntityId>,
    component_classes: BTreeSet<ComponentTypeId>,
    presence_map: BTreeMap<EntityId, BTreeSet<ComponentTypeId>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MembershipDescriptor {
    pub entity: EntityId,
    pub components: Vec<ComponentTypeId>,
}

impl Default for RegistryModel {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryModel {
    pub fn new() -> Self {
        Self {
            entity_set: BTreeSet::new(),
            component_classes: BTreeSet::new(),
            presence_map: BTreeMap::new(),
        }
    }

    pub fn register_entity(&mut self, entity: EntityId) {
        self.entity_set.insert(entity);
        self.presence_map.entry(entity).or_default();
    }

    pub fn register_component_class(&mut self, component: ComponentTypeId) {
        self.component_classes.insert(component);
    }

    pub fn attach_component(
        &mut self,
        entity: EntityId,
        component: ComponentTypeId,
    ) -> EngineCoreResult<()> {
        if !self.entity_set.contains(&entity) {
            return Err(EngineCoreError::InvalidDescriptor(
                "entity must be registered before component attach",
            ));
        }
        if !self.component_classes.contains(&component) {
            return Err(EngineCoreError::InvalidDescriptor(
                "component class must be registered before attach",
            ));
        }
        self.presence_map
            .entry(entity)
            .or_default()
            .insert(component);
        Ok(())
    }

    pub fn detach_component(&mut self, entity: EntityId, component: ComponentTypeId) -> bool {
        self.presence_map
            .get_mut(&entity)
            .map(|set| set.remove(&component))
            .unwrap_or(false)
    }

    pub fn has_component(&self, entity: EntityId, component: ComponentTypeId) -> bool {
        self.presence_map
            .get(&entity)
            .map(|set| set.contains(&component))
            .unwrap_or(false)
    }

    pub fn membership(&self, entity: EntityId) -> MembershipDescriptor {
        let components = self
            .presence_map
            .get(&entity)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default();

        MembershipDescriptor { entity, components }
    }

    pub fn entities(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.entity_set.iter().copied()
    }

    pub fn members_with_component(
        &self,
        component: ComponentTypeId,
    ) -> impl Iterator<Item = EntityId> + '_ {
        self.presence_map.iter().filter_map(move |(entity, set)| {
            if set.contains(&component) {
                Some(*entity)
            } else {
                None
            }
        })
    }
}
