use crate::lease::LeaseManager;

pub fn revoke_expired_leases(manager: &mut LeaseManager, current_tick: u64) {
    manager.revoke_expired(current_tick);
}
