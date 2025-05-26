/// This member identifies the object file version.
///
/// The value 1 signifies the original file format;
/// extensions will create new versions with higher numbers.
/// Although the value of EV_CURRENT is shown as 1 in the
/// previous table, it will change as necessary to reflect
/// the current version number.
#[allow(non_camel_case_types)]
pub enum Version {
    EV_NONE = 0,    // Invalid version
    EV_CURRENT = 1, // Current version
}
