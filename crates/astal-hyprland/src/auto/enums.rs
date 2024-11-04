// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,translate::*};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "AstalHyprlandFullscreen")]
pub enum Fullscreen {
    #[doc(alias = "ASTAL_HYPRLAND_FULLSCREEN_CURRENT")]
    Current,
    #[doc(alias = "ASTAL_HYPRLAND_FULLSCREEN_NONE")]
    None,
    #[doc(alias = "ASTAL_HYPRLAND_FULLSCREEN_FULLSCREEN")]
    Fullscreen,
    #[doc(alias = "ASTAL_HYPRLAND_FULLSCREEN_MAXIMIZED")]
    Maximized,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for Fullscreen {
    type GlibType = ffi::AstalHyprlandFullscreen;

    #[inline]
fn into_glib(self) -> ffi::AstalHyprlandFullscreen {
match self {
            Self::Current => ffi::ASTAL_HYPRLAND_FULLSCREEN_CURRENT,
            Self::None => ffi::ASTAL_HYPRLAND_FULLSCREEN_NONE,
            Self::Fullscreen => ffi::ASTAL_HYPRLAND_FULLSCREEN_FULLSCREEN,
            Self::Maximized => ffi::ASTAL_HYPRLAND_FULLSCREEN_MAXIMIZED,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::AstalHyprlandFullscreen> for Fullscreen {
    #[inline]
unsafe fn from_glib(value: ffi::AstalHyprlandFullscreen) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::ASTAL_HYPRLAND_FULLSCREEN_CURRENT => Self::Current,
            ffi::ASTAL_HYPRLAND_FULLSCREEN_NONE => Self::None,
            ffi::ASTAL_HYPRLAND_FULLSCREEN_FULLSCREEN => Self::Fullscreen,
            ffi::ASTAL_HYPRLAND_FULLSCREEN_MAXIMIZED => Self::Maximized,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for Fullscreen {
                #[inline]
    #[doc(alias = "astal_hyprland_fullscreen_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::astal_hyprland_fullscreen_get_type()) }
                }
            }

impl glib::HasParamSpec for Fullscreen {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder_with_default
                }
}

impl glib::value::ValueType for Fullscreen {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Fullscreen {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Fullscreen {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Fullscreen> for glib::Value {
    #[inline]
    fn from(v: Fullscreen) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
