use cgroup;

pub struct ControlGroupInfo {
    value: String,
}

pub struct ControlGroupError {
    reason: String,
}




pub fn get(controller: &String, path: &String) -> Result<ControlGroupInfo, ControlGroupError> {
    let init = cgroup::init();
    Ok(ControlGroupInfo { value: "asasas".to_string() })
}



#[test]
fn it_works() {}