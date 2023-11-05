use std::ops::Index;

use ascii::{AsciiChar, AsciiString, AsciiStr};

use crate::{Context, Result, MAX_CHANNELS};

#[derive(Default, Clone)]
pub struct NamedColorEntry {
    pub name: AsciiString,
    pub pcs: [u16; 3],
    pub device_colorant: [u16; MAX_CHANNELS],
}

pub struct NamedColor {
    pub(crate) context_id: Context,
    pub(crate) n_colors: usize,
    pub(crate) allocated: usize,
    pub(crate) colorant_count: usize,
    pub(crate) prefix: AsciiString,
    pub(crate) suffix: AsciiString,
    pub(crate) list: Vec<NamedColorEntry>,
}

impl NamedColor {
    fn grow(&mut self) -> Result<()> {
        const MAX_SIZE: usize = 1024 * 100;

        let size = if self.allocated == 0 {
            64usize
        } else {
            self.allocated * 2
        };

        // Keep a maximum color lists can grow, 100K entries seems reasonable
        let size = if self.allocated < MAX_SIZE && size > MAX_SIZE {
            MAX_SIZE
        } else {
            size
        };
        let size = if size > MAX_SIZE {
            if self.allocated < MAX_SIZE {
                MAX_SIZE
            } else {
                return Err("Named color lists cannot have more than 100K entries".into());
            }
        } else {
            size
        };

        self.list.resize_with(size, NamedColorEntry::default);
        self.allocated = size;

        Ok(())
    }

    pub fn new(
        context_id: &Context,
        n: usize,
        colorant_count: usize,
        prefix: &str,
        suffix: &str,
    ) -> Result<Self> {
        Self::_new(
            context_id,
            n,
            colorant_count,
            &strip_non_ascii(prefix),
            &strip_non_ascii(suffix),
        )
    }

    fn _new(
        context_id: &Context,
        n: usize,
        colorant_count: usize,
        prefix: &AsciiStr,
        suffix: &AsciiStr,
    ) -> Result<Self> {
        let list = Vec::<NamedColorEntry>::new();
        let n_colors = 0usize;
        let allocated = 0usize;

        let mut prefix = prefix.to_owned();
        let mut suffix = suffix.to_owned();

        prefix.truncate(32);
        suffix.truncate(32);

        let mut v = Self {
            context_id: context_id.clone(),
            n_colors,
            allocated,
            colorant_count,
            prefix,
            suffix,
            list,
        };

        while v.allocated < n {
            v.grow()?;
        }

        Ok(v)
    }

    pub fn dup(&self) -> Result<Self> {
        let mut new_nc = Self::_new(
            &self.context_id,
            self.n_colors,
            self.colorant_count,
            &self.prefix,
            &self.suffix,
        )?;

        // For really large tables we need this
        while new_nc.allocated < self.allocated {
            new_nc.grow()?
        }

        new_nc.list[..self.list.len()].clone_from_slice(&self.list);

        Ok(new_nc)
    }

    pub fn push(
        &mut self,
        name: &str,
        pcs: Option<[u16; 3]>,
        colorant: Option<[u16; MAX_CHANNELS]>,
    ) -> Result<()> {
        if self.n_colors + 1 > self.allocated {
            self.grow()?;
        }

        if let Some(colorant) = colorant {
            for i in 0..self.colorant_count {
                self.list[self.n_colors].device_colorant[i] = colorant[i];
            }
        }

        if let Some(pcs) = pcs {
            for i in 0..3 {
                self.list[self.n_colors].pcs[i] = pcs[i];
            }
        }

        let mut name = strip_non_ascii(name);
        name.truncate(32);
        self.list[self.n_colors].name = name;

        self.n_colors += 1;

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.n_colors
    }

    pub fn find(&self, name: &str) -> Option<usize> {
        let mut name = strip_non_ascii(name);
        name.truncate(32);

        for i in 0..self.len() {
            if name.cmp(&self.list[i].name).is_eq() {
                return Some(i);
            }
        }

        None
    }
}

impl Index<usize> for NamedColor {
    type Output = NamedColorEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}

fn strip_non_ascii(s: &str) -> AsciiString {
    let mut result = AsciiString::with_capacity(s.len());
    for c in s.chars().map(|c| if c <= 127 as char { c } else { '?' }) {
        // c cannot be a non_ascii character as it would have been changed to '?'!
        result.push(unsafe { AsciiChar::from_ascii_unchecked(c as u8) })
    }

    result
}
