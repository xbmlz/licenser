use machineid_rs::{Encryption, HWIDComponent, IdBuilder};

const SECRET_KEY: &str = "licenser_secret_key";

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
    let mut builder = IdBuilder::new(Encryption::SHA256);
    builder
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::MacAddress)
        .add_component(HWIDComponent::CPUID);

    builder
        .build(SECRET_KEY)
        .unwrap_or_else(|_| "UNKNOWN".to_string())
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