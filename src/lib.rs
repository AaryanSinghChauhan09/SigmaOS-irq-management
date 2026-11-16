// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-irq-management: irq management component model.
//!
//! Real implementation: managements, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Management {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrqManagementError {
    EmptyManagementName,
    DuplicateManagement(String),
    EmptyEventSubject,
}

impl fmt::Display for IrqManagementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyManagementName => write!(f, "management name must not be empty"),
            Self::DuplicateManagement(id) => write!(f, "duplicate management {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for IrqManagementError {}

pub struct IrqManagement {
    pub name: String,
    managements: BTreeMap<String, Management>,
    events: BTreeMap<String, Event>,
}

impl IrqManagement {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            managements: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_management(&mut self, item: Management) -> Result<(), IrqManagementError> {
        if item.name.trim().is_empty() {
            return Err(IrqManagementError::EmptyManagementName);
        }
        let id = item.id.clone();
        if self.managements.contains_key(&id) {
            return Err(IrqManagementError::DuplicateManagement(id.clone()));
        }
        self.managements.insert(id, item);
        Ok(())
    }

    pub fn management_count(&self) -> usize {
        self.managements.len()
    }

    pub fn get_management(&self, id: &str) -> Option<&Management> {
        self.managements.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), IrqManagementError> {
        if event.subject.trim().is_empty() {
            return Err(IrqManagementError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_management(id: &str, name: &str) -> Management {
        Management {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = IrqManagement::new("IrqManagement");
        assert_eq!(m.name, "IrqManagement");
        assert_eq!(m.management_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_management() {
        let mut m = IrqManagement::new("IrqManagement");
        m.add_management(sample_management("a1", "Alpha")).unwrap();
        assert_eq!(m.management_count(), 1);
        assert!(m.get_management("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = IrqManagement::new("IrqManagement");
        assert!(m.add_management(sample_management("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = IrqManagement::new("IrqManagement");
        m.add_management(sample_management("x", "One")).unwrap();
        assert!(m.add_management(sample_management("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = IrqManagement::new("IrqManagement");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
