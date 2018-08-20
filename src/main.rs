extern crate libusb;
extern crate ptp;

use std::fs::File;
use std::io::prelude::*;

const GOPRO_VENDOR: u16 = 0x2672;
const HERO4_SILVER: u16 = 0x000d;

fn main() {
    let context = libusb::Context::new().unwrap();

    for mut device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if device_desc.vendor_id() == GOPRO_VENDOR &&
            device_desc.product_id() == HERO4_SILVER {
            println!("Found a hero4");

            println!("configuration: {:#?}", device.active_config_descriptor());
            println!("speed: {:?}", device.speed());

            println!("Trying to stand up a ptp connection");

            let mut camera = ptp::PtpCamera::new(&device).expect("Can't open camera");
            let info = camera.get_device_info(None).expect("Couldn't get info");
            println!("Camera: {:#?}", info);
            println!("operations:");
            for op in info.OperationsSupported {
                println!("  {:x}", op);
            }

            println!("Opening session");
            camera.open_session(None).expect("Couldn't open session");

            let storage_ids = camera.get_storageids(None).expect("Couldn't get storage ids");
            println!("Storage_ids: {:#?}", storage_ids);

            let storage_info = camera.get_storage_info(storage_ids[0], None).expect("Couldn't get storage info");
            println!("Storage info: {:#?}", storage_info);

            let handles = camera.get_objecthandles_all(storage_ids[0], None, None).expect("Couldn't get handles");
            println!("handles: {:#?}", storage_ids);

            let object_handles_lol_maybe = camera.get_objecthandles(storage_ids[0], handles[0], None, None).expect("Couldn't get any more handles");
            println!("maybe more handles?: {:#?}", object_handles_lol_maybe);

            let object_handles_files = camera.get_objecthandles(storage_ids[0], object_handles_lol_maybe[0], None, None).expect("Couldn't get any more handles");
            println!("maybe files?: {:#?}", object_handles_files);

            let object_info = camera.get_objectinfo(object_handles_files[0], None).expect("Couldn't get object_info");
            println!("first object info: {:#?}", object_info);

            let object = camera.get_object(object_handles_files[0], None).expect("Couldn't get object");

            let mut file = File::create("/tmp/video.mp4").expect("Couldn't create file");
            file.write_all(object.as_slice()).expect("Couldn't write file");

            camera.close_session(None).expect("Couldn't close session");
            camera.disconnect(None).expect("Couldn't disconnect");
        }
    }
}
