extern crate libc;

use std::ffi::CString;
use std::fmt;
use std::os::raw::c_char;

// libcgroup pointers.
enum CGroup {}
enum CGroupController {}

#[link(name = "cgroup")]
extern "C" {
    fn cgroup_init() -> i32;
    fn cgroup_new_cgroup(s: *const c_char) -> *mut CGroup;
    fn cgroup_add_controller(cgroup: *mut CGroup, name: *const c_char) -> *mut CGroupController;
    fn cgroup_get_controller(cgroup: *mut CGroup, name: *const c_char) -> *mut CGroupController;
    fn cgroup_create_cgroup(cgroup: *mut CGroup, ignore_ownership: i32) -> i32;
    fn cgroup_get_cgroup(cgroup: *mut CGroup) -> i32;
}

#[derive(Debug)]
pub struct Controller {
    controller: *mut CGroupController,
}

#[derive(Debug)]
pub struct Group<'a> {
    name: &'a str,
    cgroup: *mut CGroup,
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
        let cgroup = Group::new(name);
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

    pub fn get_controller(&mut self, name: &'a str) -> Controller {
        let controller = unsafe {
            cgroup_get_controller(self.cgroup, CString::new(name).unwrap().into_raw())
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

pub fn init() -> Result<(), i32> {
    let retval = unsafe { cgroup_init() };
    match retval {
        0 => Ok(()),
        _ => Err(retval),
    }
}

#[allow(dead_code)]
fn init_or_panic() {
    match init() {
        Err(i) => panic!("Failed to init with error code: {}", i),
        _ => {},
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
    init_or_panic();
    
    let name = "foo";
    let mut new_group = Group::new(name);
    new_group.add_controller("cpuset");
    println!("New group: {:?}", new_group);
    let newer_group = new_group.create();
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

#[test]
fn test_get_controller() {
    init_or_panic();
    let cgroup_name = "testgroup";
    let mut group = Group::new(cgroup_name);
    group.add_controller("cpuset");
    group.get_controller("cpuset");
}
