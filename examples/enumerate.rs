use drm::node;

pub fn main() {
    let devices = node::drm_get_devices();

    println!("{:?}", devices.unwrap());
}