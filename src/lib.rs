// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode character name properties
//! as described in
//! [Unicode Standard Annex #44](http://www.unicode.org/reports/tr44/).
//!
//! ```rust
//! extern crate unicode_charname;
//!
//! use unicode_charname::CharName;
//!
//! fn main() {
//!     assert_eq!('A'.char_name().unwrap_or_default().to_string(),
//!                "LATIN CAPITAL LETTER A");
//! }
//! ```
//!
//! ## crates.io
//!
//! You can use this package in your project by adding the following
//! to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! unicode-charname = "0.1"
//! ```
//!
//! ## `no_std` + `alloc` support
//!
//! This crate is completely `no_std` + `alloc` compatible. This can be enabled by disabling the `std` feature, i.e. specifying `default-features = false` for this crate on your `Cargo.toml`.

#![deny(missing_docs, unsafe_code)]
#![doc(
    html_logo_url = "https://unicode-rs.github.io/unicode-rs_sm.png",
    html_favicon_url = "https://unicode-rs.github.io/unicode-rs_sm.png"
)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate core;

#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::fmt;

#[rustfmt::skip]
mod tables;

mod jamo;
mod reserved;

pub use tables::UNICODE_VERSION;

/// Methods for retrieving character name for a code point.
pub trait CharName {
    /// Retrieve the character name for a code point.
    ///
    /// # Examples
    ///
    /// ```
    /// # use unicode_charname::CharName;
    /// assert_eq!('\u{1F402}'.char_name().unwrap_or_default().to_string(), "OX");
    /// ```
    ///
    /// Note that for all code points having a property value
    /// of na = "" for the Name property, the return value
    /// will be its hex representation dash-prefixed with a special label
    /// indicating its code point type (See D10a Code point type within
    /// the Unicode standard) within angle brackets.
    /// 
    /// The following special labels are used: `control`, `private-use`,
    /// `surrogate`, `noncharacter` and `reserved`.
    /// # Examples
    ///
    /// ```
    /// # use unicode_charname::CharName;
    /// assert_eq!('\u{81}'.char_name().unwrap_or_default().to_string(), "<control-0081>");
    /// ```
    /// This function never return `None` for valid
    /// Unicode code points, but always return `None` for
    /// other integers.
    ///
    /// # Examples
    ///
    /// ```
    /// # use unicode_charname::CharName;
    /// assert!(0x200000.char_name().is_none());
    /// ```
    fn char_name(self) -> Option<Name>;

    /// Retrieve the Unicode Name property value for a code point.
    ///
    /// Similar to `char_name` function, but also returns `None` for
    /// all code points having a property value of na = "" for the Name property.
    /// # Examples
    ///
    /// ```
    /// # use unicode_charname::CharName;
    /// assert!('\u{81}'.property_name().is_none());
    /// ```
    fn property_name(self) -> Option<Name>;
}

impl CharName for char {
    fn char_name(self) -> Option<Name> {
        CharName::char_name(self as u32)
    }
    fn property_name(self) -> Option<Name> {
        CharName::property_name(self as u32)
    }
}

impl CharName for u32 {
    fn char_name(self) -> Option<Name> {
        if let Some(slice) = tables::find_in_enumerate_names(self) {
            let name = Name(NameInner::Enumeration {
                encoded_slice: slice,
                codepoint_repr: alloc::format!("{:04X}", self),
            });
            return Some(name);
        }
        if let Some(special_group) = tables::find_in_special_groups(self) {
            return name_for_special_group_char(
                self,
                special_group,
                CodePointLabelMode::Label {
                    use_angle_bracket: true,
                },
            );
        }
        if reserved::is_code_point(self) {
            if reserved::is_noncharacter(self) {
                return Some(code_point_label("noncharacter-", self, true));
            } else {
                return Some(code_point_label("reserved-", self, true));
            }
        }
        None
    }

    fn property_name(self) -> Option<Name> {
        if let Some(slice) = tables::find_in_enumerate_names(self) {
            let name = Name(NameInner::Enumeration {
                encoded_slice: slice,
                codepoint_repr: alloc::format!("{:04X}", self),
            });
            return Some(name);
        }
        if let Some(special_group) = tables::find_in_special_groups(self) {
            return name_for_special_group_char(self, special_group, CodePointLabelMode::None);
        }
        None
    }
}

fn nr1_name(_prefix: &str, v: u32) -> Name {
    // ignore prefix here, because hangul_name will provide one.
    let str = jamo::hangul_name(v);
    Name(NameInner::Generated(str))
}

fn nr2_name(prefix: &str, v: u32) -> Name {
    Name(NameInner::Generated(alloc::format!("{}{:04X}", prefix, v)))
}

fn code_point_label(prefix: &str, v: u32, use_angle_bracket: bool) -> Name {
    let str = if use_angle_bracket {
        alloc::format!("<{}{:04X}>", prefix, v)
    } else {
        alloc::format!("{}{:04X}", prefix, v)
    };
    Name(NameInner::Generated(str))
}

enum CodePointLabelMode {
    None,
    Label { use_angle_bracket: bool },
}

fn name_for_special_group_char(
    v: u32,
    special_group: tables::SpecialGroup,
    code_point_label_mode: CodePointLabelMode,
) -> Option<Name> {
    use tables::SpecialGroup;
    match special_group {
        SpecialGroup::HangulSyllable => {
            // NR1
            Some(nr1_name("HANGUL SYLLABLE ", v))
        }
        SpecialGroup::CJKIdeographExtensionA
        | SpecialGroup::CJKIdeograph
        | SpecialGroup::CJKIdeographExtensionB
        | SpecialGroup::CJKIdeographExtensionC
        | SpecialGroup::CJKIdeographExtensionD
        | SpecialGroup::CJKIdeographExtensionE
        | SpecialGroup::CJKIdeographExtensionF
        | SpecialGroup::CJKIdeographExtensionG
        | SpecialGroup::CJKIdeographExtensionH
        | SpecialGroup::CJKIdeographExtensionI
        | SpecialGroup::CJKIdeographExtensionJ => {
            // NR2
            Some(nr2_name("CJK UNIFIED IDEOGRAPH-", v))
        }
        SpecialGroup::TangutIdeograph | SpecialGroup::TangutIdeographSupplement => {
            // NR2
            Some(nr2_name("TANGUT IDEOGRAPH-", v))
        }
        /* other NR2 cases already covered in UnicodeData.txt */
        SpecialGroup::control => {
            if let CodePointLabelMode::Label { use_angle_bracket } = code_point_label_mode {
                Some(code_point_label("control-", v, use_angle_bracket))
            } else {
                None
            }
        }
        SpecialGroup::NonPrivateUseHighSurrogate
        | SpecialGroup::PrivateUseHighSurrogate
        | SpecialGroup::LowSurrogate => {
            if let CodePointLabelMode::Label { use_angle_bracket } = code_point_label_mode {
                Some(code_point_label("surrogate-", v, use_angle_bracket))
            } else {
                None
            }
        }
        SpecialGroup::PrivateUse
        | SpecialGroup::Plane15PrivateUse
        | SpecialGroup::Plane16PrivateUse => {
            if let CodePointLabelMode::Label { use_angle_bracket } = code_point_label_mode {
                Some(code_point_label("private-use-", v, use_angle_bracket))
            } else {
                None
            }
        }
    }
}

#[derive(Clone)]
enum NameInner {
    Enumeration {
        encoded_slice: &'static [u16],
        codepoint_repr: String,
    },
    Generated(String),
}

/// Represents retrieved Unicode character name.
///
/// It implements the `Display` trait and can also
/// be converted to a `String` value with `to_string`
/// method.
#[derive(Clone)]
pub struct Name(NameInner);

impl Default for Name {
    fn default() -> Self {
        Name(NameInner::Generated(Default::default()))
    }
}

impl Name {
    fn iter(&self) -> NameIter<'_> {
        NameIter {
            name: &self.0,
            offset: 0,
            state: NameIterState::Initial,
        }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for s in self.iter() {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
#[non_exhaustive]
struct NameIter<'a> {
    name: &'a NameInner,
    offset: usize,
    state: NameIterState,
}

#[derive(Copy, Clone)]
enum NameIterState {
    Initial,
    InsertSpace { cur_special: bool },
    Middle { cur_special: bool },
    Finished,
}

impl<'a> Iterator for NameIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        match self.name {
            NameInner::Enumeration {
                encoded_slice,
                codepoint_repr,
            } => match self.state {
                NameIterState::Finished => None,
                _ if self.offset >= encoded_slice.len() => {
                    self.state = NameIterState::Finished;
                    None
                }
                NameIterState::InsertSpace { cur_special } => {
                    self.state = NameIterState::Middle { cur_special };
                    Some(tables::ENUMERATION_WORD_TABLE[tables::WORD_TABLE_INDEX_SPACE as usize])
                }
                _ => {
                    /* NameIterState::Initial | NameIterState::Middle {..} */
                    let cur_word_idx = encoded_slice[self.offset];
                    self.offset += 1;
                    if let Some(&next_word_idx) = encoded_slice.get(self.offset) {
                        let cur_special = match self.state {
                            NameIterState::Initial => tables::is_special_word_index(cur_word_idx),
                            NameIterState::Middle { cur_special } => cur_special,
                            _ => unreachable!(),
                        };
                        let next_special = tables::is_special_word_index(next_word_idx);
                        if !cur_special && !next_special {
                            self.state = NameIterState::InsertSpace {
                                cur_special: next_special,
                            };
                        } else {
                            self.state = NameIterState::Middle {
                                cur_special: next_special,
                            };
                        }
                    } else {
                        self.state = NameIterState::Finished;
                    }
                    if cur_word_idx == tables::WORD_TABLE_INDEX_CODEPOINT {
                        Some(codepoint_repr)
                    } else {
                        Some(tables::ENUMERATION_WORD_TABLE[cur_word_idx as usize])
                    }
                }
            },
            NameInner::Generated(s) => match self.state {
                NameIterState::Initial => {
                    self.state = NameIterState::Finished;
                    Some(&s)
                }
                NameIterState::Finished => None,
                _ => unreachable!(),
            },
        }
    }
}
