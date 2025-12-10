use rs_machineid::MachineId;

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
    let machine_id = MachineId::get_hashed("-app-id-2025-");
    if !machine_id.is_ok() {
        return "UNKNOWN".to_string();
    }
    let machine_id = machine_id.unwrap();
    // slice the machine ID to 16 characters, like XXXX-XXXX-XXXX-XXXX
    let machine_id = &machine_id[..16];
    machine_id
        .to_uppercase()
        .as_bytes()
        .chunks(4)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<&str>>()
        .join("-")
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
