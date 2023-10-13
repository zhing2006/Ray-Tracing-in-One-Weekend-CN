use std::vec::Vec;

use clap::{arg, Command};

use with_oidn::oidn::*;

fn cli() -> Command {
  Command::new("denoiser")
    .about("Load images and denoise them.")
    .arg_required_else_help(true)
    .arg(arg!(-i --input <INPUT_FILE> "Input file name.").required(true))
    .arg(arg!(-a --albedo <ALBEDO_FILE> "Albedo file name.").required(true))
    .arg(arg!(-n --normal <NORMAL_FILE> "Normal file name.").required(true))
    .arg(arg!(-o --output <OUTPUT_FILE> "Output file name.").required(true))
}

fn main() {
  let matches = cli().get_matches();
  let input_file = matches.get_one::<String>("input").expect("Failed to get input file name.");
  let albedo_file = matches.get_one::<String>("albedo").expect("Failed to get albedo file name.");
  let normal_file = matches.get_one::<String>("normal").expect("Failed to get normal file name.");
  let output_file = matches.get_one::<String>("output").expect("Failed to get output file name.");

  let mut input_file = std::fs::OpenOptions::new()
    .read(true)
    .open(input_file)
    .expect("Failed to open input file.");
  let mut albedo_file = std::fs::OpenOptions::new()
    .read(true)
    .open(albedo_file)
    .expect("Failed to open albedo file.");
  let mut normal_file = std::fs::OpenOptions::new()
    .read(true)
    .open(normal_file)
    .expect("Failed to open normal file.");

  let input = pxm::PFM::read_from(&mut input_file).expect("Failed to read input file.");
  let albedo = pxm::PFM::read_from(&mut albedo_file).expect("Failed to read albedo file.");
  let mut normal = pxm::PFM::read_from(&mut normal_file).expect("Failed to read normal file.");

  // Remap normal values from [0, 1] to [-1, 1].
  normal.data.iter_mut().for_each(|value| *value = *value * 2.0 - 1.0);

  let mut output = Vec::<f32>::with_capacity(input.width * input.height * 3);

  unsafe {
    let device = oidnNewDevice(OIDNDeviceType_OIDN_DEVICE_TYPE_CPU);
    oidnCommitDevice(device);

    let color_buf = oidnNewBuffer(device, input.width * input.height * 3 * std::mem::size_of::<f32>());
    let albedo_buf = oidnNewBuffer(device, albedo.width * input.height * 3 * std::mem::size_of::<f32>());
    let normal_buf = oidnNewBuffer(device, normal.width * input.height * 3 * std::mem::size_of::<f32>());

    let color_ptr: *mut f32 = oidnGetBufferData(color_buf) as *mut f32;
    let albedo_ptr: *mut f32 = oidnGetBufferData(albedo_buf) as *mut f32;
    let normal_ptr: *mut f32 = oidnGetBufferData(normal_buf) as *mut f32;
    for y in 0..input.height {
      for x in 0..input.width {
        for c in 0..3 {
          *color_ptr.add((y * input.width + x) * 3 + c) = input.data[(y * input.width + x) * 3 + c];
          *albedo_ptr.add((y * input.width + x) * 3 + c) = albedo.data[(y * input.width + x) * 3 + c];
          *normal_ptr.add((y * input.width + x) * 3 + c) = normal.data[(y * input.width + x) * 3 + c];
        }
      }
    }

    let type_ = std::ffi::CString::new("RT").unwrap();
    let filter = oidnNewFilter(device, type_.as_ptr()); // generic ray tracing filter
    let color_ = std::ffi::CString::new("color").unwrap();
    oidnSetFilterImage(filter, color_.as_ptr(), color_buf, OIDNFormat_OIDN_FORMAT_FLOAT3, input.width, input.height, 0, 0, 0);
    let albedo_ = std::ffi::CString::new("albedo").unwrap();
    oidnSetFilterImage(filter, albedo_.as_ptr(), albedo_buf, OIDNFormat_OIDN_FORMAT_FLOAT3, albedo.width, albedo.height, 0, 0, 0);
    let normal_ = std::ffi::CString::new("normal").unwrap();
    oidnSetFilterImage(filter, normal_.as_ptr(), normal_buf, OIDNFormat_OIDN_FORMAT_FLOAT3, normal.width, normal.height, 0, 0, 0);
    let output_ = std::ffi::CString::new("output").unwrap();
    oidnSetFilterImage(filter, output_.as_ptr(), color_buf, OIDNFormat_OIDN_FORMAT_FLOAT3, input.width, input.height, 0, 0, 0);
    let hdr_ = std::ffi::CString::new("hdr").unwrap();
    oidnSetFilterBool(filter, hdr_.as_ptr(), true);
    oidnCommitFilter(filter);

    oidnExecuteFilter(filter);
    let mut error_msg: *const std::os::raw::c_char = std::ptr::null();
    if oidnGetDeviceError(device, &mut error_msg) != OIDNError_OIDN_ERROR_NONE {
      println!("Error: {}", std::ffi::CStr::from_ptr(error_msg).to_str().unwrap());
    } else {
      let color_ptr: *mut f32 = oidnGetBufferData(color_buf) as *mut f32;
      for y in 0..input.height {
        for x in 0..input.width {
          for c in 0..3 {
            output.push(*color_ptr.add((y * input.width + x) * 3 + c));
          }
        }
      }
    }

    oidnReleaseBuffer(color_buf);
    oidnReleaseBuffer(albedo_buf);
    oidnReleaseBuffer(normal_buf);
    oidnReleaseDevice(device);
  }

  if !output.is_empty() {
    println!("Writing output file.");
    let output = pxm::PFMBuilder::new()
      .size(input.width, input.height)
      .color(input.color)
      .scale(input.scale_factor)
      .data(output)
      .build()
      .expect("Failed to build output image.");

    let mut output_file = std::fs::File::create(output_file).expect("Failed to create output file.");
    output.write_into(&mut output_file).expect("Failed to write output file.");
  }
}