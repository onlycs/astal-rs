// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

mod audio;
pub use self::audio::Audio;

mod device;
pub use self::device::Device;

mod endpoint;
pub use self::endpoint::Endpoint;

mod profile;
pub use self::profile::Profile;

mod video;
pub use self::video::Video;

mod wp;
pub use self::wp::Wp;

mod enums;
pub use self::enums::DeviceType;
pub use self::enums::MediaClass;
pub use self::enums::Scale;

pub mod builders {
    pub use super::device::DeviceBuilder;
    pub use super::endpoint::EndpointBuilder;
    pub use super::profile::ProfileBuilder;
    pub use super::wp::WpBuilder;
}
