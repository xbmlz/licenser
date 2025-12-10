use rs_machineid::{MachineId};

/**
 * Get the machine ID.
 *
 * The machine ID is a unique identifier for the machine.
 *
 * # Returns
 *
 * The machine ID as a string.
 */
pub fn get_machine_id() -> String {
    MachineId::get().unwrap_or_else(|_| "UNKNOWN".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_machine_id() {
        let machine_id = get_machine_id();
        assert_ne!(machine_id, "UNKNOWN");
        // The machine ID should be the same for each call.
        let machine_id2 = get_machine_id();
        assert_ne!(machine_id2, "UNKNOWN");
        assert_eq!(machine_id, machine_id2);
    }
}