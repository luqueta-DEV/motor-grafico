extern crate openvr;

use openvr::Context;
use openvr::TrankedDevicePose_t;

fn setup_vr() -> Result<(), String> {
let vr = Context::init()?;
let poses: vec<TrankedDevicePose_t> = vr.get_device_poses();
ok(())

}
