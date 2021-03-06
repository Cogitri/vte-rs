// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::Quark;
use glib::StaticType;
use glib::Type;
use glib::error::ErrorDomain;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_sys;
use std::fmt;
use vte_sys;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum CursorBlinkMode {
    System,
    On,
    Off,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for CursorBlinkMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CursorBlinkMode::{}", match *self {
            CursorBlinkMode::System => "System",
            CursorBlinkMode::On => "On",
            CursorBlinkMode::Off => "Off",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for CursorBlinkMode {
    type GlibType = vte_sys::VteCursorBlinkMode;

    fn to_glib(&self) -> vte_sys::VteCursorBlinkMode {
        match *self {
            CursorBlinkMode::System => vte_sys::VTE_CURSOR_BLINK_SYSTEM,
            CursorBlinkMode::On => vte_sys::VTE_CURSOR_BLINK_ON,
            CursorBlinkMode::Off => vte_sys::VTE_CURSOR_BLINK_OFF,
            CursorBlinkMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<vte_sys::VteCursorBlinkMode> for CursorBlinkMode {
    fn from_glib(value: vte_sys::VteCursorBlinkMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => CursorBlinkMode::System,
            1 => CursorBlinkMode::On,
            2 => CursorBlinkMode::Off,
            value => CursorBlinkMode::__Unknown(value),
        }
    }
}

impl StaticType for CursorBlinkMode {
    fn static_type() -> Type {
        unsafe { from_glib(vte_sys::vte_cursor_blink_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CursorBlinkMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CursorBlinkMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for CursorBlinkMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum CursorShape {
    Block,
    Ibeam,
    Underline,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for CursorShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CursorShape::{}", match *self {
            CursorShape::Block => "Block",
            CursorShape::Ibeam => "Ibeam",
            CursorShape::Underline => "Underline",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for CursorShape {
    type GlibType = vte_sys::VteCursorShape;

    fn to_glib(&self) -> vte_sys::VteCursorShape {
        match *self {
            CursorShape::Block => vte_sys::VTE_CURSOR_SHAPE_BLOCK,
            CursorShape::Ibeam => vte_sys::VTE_CURSOR_SHAPE_IBEAM,
            CursorShape::Underline => vte_sys::VTE_CURSOR_SHAPE_UNDERLINE,
            CursorShape::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<vte_sys::VteCursorShape> for CursorShape {
    fn from_glib(value: vte_sys::VteCursorShape) -> Self {
        skip_assert_initialized!();
        match value {
            0 => CursorShape::Block,
            1 => CursorShape::Ibeam,
            2 => CursorShape::Underline,
            value => CursorShape::__Unknown(value),
        }
    }
}

impl StaticType for CursorShape {
    fn static_type() -> Type {
        unsafe { from_glib(vte_sys::vte_cursor_shape_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CursorShape {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CursorShape {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for CursorShape {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum EraseBinding {
    Auto,
    AsciiBackspace,
    AsciiDelete,
    DeleteSequence,
    Tty,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for EraseBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EraseBinding::{}", match *self {
            EraseBinding::Auto => "Auto",
            EraseBinding::AsciiBackspace => "AsciiBackspace",
            EraseBinding::AsciiDelete => "AsciiDelete",
            EraseBinding::DeleteSequence => "DeleteSequence",
            EraseBinding::Tty => "Tty",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for EraseBinding {
    type GlibType = vte_sys::VteEraseBinding;

    fn to_glib(&self) -> vte_sys::VteEraseBinding {
        match *self {
            EraseBinding::Auto => vte_sys::VTE_ERASE_AUTO,
            EraseBinding::AsciiBackspace => vte_sys::VTE_ERASE_ASCII_BACKSPACE,
            EraseBinding::AsciiDelete => vte_sys::VTE_ERASE_ASCII_DELETE,
            EraseBinding::DeleteSequence => vte_sys::VTE_ERASE_DELETE_SEQUENCE,
            EraseBinding::Tty => vte_sys::VTE_ERASE_TTY,
            EraseBinding::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<vte_sys::VteEraseBinding> for EraseBinding {
    fn from_glib(value: vte_sys::VteEraseBinding) -> Self {
        skip_assert_initialized!();
        match value {
            0 => EraseBinding::Auto,
            1 => EraseBinding::AsciiBackspace,
            2 => EraseBinding::AsciiDelete,
            3 => EraseBinding::DeleteSequence,
            4 => EraseBinding::Tty,
            value => EraseBinding::__Unknown(value),
        }
    }
}

impl StaticType for EraseBinding {
    fn static_type() -> Type {
        unsafe { from_glib(vte_sys::vte_erase_binding_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for EraseBinding {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for EraseBinding {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for EraseBinding {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v0_50", feature = "dox"))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum Format {
    Text,
    Html,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v0_50", feature = "dox"))]
impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Format::{}", match *self {
            Format::Text => "Text",
            Format::Html => "Html",
            _ => "Unknown",
        })
    }
}

#[cfg(any(feature = "v0_50", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for Format {
    type GlibType = vte_sys::VteFormat;

    fn to_glib(&self) -> vte_sys::VteFormat {
        match *self {
            Format::Text => vte_sys::VTE_FORMAT_TEXT,
            Format::Html => vte_sys::VTE_FORMAT_HTML,
            Format::__Unknown(value) => value
        }
    }
}

#[cfg(any(feature = "v0_50", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<vte_sys::VteFormat> for Format {
    fn from_glib(value: vte_sys::VteFormat) -> Self {
        skip_assert_initialized!();
        match value {
            1 => Format::Text,
            2 => Format::Html,
            value => Format::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v0_50", feature = "dox"))]
impl StaticType for Format {
    fn static_type() -> Type {
        unsafe { from_glib(vte_sys::vte_format_get_type()) }
    }
}

#[cfg(any(feature = "v0_50", feature = "dox"))]
impl<'a> FromValueOptional<'a> for Format {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v0_50", feature = "dox"))]
impl<'a> FromValue<'a> for Format {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_50", feature = "dox"))]
impl SetValue for Format {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum PtyError {
    PtyHelperFailed,
    Pty98Failed,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PtyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PtyError::{}", match *self {
            PtyError::PtyHelperFailed => "PtyHelperFailed",
            PtyError::Pty98Failed => "Pty98Failed",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for PtyError {
    type GlibType = vte_sys::VtePtyError;

    fn to_glib(&self) -> vte_sys::VtePtyError {
        match *self {
            PtyError::PtyHelperFailed => vte_sys::VTE_PTY_ERROR_PTY_HELPER_FAILED,
            PtyError::Pty98Failed => vte_sys::VTE_PTY_ERROR_PTY98_FAILED,
            PtyError::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<vte_sys::VtePtyError> for PtyError {
    fn from_glib(value: vte_sys::VtePtyError) -> Self {
        skip_assert_initialized!();
        match value {
            0 => PtyError::PtyHelperFailed,
            1 => PtyError::Pty98Failed,
            value => PtyError::__Unknown(value),
        }
    }
}

impl ErrorDomain for PtyError {
    fn domain() -> Quark {
        skip_assert_initialized!();
        unsafe { from_glib(vte_sys::vte_pty_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            0 => Some(PtyError::PtyHelperFailed),
            1 => Some(PtyError::Pty98Failed),
            value => Some(PtyError::__Unknown(value)),
        }
    }
}

impl StaticType for PtyError {
    fn static_type() -> Type {
        unsafe { from_glib(vte_sys::vte_pty_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PtyError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PtyError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PtyError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v0_52", feature = "dox"))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum TextBlinkMode {
    Never,
    Focused,
    Unfocused,
    Always,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v0_52", feature = "dox"))]
impl fmt::Display for TextBlinkMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextBlinkMode::{}", match *self {
            TextBlinkMode::Never => "Never",
            TextBlinkMode::Focused => "Focused",
            TextBlinkMode::Unfocused => "Unfocused",
            TextBlinkMode::Always => "Always",
            _ => "Unknown",
        })
    }
}

#[cfg(any(feature = "v0_52", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for TextBlinkMode {
    type GlibType = vte_sys::VteTextBlinkMode;

    fn to_glib(&self) -> vte_sys::VteTextBlinkMode {
        match *self {
            TextBlinkMode::Never => vte_sys::VTE_TEXT_BLINK_NEVER,
            TextBlinkMode::Focused => vte_sys::VTE_TEXT_BLINK_FOCUSED,
            TextBlinkMode::Unfocused => vte_sys::VTE_TEXT_BLINK_UNFOCUSED,
            TextBlinkMode::Always => vte_sys::VTE_TEXT_BLINK_ALWAYS,
            TextBlinkMode::__Unknown(value) => value
        }
    }
}

#[cfg(any(feature = "v0_52", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<vte_sys::VteTextBlinkMode> for TextBlinkMode {
    fn from_glib(value: vte_sys::VteTextBlinkMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => TextBlinkMode::Never,
            1 => TextBlinkMode::Focused,
            2 => TextBlinkMode::Unfocused,
            3 => TextBlinkMode::Always,
            value => TextBlinkMode::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v0_52", feature = "dox"))]
impl StaticType for TextBlinkMode {
    fn static_type() -> Type {
        unsafe { from_glib(vte_sys::vte_text_blink_mode_get_type()) }
    }
}

#[cfg(any(feature = "v0_52", feature = "dox"))]
impl<'a> FromValueOptional<'a> for TextBlinkMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v0_52", feature = "dox"))]
impl<'a> FromValue<'a> for TextBlinkMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_52", feature = "dox"))]
impl SetValue for TextBlinkMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum WriteFlags {
    Default,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for WriteFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WriteFlags::{}", match *self {
            WriteFlags::Default => "Default",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for WriteFlags {
    type GlibType = vte_sys::VteWriteFlags;

    fn to_glib(&self) -> vte_sys::VteWriteFlags {
        match *self {
            WriteFlags::Default => vte_sys::VTE_WRITE_DEFAULT,
            WriteFlags::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<vte_sys::VteWriteFlags> for WriteFlags {
    fn from_glib(value: vte_sys::VteWriteFlags) -> Self {
        skip_assert_initialized!();
        match value {
            0 => WriteFlags::Default,
            value => WriteFlags::__Unknown(value),
        }
    }
}

impl StaticType for WriteFlags {
    fn static_type() -> Type {
        unsafe { from_glib(vte_sys::vte_write_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WriteFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WriteFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WriteFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

