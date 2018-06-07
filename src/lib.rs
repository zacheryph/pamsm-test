extern crate pamsm;
extern crate time;

use pamsm::pam_raw::{PamError, PamFlag};
use pamsm::{Pam, PamServiceModule};

struct PamTime;

impl PamServiceModule for PamTime {
    fn authenticate(self: &Self, pamh: Pam, _flags: PamFlag, args: Vec<String>) -> PamError {
        println!("ARGS: {:?}", args);

        // This causes callbacks to show up
        let r = pamh.get_cached_authtok();
        println!("AUTHTOK: {:?}", r);

        let hour = time::now().tm_hour;
        if hour != 4 {
            // Only allow authentication when it's 4 AM
            PamError::SUCCESS
        } else {
            PamError::AUTH_ERR
        }
    }
}

#[no_mangle]
pub extern "C" fn get_pam_sm() -> Box<PamServiceModule> {
    return Box::new(PamTime {});
}

pub fn main() {}
