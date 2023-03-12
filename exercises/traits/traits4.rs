// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
// Don't change any line other than the marked one.
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a hint.

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn compare_license_types<ImplLicensed1: Licensed, ImplLicensed2: Licensed>(software: ImplLicensed1, software_two: ImplLicensed2) -> bool {
    software.licensing_info() == software_two.licensing_info()
}
/**
 * Since Rust has no inheritence, it must be two different generic types both having one common trait but still are different classes/structs
 * A simpler way is using impl instead of generics (its just synthetic sugar)
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
