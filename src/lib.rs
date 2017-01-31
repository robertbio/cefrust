pub mod cef;
pub mod base;

use std::ffi;
use std::os::unix::ffi::OsStrExt;
use std::os::raw;

pub fn prepare_args() -> cef::_cef_main_args_t {
    let argv:Vec<ffi::CString> = std::env::args_os().map(|arg| {
        //println!("argv: {:?}", arg);     
        //ffi::CString::new(arg.into_string().unwrap()).unwrap() 
        let osstr: &ffi::OsStr = arg.as_os_str();
        let carg = ffi::CString::new(osstr.as_bytes());
        carg.expect("Null arg") 
    } ).collect();

    let mut args:Vec<*const raw::c_char> = argv.iter().map(|arg| { 
        println!("args: {:?}", arg);
        arg.as_ptr() 
    } ).collect();

    //let gpu = ffi::CString::new("--disable-gpu").unwrap();
    //args.insert(1, gpu.as_ptr());
    //fuse_main_real(args.len() as c_int, args.as_ptr() as *const *const c_char, .... );
 
    // Structure for passing command-line arguments.
    // The definition of this structure is platform-specific.
    /*let mut args:Vec<*mut raw::c_char> = std::env::args().map(|arg| {
        println!("argv: {:?}", arg);     
        //ffi::CString::new(arg.into_string().unwrap()).unwrap() 
        //let osstr: &ffi::OsStr = arg.as_os_str();
        let carg_rslt = ffi::CString::new(arg.as_bytes());
        let carg = carg_rslt.expect("Null arg");
        carg.into_raw()
    } ).collect();*/
    let args_size = args.len() as raw::c_int;
    let args_ptr = args.as_mut_ptr();
    let args_box = Box::new(args);
    //let args = Box::into_raw(args);
    std::mem::forget(args_box);

    let main_args = cef::_cef_main_args_t {
        argc : args_size,
        argv : args_ptr as *mut *mut raw::c_char
    };
    println!("Hello CEF, ARGS: {:?}", main_args.argc);

    //println!("arg0: {:?}", argv[0].into_string().unwrap());
    //println!("arg0: {:?}", args[0]);

    main_args
}

pub fn cef_string(value: &str) -> cef::cef_string_t {
    let mut str_cef = cef::cef_string_t {str: std::ptr::null_mut(), length: 0, dtor: Option::Some(dtr)};
    //unsafe { cef::cef_string_utf16_set(value.as_ptr() as *mut cef::char16, value.len(), &mut str_cef, 1) };
    unsafe {cef::cef_string_utf8_to_utf16(value.as_ptr() as *mut std::os::raw::c_char, value.len(), &mut str_cef);}
    str_cef
}


pub fn cef_string_empty() -> cef::cef_string_t {
    let mut empty_str = cef::cef_string_t {
        str: std::ptr::null_mut(), 
        length: 0, 
        dtor: Option::Some(dtr)
    };
    
    let emp = "";
    //unsafe { cef::cef_string_utf16_set(emp.as_ptr() as *mut cef::char16, 0, &mut empty_str, 1) };
    unsafe { cef::cef_string_utf8_to_utf16(emp.as_ptr() as *mut std::os::raw::c_char, 0, &mut empty_str);}

    empty_str
}

unsafe extern "C" fn dtr(str: *mut cef::char16) {
    println!("DESTROY CEF_STRING");
}