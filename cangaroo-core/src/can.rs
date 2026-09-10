use socketcan::enumerate::available_interfaces;

pub fn list_ifaces() -> Result<Vec<String>, socketcan::Error> {
    available_interfaces()
}