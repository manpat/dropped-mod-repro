
pub mod blah;

#[no_mangle] pub fn export_bin_main() {}

fn main() {
	// Make sure our lib crate is actually used
	// I would expect the lib not to be linked if unused, so this is fine
	dropped_mod_repro::force_linkage();
}