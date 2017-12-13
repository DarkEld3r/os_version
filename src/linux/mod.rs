mod lsb_release;
mod file_release;

use { Info };

pub fn current_platform() -> Info {
    if lsb_release::is_available() {
        lsb_release::lsb_release()
    } else {
        file_release::file_release()
    }
}
