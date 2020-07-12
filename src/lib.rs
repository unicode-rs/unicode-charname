use std::fmt;

#[rustfmt::skip]
mod tables;

pub trait CharName {
    fn char_name(self) -> Option<Name>;
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

fn find_in_enumerate_names(ch: u32) -> Option<&'static [u16]> {
    use tables::ENUMERATION_CHAR_NAMES;
    let record_idx = ENUMERATION_CHAR_NAMES
        .binary_search_by(|record| {
            use std::cmp::Ordering;
            if record.1 < ch {
                Ordering::Less
            } else if record.0 > ch {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
        .ok()?;
    let offset = (ch - ENUMERATION_CHAR_NAMES[record_idx].0) as usize;
    let index_slice = ENUMERATION_CHAR_NAMES[record_idx].2;
    let offset_slice = ENUMERATION_CHAR_NAMES[record_idx].3;
    let range = (offset_slice[offset] as usize)..(offset_slice[offset + 1] as usize);
    Some(&index_slice[range])
}

impl CharName for u32 {
    fn char_name(self) -> Option<Name> {
        if let Some(slice) = find_in_enumerate_names(self) {
            let name = Name(NameInner::Enumeration {
                encoded_slice: slice,
                codepoint_repr: format!("{:04X}", self),
            });
            return Some(name);
        }
        todo!()
    }

    fn property_name(self) -> Option<Name> {
        if let Some(slice) = find_in_enumerate_names(self) {
            let name = Name(NameInner::Enumeration {
                encoded_slice: slice,
                codepoint_repr: format!("{:04X}", self),
            });
            return Some(name);
        }
        todo!()
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

#[derive(Clone)]
pub struct Name(NameInner);

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
pub struct NameIter<'a> {
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
                NameIterState::Finished => {
                    None
                },
                _ if self.offset >= encoded_slice.len() => {
                    self.state = NameIterState::Finished;
                    None
                },
                NameIterState::InsertSpace {cur_special} => {
                    self.state = NameIterState::Middle{cur_special};
                    Some(tables::ENUMERATION_WORD_TABLE[tables::WORD_TABLE_INDEX_SPACE as usize])
                },
                _ => {
                    /* NameIterState::Initial | NameIterState::Midle {..} */
                    let cur_word_idx = encoded_slice[self.offset];
                    self.offset += 1;
                    if let Some(&next_word_idx) = encoded_slice.get(self.offset) {
                        let cur_special = match self.state {
                            NameIterState::Initial => {
                                tables::is_special_word_index(cur_word_idx)
                            },
                            NameIterState::Middle{cur_special} => cur_special,
                            _ => unreachable!(),
                        };
                        let next_special = tables::is_special_word_index(next_word_idx);
                        if !cur_special && !next_special {
                            self.state = NameIterState::InsertSpace{cur_special: next_special};
                        } else {
                            self.state = NameIterState::Middle{cur_special: next_special};
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
