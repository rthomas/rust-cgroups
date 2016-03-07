- Layout:


- cgroup: (Core structs, ffi code)
  - tools: external interface for interacting


Usage should be something like:

// Setup structs
let group = cgroup::Group::new("foo").add_controller("cpu");

// Create:
group.create();
// OR
group.create_from_parent();

// Get values from kernel
group.get();


