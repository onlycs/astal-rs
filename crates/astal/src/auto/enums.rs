// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../../gobject/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,translate::*};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "AstalExclusivity")]
pub enum Exclusivity {
    #[doc(alias = "ASTAL_EXCLUSIVITY_NORMAL")]
    Normal,
    #[doc(alias = "ASTAL_EXCLUSIVITY_EXCLUSIVE")]
    Exclusive,
    #[doc(alias = "ASTAL_EXCLUSIVITY_IGNORE")]
    Ignore,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for Exclusivity {
    type GlibType = ffi::AstalExclusivity;

    #[inline]
fn into_glib(self) -> ffi::AstalExclusivity {
match self {
            Self::Normal => ffi::ASTAL_EXCLUSIVITY_NORMAL,
            Self::Exclusive => ffi::ASTAL_EXCLUSIVITY_EXCLUSIVE,
            Self::Ignore => ffi::ASTAL_EXCLUSIVITY_IGNORE,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::AstalExclusivity> for Exclusivity {
    #[inline]
unsafe fn from_glib(value: ffi::AstalExclusivity) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::ASTAL_EXCLUSIVITY_NORMAL => Self::Normal,
            ffi::ASTAL_EXCLUSIVITY_EXCLUSIVE => Self::Exclusive,
            ffi::ASTAL_EXCLUSIVITY_IGNORE => Self::Ignore,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for Exclusivity {
                #[inline]
    #[doc(alias = "astal_exclusivity_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::astal_exclusivity_get_type()) }
                }
            }

impl glib::HasParamSpec for Exclusivity {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder_with_default
                }
}

impl glib::value::ValueType for Exclusivity {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Exclusivity {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Exclusivity {
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

impl From<Exclusivity> for glib::Value {
    #[inline]
    fn from(v: Exclusivity) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "AstalKeymode")]
pub enum Keymode {
    #[doc(alias = "ASTAL_KEYMODE_NONE")]
    None,
    #[doc(alias = "ASTAL_KEYMODE_EXCLUSIVE")]
    Exclusive,
    #[doc(alias = "ASTAL_KEYMODE_ON_DEMAND")]
    OnDemand,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for Keymode {
    type GlibType = ffi::AstalKeymode;

    #[inline]
fn into_glib(self) -> ffi::AstalKeymode {
match self {
            Self::None => ffi::ASTAL_KEYMODE_NONE,
            Self::Exclusive => ffi::ASTAL_KEYMODE_EXCLUSIVE,
            Self::OnDemand => ffi::ASTAL_KEYMODE_ON_DEMAND,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::AstalKeymode> for Keymode {
    #[inline]
unsafe fn from_glib(value: ffi::AstalKeymode) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::ASTAL_KEYMODE_NONE => Self::None,
            ffi::ASTAL_KEYMODE_EXCLUSIVE => Self::Exclusive,
            ffi::ASTAL_KEYMODE_ON_DEMAND => Self::OnDemand,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for Keymode {
                #[inline]
    #[doc(alias = "astal_keymode_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::astal_keymode_get_type()) }
                }
            }

impl glib::HasParamSpec for Keymode {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder_with_default
                }
}

impl glib::value::ValueType for Keymode {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Keymode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Keymode {
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

impl From<Keymode> for glib::Value {
    #[inline]
    fn from(v: Keymode) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "AstalLayer")]
pub enum Layer {
    #[doc(alias = "ASTAL_LAYER_BACKGROUND")]
    Background,
    #[doc(alias = "ASTAL_LAYER_BOTTOM")]
    Bottom,
    #[doc(alias = "ASTAL_LAYER_TOP")]
    Top,
    #[doc(alias = "ASTAL_LAYER_OVERLAY")]
    Overlay,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for Layer {
    type GlibType = ffi::AstalLayer;

    #[inline]
fn into_glib(self) -> ffi::AstalLayer {
match self {
            Self::Background => ffi::ASTAL_LAYER_BACKGROUND,
            Self::Bottom => ffi::ASTAL_LAYER_BOTTOM,
            Self::Top => ffi::ASTAL_LAYER_TOP,
            Self::Overlay => ffi::ASTAL_LAYER_OVERLAY,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::AstalLayer> for Layer {
    #[inline]
unsafe fn from_glib(value: ffi::AstalLayer) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::ASTAL_LAYER_BACKGROUND => Self::Background,
            ffi::ASTAL_LAYER_BOTTOM => Self::Bottom,
            ffi::ASTAL_LAYER_TOP => Self::Top,
            ffi::ASTAL_LAYER_OVERLAY => Self::Overlay,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for Layer {
                #[inline]
    #[doc(alias = "astal_layer_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::astal_layer_get_type()) }
                }
            }

impl glib::HasParamSpec for Layer {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder_with_default
                }
}

impl glib::value::ValueType for Layer {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Layer {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Layer {
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

impl From<Layer> for glib::Value {
    #[inline]
    fn from(v: Layer) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "AstalMouseButton")]
pub enum MouseButton {
    #[doc(alias = "ASTAL_MOUSE_BUTTON_PRIMARY")]
    Primary,
    #[doc(alias = "ASTAL_MOUSE_BUTTON_MIDDLE")]
    Middle,
    #[doc(alias = "ASTAL_MOUSE_BUTTON_SECONDARY")]
    Secondary,
    #[doc(alias = "ASTAL_MOUSE_BUTTON_BACK")]
    Back,
    #[doc(alias = "ASTAL_MOUSE_BUTTON_FORWARD")]
    Forward,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for MouseButton {
    type GlibType = ffi::AstalMouseButton;

    #[inline]
fn into_glib(self) -> ffi::AstalMouseButton {
match self {
            Self::Primary => ffi::ASTAL_MOUSE_BUTTON_PRIMARY,
            Self::Middle => ffi::ASTAL_MOUSE_BUTTON_MIDDLE,
            Self::Secondary => ffi::ASTAL_MOUSE_BUTTON_SECONDARY,
            Self::Back => ffi::ASTAL_MOUSE_BUTTON_BACK,
            Self::Forward => ffi::ASTAL_MOUSE_BUTTON_FORWARD,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::AstalMouseButton> for MouseButton {
    #[inline]
unsafe fn from_glib(value: ffi::AstalMouseButton) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::ASTAL_MOUSE_BUTTON_PRIMARY => Self::Primary,
            ffi::ASTAL_MOUSE_BUTTON_MIDDLE => Self::Middle,
            ffi::ASTAL_MOUSE_BUTTON_SECONDARY => Self::Secondary,
            ffi::ASTAL_MOUSE_BUTTON_BACK => Self::Back,
            ffi::ASTAL_MOUSE_BUTTON_FORWARD => Self::Forward,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for MouseButton {
                #[inline]
    #[doc(alias = "astal_mouse_button_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::astal_mouse_button_get_type()) }
                }
            }

impl glib::HasParamSpec for MouseButton {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder_with_default
                }
}

impl glib::value::ValueType for MouseButton {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for MouseButton {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for MouseButton {
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

impl From<MouseButton> for glib::Value {
    #[inline]
    fn from(v: MouseButton) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "AstalWindowAnchor")]
pub enum WindowAnchor {
    #[doc(alias = "ASTAL_WINDOW_ANCHOR_NONE")]
    None,
    #[doc(alias = "ASTAL_WINDOW_ANCHOR_TOP")]
    Top,
    #[doc(alias = "ASTAL_WINDOW_ANCHOR_RIGHT")]
    Right,
    #[doc(alias = "ASTAL_WINDOW_ANCHOR_LEFT")]
    Left,
    #[doc(alias = "ASTAL_WINDOW_ANCHOR_BOTTOM")]
    Bottom,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for WindowAnchor {
    type GlibType = ffi::AstalWindowAnchor;

    #[inline]
fn into_glib(self) -> ffi::AstalWindowAnchor {
match self {
            Self::None => ffi::ASTAL_WINDOW_ANCHOR_NONE,
            Self::Top => ffi::ASTAL_WINDOW_ANCHOR_TOP,
            Self::Right => ffi::ASTAL_WINDOW_ANCHOR_RIGHT,
            Self::Left => ffi::ASTAL_WINDOW_ANCHOR_LEFT,
            Self::Bottom => ffi::ASTAL_WINDOW_ANCHOR_BOTTOM,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::AstalWindowAnchor> for WindowAnchor {
    #[inline]
unsafe fn from_glib(value: ffi::AstalWindowAnchor) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::ASTAL_WINDOW_ANCHOR_NONE => Self::None,
            ffi::ASTAL_WINDOW_ANCHOR_TOP => Self::Top,
            ffi::ASTAL_WINDOW_ANCHOR_RIGHT => Self::Right,
            ffi::ASTAL_WINDOW_ANCHOR_LEFT => Self::Left,
            ffi::ASTAL_WINDOW_ANCHOR_BOTTOM => Self::Bottom,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for WindowAnchor {
                #[inline]
    #[doc(alias = "astal_window_anchor_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::astal_window_anchor_get_type()) }
                }
            }

impl glib::HasParamSpec for WindowAnchor {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder_with_default
                }
}

impl glib::value::ValueType for WindowAnchor {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for WindowAnchor {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for WindowAnchor {
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

impl From<WindowAnchor> for glib::Value {
    #[inline]
    fn from(v: WindowAnchor) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

