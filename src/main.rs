mod process_args;
use process_args::CommandLineOpts;
use process_args::Opt;
use structopt::StructOpt;
use libc::c_char;
use libc::c_void;
use libc::size_t;
use std::ffi::CString;

extern "C" {
    fn jailparam_init(param: *mut jailparam, name: *const c_char) -> i32;
    fn jailparam_get(param: *mut jailparam, njp: u32, flags: i32) -> i32;
    fn jail_getv(flags: i32) -> i32;
}

#[repr(C)]
struct jailparam {
	jp_name:      *const c_char,
	jp_value:     *mut c_void,
	jp_valuelen:   size_t,
	jp_elemlen:    size_t,
	jp_ctltype:    i32,
	jp_structtype: i32,
	jp_flags:      u32
}

fn main() {
    match CommandLineOpts::from_args().opt {
        Opt::Show { all } => unsafe {
          let vec: Vec<u8> = vec!(0);
          let mut param : jailparam = jailparam {
            jp_name: &0,
            jp_value: vec.as_ptr() as *mut c_void,
            jp_valuelen: 0,
            jp_elemlen: 0,
            jp_ctltype: 0,
            jp_structtype: 0,
            jp_flags: 0
          };
          jailparam_init(&mut param, CString::new("jls").unwrap().into_raw());
          println!("JID");
        },
        Opt::Create { name } => println!("Not implemented"),
        Opt::Delete { name } => println!("Not implemented")
    }
}
