
pub mod this_will_be_dropped;
pub mod this_will_be_exported;

#[no_mangle]
pub fn export_lib_main() {
	this_will_be_exported::force_linkage();
}

pub fn force_linkage() {}