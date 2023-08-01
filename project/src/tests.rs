use crate::PacketMetadata;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_packet() {
        // Define test data
        let packet_data = "test_packet_data";
        // Assuming process_packet function is defined and returns PacketMetadata
        let expected_metadata = PacketMetadata {
            src_ip: None,
            dst_ip: None,
            src_port: 1234,
            dst_port: 5678,
            seq_number: 1,
            ack_number: 2,
            flags: 0,
            window_size: 4096,
        };

        // Call the function being tested
        let result = process_packet(packet_data);

        // Assert that the result is as expected
        assert_eq!(result, expected_metadata);
    }
}
