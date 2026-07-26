use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CapabilityKey {
    pub(crate) domain: &'static str,
    pub(crate) opcode: i32,
    pub(crate) type_name: String,
}

impl CapabilityKey {
    pub(crate) fn new(domain: &'static str, opcode: i32, type_name: impl Into<String>) -> Self {
        Self {
            domain,
            opcode,
            type_name: type_name.into(),
        }
    }
}

#[derive(Default)]
pub(crate) struct Report {
    pub(crate) errors: BTreeSet<String>,
    pub(crate) warnings: BTreeSet<String>,
    pub(crate) checked_skills: HashSet<i32>,
    pub(crate) checked_buffs: HashSet<i32>,
    pub(crate) capabilities: BTreeSet<CapabilityKey>,
    pub(crate) gaps: BTreeMap<CapabilityKey, BTreeSet<&'static str>>,
    pub(crate) gap_paths: BTreeMap<CapabilityKey, BTreeSet<String>>,
    pub(crate) explanations: Vec<String>,
    pub(crate) quiet: bool,
    pub(crate) explain: bool,
    pub(crate) wire_evidence: crate::wire_evidence::Evidence,
}

impl Report {
    pub(crate) fn is_ready(&self) -> bool {
        self.errors.is_empty() && self.gaps.is_empty()
    }

    pub(super) fn error(&mut self, message: impl Into<String>) {
        if !self.quiet {
            self.errors.insert(message.into());
        }
    }

    pub(super) fn warning(&mut self, message: impl Into<String>) {
        if !self.quiet {
            self.warnings.insert(message.into());
        }
    }

    pub(super) fn capability(&mut self, key: CapabilityKey) {
        self.capabilities.insert(key);
    }

    pub(super) fn gap(&mut self, key: CapabilityKey, reason: &'static str) {
        self.capabilities.insert(key.clone());
        self.gaps.entry(key).or_default().insert(reason);
    }

    pub(super) fn gap_at(&mut self, key: CapabilityKey, reason: &'static str, path: String) {
        self.gap(key.clone(), reason);
        self.gap_paths.entry(key).or_default().insert(path);
    }

    pub(super) fn explain(&mut self, message: impl Into<String>) {
        if self.explain {
            let message = message.into();
            if !self.explanations.contains(&message) {
                self.explanations.push(message);
            }
        }
    }

    pub(crate) fn print(&self) {
        for error in &self.errors {
            println!("ERROR {error}");
        }
        for warning in &self.warnings {
            println!("WARN  {warning}");
        }
        for explanation in &self.explanations {
            println!("INFO  {explanation}");
        }
        println!(
            "{} skills={} buffs={} errors={} warnings={} gaps={}",
            if self.is_ready() {
                "READY"
            } else {
                "INCOMPLETE"
            },
            self.checked_skills.len(),
            self.checked_buffs.len(),
            self.errors.len(),
            self.warnings.len(),
            self.gaps.len(),
        );
    }
}
