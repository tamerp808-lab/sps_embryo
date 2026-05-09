#![allow(dead_code)]
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lease {
    pub lease_id: u64,
    pub resource_id: u64,
    pub owner_agent_id: u64,
    pub acquired_tick: u64,
    pub expires_tick: u64,
    pub active: bool,
}

#[derive(Debug)]
pub struct LeaseManager {
    leases: BTreeMap<u64, Lease>,
}

impl LeaseManager {
    pub fn new() -> Self {
        Self {
            leases: BTreeMap::new(),
        }
    }

    pub fn acquire(
        &mut self,
        lease_id: u64,
        resource_id: u64,
        owner_agent_id: u64,
        current_tick: u64,
        duration_ticks: u64,
    ) {
        let lease = Lease {
            lease_id,
            resource_id,
            owner_agent_id,
            acquired_tick: current_tick,
            expires_tick: current_tick + duration_ticks,
            active: true,
        };
        self.leases.insert(lease_id, lease);
    }

    pub fn is_owner(&self, resource_id: u64, agent_id: u64, current_tick: u64) -> bool {
        self.leases.values().any(|l| {
            l.resource_id == resource_id
                && l.owner_agent_id == agent_id
                && l.active
                && current_tick <= l.expires_tick
        })
    }

    pub fn revoke(&mut self, lease_id: u64) {
        if let Some(lease) = self.leases.get_mut(&lease_id) {
            lease.active = false;
        }
    }

    /// يُلغي كل العُقد المنتهية تلقائيًا (للحوكمة)
    pub fn revoke_expired(&mut self, current_tick: u64) {
        for lease in self.leases.values_mut() {
            if lease.active && current_tick > lease.expires_tick {
                lease.active = false;
            }
        }
    }

    pub fn total_active_leases(&self, current_tick: u64) -> usize {
        self.leases
            .values()
            .filter(|l| l.active && l.expires_tick >= current_tick)
            .count()
    }
}
