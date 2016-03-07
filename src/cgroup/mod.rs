extern crate libc;

use std::ffi::CString;
use std::fmt;
use std::os::raw::c_char;

#[allow(non_camel_case_types)]
#[repr(C)]
struct extern_cgroup;

#[allow(non_camel_case_types)]
#[repr(C)]
struct extern_cgroup_controller;

#[link(name = "cgroup")]
extern "C" {
    fn cgroup_init() -> i32;
    fn cgroup_new_cgroup(s: *const c_char) -> *mut extern_cgroup;
    fn cgroup_add_controller(cgroup: *mut extern_cgroup, name: *const c_char) -> *mut extern_cgroup_controller;
    fn cgroup_create_cgroup(cgroup: *mut extern_cgroup, ignore_ownership: i32) -> i32;
    fn cgroup_get_cgroup(cgroup: *mut extern_cgroup) -> i32;
}

pub struct Controller {
    controller: *mut extern_cgroup_controller,
}

pub struct Group<'a> {
    name: &'a str,
    cgroup: *mut extern_cgroup,
}

impl<'a> fmt::Debug for Group<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Group {{ name: {} }}", self.name)
    }
}

impl<'a> Group<'a> {
    pub fn new(name: &'a str) -> Group<'a> {
        let cgroup = unsafe { cgroup_new_cgroup(CString::new(name).unwrap().into_raw()) };
        Group {
            name: name,
            cgroup: cgroup,
        }
    }

    pub fn get(name: &'a str) -> Result<Group<'a>, i32> {
        let mut cgroup = Group::new(name);
        let retval = unsafe { cgroup_get_cgroup(cgroup.cgroup) };
        match retval {
            0 => {
                Ok(Group {
                    name: name,
                    cgroup: cgroup.cgroup,
                })
            }
            _ => Err(retval),
        }
    }

    pub fn add_controller(&mut self, name: &'a str) -> Controller {
        let controller = unsafe {
            cgroup_add_controller(self.cgroup, CString::new(name).unwrap().into_raw())
        };
        Controller { controller: controller }
    }

    pub fn create(&mut self) -> Result<&Group, i32> {
        let retval = unsafe { cgroup_create_cgroup(self.cgroup, 0) };
        match retval {
            0 => Ok(self),
            _ => Err(retval),
        }
    }
}

fn init() -> Result<(), i32> {
    let retval = unsafe { cgroup_init() };
    match retval {
        0 => Ok(()),
        _ => Err(retval),
    }
}

#[test]
fn test_init() {
    let i = init();
    match i {
        Err(j) => panic!("Failed to init with exit status: {}", j),
        _ => return,
    }
}

#[test]
fn test_get_cgroup() {
    match init() {
        Err(i) => panic!("Failed to init with error code: {}", i),
        _ => {},
    }
    
    let name = "foo";
    let mut new_group = Group::new(name);
    new_group.add_controller("cpuset");
    println!("New group: {:?}", new_group);
    let mut newer_group = new_group.create();
    match newer_group {
        Ok(g) => {
            println!("Created group: {:?}", g);
        }
        Err(code) => {
            panic!("Error creating group: {}", code);
        }
    }
    let get_group = Group::get(name);
    println!("Get group: {:?}", get_group);
    match get_group {
        Ok(g) => {
            assert!(g.name == name);
        },
        Err(i) => panic!("Failed to get the group with error code: {}", i),
    }
}
