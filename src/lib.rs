
use std::fmt;
use std::marker::PhantomData;

/// type used for USB identifiers
type UsbIdType = u16;
const PIXY_PRODUCT_ID: UsbIdType = 0xF000;
const PIXY_VENDOR_ID: UsbIdType = 0xB1AC;

//Note: DFU is for "device firmware update" (bootloader) mode
//const PIXY_DFU_PRODUCT_ID: UsbIdType = 0x000C;
//const PIXY_DFU_VENDOR_ID: UsbIdType = 0x1FC9;


#[derive(Clone)]
pub struct PixyError {
  code: usize,
  message: String,
}

impl fmt::Display for PixyError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "An Error Occurred, Please Try Again!") // user-facing output
  }
}

impl fmt::Debug for PixyError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
  }
}

type Result<T> = std::result::Result<T, PixyError>;


pub struct PixyContext<'a> {
  device: libusb::Device<'a>,
  device_desc: libusb::DeviceDescriptor,
  device_handle: libusb::DeviceHandle<'a>,
}

fn open_pixy2_device() ->  Option<PixyContext> {
  if let Ok(ctx) = libusb::Context::new() {
    if let Ok(device_list) = ctx.devices() {
      for device in device_list.iter() {
        if let Ok(desc) = device.device_descriptor() {
          if desc.vendor_id() == PIXY_VENDOR_ID && desc.product_id() == PIXY_PRODUCT_ID {
            if let Ok(handle) = device.open() {
              return Some(
                PixyContext {
                  device: device,
                  device_desc: desc,
                  device_handle: handle,
                }
              )
            }
          }
        }
      }
    }
  }

  None
}

pub fn init_device() -> Result<PixyContext>  {
  if let Some(pixy_context) =  open_pixy2_device() {
      println!("found the pixy device");
      return Ok(pixy_context)
  }

  Err(
  PixyError {
    code: 0,
    message: "Could not open Pixy2".to_string()
  })

}
