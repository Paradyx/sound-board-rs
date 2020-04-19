use portmidi::DeviceInfo;

pub fn find_device_with_name(devices: &Vec<DeviceInfo>, name: &str) -> (Option<i32>, Option<i32>) {
    let mut rx_id = None;
    let mut tx_id = None;

    for device in devices {
        if device.name().contains(name) && device.is_input() {
            rx_id = Some(device.id());
        }
        if device.name().contains(name) && device.is_output() {
            tx_id = Some(device.id());
        }
    }
    return (rx_id, tx_id);
}
