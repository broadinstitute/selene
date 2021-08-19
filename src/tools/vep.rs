use crate::util::error::Error;

#[cfg(linux_installation)]
const VEP_WRAPPER_PATH: &str = "/usr/share/selene/bash/vep_wrapper.sh";

#[cfg(not(linux_installation))]
const VEP_WRAPPER_PATH: &str = "assets/bash/vep_wrapper.sh";

pub(crate) struct VepArgs {

}

pub(crate) fn run_vep(args: VepArgs) -> Result<i32, Error> {
    todo!()
}