use super::{RuleDescriptor, RuleDomain};

pub struct RuleCatalog;

impl RuleCatalog {
    pub fn descriptors() -> impl Iterator<Item = RuleDescriptor> {
        crate::engine::skill::behavior::registry::definitions()
            .map(|definition| RuleDescriptor::new(RuleDomain::Behavior, definition.key))
            .chain(
                crate::engine::skill::condition::registry::definitions()
                    .map(|definition| RuleDescriptor::new(RuleDomain::Condition, definition.key)),
            )
            .chain(
                crate::engine::skill::buff_act::effect_time::definitions()
                    .map(|definition| RuleDescriptor::new(RuleDomain::EffectTime, definition.key)),
            )
            .chain(
                crate::engine::skill::buff_act::registry::definitions()
                    .map(|definition| RuleDescriptor::new(RuleDomain::BuffAct, definition.key)),
            )
    }

    pub fn find(domain: RuleDomain, opcode: i32, type_name: &str) -> Option<RuleDescriptor> {
        Self::descriptors().find(|descriptor| {
            descriptor.domain == domain && descriptor.key.matches(opcode, type_name)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::engine::skill::rule::DefinitionKey;

    #[test]
    fn catalog_keeps_domains_and_exact_types_distinct() {
        assert!(RuleCatalog::find(RuleDomain::Behavior, 20002, "AddExPoint").is_some());
        assert!(RuleCatalog::find(RuleDomain::Behavior, 20002, "DelExPoint").is_none());
        assert!(RuleCatalog::find(RuleDomain::Condition, 20002, "AddExPoint").is_none());
        assert!(RuleCatalog::find(RuleDomain::Condition, 210, "None").is_some());
        assert!(RuleCatalog::find(RuleDomain::BuffAct, 815, "AddSpTempCard").is_some());
        assert!(RuleCatalog::find(RuleDomain::Behavior, 815, "AddSpTempCard").is_none());

        let same_key = DefinitionKey::new(210, "None");
        let keys = HashSet::from([
            RuleDescriptor::new(RuleDomain::Behavior, same_key),
            RuleDescriptor::new(RuleDomain::Condition, same_key),
        ]);
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn registered_exact_keys_are_unique_inside_their_domain() {
        let descriptors = RuleCatalog::descriptors().collect::<Vec<_>>();
        let unique = descriptors.iter().copied().collect::<HashSet<_>>();

        assert_eq!(descriptors.len(), unique.len());
    }
}
