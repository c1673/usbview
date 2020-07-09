extern crate winapi;

fn main() {
	use std::ffi::OsStr;
	use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
	use std::ptr::null_mut;
	use std::mem;
		
	use winapi::shared::guiddef::GUID;
	use winapi::um::handleapi::INVALID_HANDLE_VALUE;
	
	use winapi::um::setupapi::{SetupDiGetClassDevsW, SetupDiEnumDeviceInfo, SetupDiGetDeviceRegistryPropertyW};
	
	use winapi::um::setupapi::{DIGCF_PRESENT, DIGCF_ALLCLASSES, SP_DEVINFO_DATA};
	
	use winapi::um::setupapi::{SPDRP_PHYSICAL_DEVICE_OBJECT_NAME, SPDRP_HARDWAREID, SPDRP_DRIVER, SPDRP_LOCATION_INFORMATION, SPDRP_FRIENDLYNAME, SPDRP_COMPATIBLEIDS, SPDRP_DEVICEDESC, SPDRP_ENUMERATOR_NAME};
	
	let usb: Vec<u16> = OsStr::new("USB").encode_wide().chain(once(0)).collect();

	let _ret = unsafe{
		let mut string_buff : [u8; 1024] = [0; 1024];
			
		let blank_guid = GUID{
			Data1 : 0x00000000, 
			Data2 : 0x0000,
			Data3 : 0x0000,
			Data4 : [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 
		};
				
		//1.
		//List of all USB devices that are present in the system.
		let hdevinfo_usb = SetupDiGetClassDevsW(null_mut(), usb.as_ptr(), null_mut(), DIGCF_ALLCLASSES | DIGCF_PRESENT);
		
		if hdevinfo_usb == INVALID_HANDLE_VALUE
		{
			println!("Failed!!");
			std::process::exit(INVALID_HANDLE_VALUE as i32);
		}
		
		//2.
		//Specifies device information element in device information set.
		let mut while_count = 0;
		
		let mut sp_devinfo_data_usb = SP_DEVINFO_DATA
		{
			cbSize : mem::size_of::<SP_DEVINFO_DATA>() as u32,
			ClassGuid : blank_guid,
			DevInst : 0,
			Reserved : 0, 
		};
		
		//3.List device information
		while SetupDiEnumDeviceInfo(hdevinfo_usb, while_count, &mut sp_devinfo_data_usb) != 0 {
			println!("---");
			println!("Device {} :", while_count);
			println!("GUID : {:#X?}, {:#X?}, {:#X?}, 0x{:02X?}", sp_devinfo_data_usb.ClassGuid.Data1, sp_devinfo_data_usb.ClassGuid.Data2, sp_devinfo_data_usb.ClassGuid.Data3, sp_devinfo_data_usb.ClassGuid.Data4);
			
			//3-1. Query hardware Id
			if SetupDiGetDeviceRegistryPropertyW(hdevinfo_usb, &mut sp_devinfo_data_usb, SPDRP_HARDWAREID, null_mut(), &mut string_buff[0], 1024, null_mut()) != 0
			{
				let mut hardware_id_str = String::new();
					
				for i in 0..string_buff.len()
				{
					if 0 != string_buff[i] {
						hardware_id_str.push(string_buff[i] as char);
					}
				}
				
				println!("Hardware Id : {}", hardware_id_str);
				
				hardware_id_str = hardware_id_str.replace("_", "");
				
				let vid_start_pos = hardware_id_str.find("VID");
				let pid_start_pos = hardware_id_str.find("PID");

				//println!("VID : 0x{}", hardware_id_str.slice_unchecked(vid_start_pos.expect("") + 3, vid_start_pos.expect("") + 7));
				//println!("PID : 0x{}", hardware_id_str.slice_unchecked(pid_start_pos.expect("") + 3, pid_start_pos.expect("") + 7));
				println!("VID : 0x{}", hardware_id_str.get_unchecked(vid_start_pos.expect("") + 3.. vid_start_pos.expect("") + 7));
				println!("PID : 0x{}", hardware_id_str.get_unchecked(pid_start_pos.expect("") + 3.. pid_start_pos.expect("") + 7));
			}
			else
			{
				println!("Hardware Id : Null");
			}
			
			string_buff = [0u8; 1024];			//Reset array 
			
			//3-2. Query driver key
			if SetupDiGetDeviceRegistryPropertyW(hdevinfo_usb, &mut sp_devinfo_data_usb, SPDRP_DRIVER, null_mut(), &mut string_buff[0], 1024, null_mut()) != 0
			{
				let mut driver_key_str = String::new();
			
				for i in 0..string_buff.len()
				{
					if 0 != string_buff[i] {
						driver_key_str.push(string_buff[i] as char);
					}
				}
				println!("Driver key : {}", driver_key_str);
			}
			else
			{
				println!("Driver key : Null");
			}
			
			string_buff = [0u8; 1024];
			
			//3-3. Query hardware location
			if SetupDiGetDeviceRegistryPropertyW(hdevinfo_usb, &mut sp_devinfo_data_usb, SPDRP_LOCATION_INFORMATION, null_mut(), &mut string_buff[0], 1024, null_mut()) != 0
			{
				let mut location_str = String::new();
			
				for i in 0..string_buff.len()
				{
					if 0 != string_buff[i] {
						location_str.push(string_buff[i] as char);
					}
				}
				println!("Hardware location : {}", location_str);
			}
			else
			{
				println!("Hardware location : Null");
			}
			
			string_buff = [0u8; 1024];
			
			//3-4. Query PDO
			if SetupDiGetDeviceRegistryPropertyW(hdevinfo_usb, &mut sp_devinfo_data_usb, SPDRP_PHYSICAL_DEVICE_OBJECT_NAME, null_mut(), &mut string_buff[0], 1024, null_mut()) != 0
			{
				let mut product_str = String::new();
			
				for i in 0..string_buff.len()
				{
					if 0 != string_buff[i] {
						product_str.push(string_buff[i] as char);
					}
				}
				println!("PDO : {}", product_str);
			}
			else
			{
				println!("PDO : Null");
			}
			
			//3-5. Query friendly name
			if SetupDiGetDeviceRegistryPropertyW(hdevinfo_usb, &mut sp_devinfo_data_usb, SPDRP_FRIENDLYNAME, null_mut(), &mut string_buff[0], 1024, null_mut()) != 0
			{
				let mut friendly_name_str = String::new();
			
				for i in 0..string_buff.len()
				{
					if 0 != string_buff[i]{
						friendly_name_str.push(string_buff[i] as char);
					}
				}
				println!("Friendly name : {}", friendly_name_str);
			}
			else
			{
				println!("Friendly name : Null");
			}
			
			string_buff = [0u8; 1024];
			
			//3-6. Query Compatible IDs
			if SetupDiGetDeviceRegistryPropertyW(hdevinfo_usb, &mut sp_devinfo_data_usb, SPDRP_COMPATIBLEIDS, null_mut(), &mut string_buff[0], 1024, null_mut()) != 0
			{
				let mut compatible_ids_str = String::new();
			
				for i in 0..string_buff.len()
				{
					if 0 != string_buff[i] {
						compatible_ids_str.push(string_buff[i] as char);
					}
				}
				println!("Compatible IDs : {}", compatible_ids_str);
			}
			else
			{
				println!("Compatible IDs : Null");
			}
			
			string_buff = [0u8; 1024];
			
			//3-7. Query description of a device
			if SetupDiGetDeviceRegistryPropertyW(hdevinfo_usb, &mut sp_devinfo_data_usb, SPDRP_DEVICEDESC, null_mut(), &mut string_buff[0], 1024, null_mut()) != 0
			{
				let mut compatible_ids_str = String::new();
			
				for i in 0..string_buff.len()
				{
					if 0 != string_buff[i] {
						compatible_ids_str.push(string_buff[i] as char);
					}
				}
				println!("Description : {}", compatible_ids_str);
			}
			else
			{
				println!("Description : Null");
			}
			
			string_buff = [0u8; 1024];
			
			//3-8. Query enumerator of device
			if SetupDiGetDeviceRegistryPropertyW(hdevinfo_usb, &mut sp_devinfo_data_usb, SPDRP_ENUMERATOR_NAME, null_mut(), &mut string_buff[0], 1024, null_mut()) != 0
			{
				let mut enumerator_str = String::new();
			
				for i in 0..string_buff.len()
				{
					if 0 != string_buff[i] {
						enumerator_str.push(string_buff[i] as char);
					}
				}
				println!("Enumerator : {}", enumerator_str);
			}
			else
			{
				println!("Enumerator : Null");
			}
			
			string_buff = [0u8; 1024];
			
			while_count += 1;
		}
	};
	
}
