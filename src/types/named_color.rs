use crate::{Context, Result, MAX_CHANNELS};

#[derive(Default, Copy, Clone)]
pub struct NamedColorEntry {
    pub name: [u8; 32],
    pub pcs: [u16; 3],
    pub device_colorant: [u16; MAX_CHANNELS],
}

pub struct NamedColor {
    pub(crate) context_id: Context,
    pub(crate) n_colors: usize,
    pub(crate) allocated: usize,
    pub(crate) colorant_count: usize,
    pub(crate) prefix: [u8; 32],
    pub(crate) suffix: [u8; 32],
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
            str_to_ascii_u8_32(prefix),
            str_to_ascii_u8_32(suffix),
        )
    }

    fn _new(
        context_id: &Context,
        n: usize,
        colorant_count: usize,
        prefix: [u8; 32],
        suffix: [u8; 32],
    ) -> Result<Self> {
        let list = Vec::<NamedColorEntry>::new();
        let n_colors = 0usize;
        let allocated = 0usize;

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
            self.prefix,
            self.suffix,
        )?;

        // For really large tables we need this
        while new_nc.allocated < self.allocated {
            new_nc.grow()?
        }

        new_nc.list[..self.list.len()].copy_from_slice(&self.list);

        Err("".into())
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

        self.list[self.n_colors].name = str_to_ascii_u8_32(name);

        self.n_colors += 1;

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.n_colors
    }

    pub fn find_index(&self, name: &str) -> Option<usize> {
        let name = strip_non_ascii(name);

        for i in 0..self.len() {
            if str::cmp(name, str_from_u8_nul_ascii(&self.list[i].name)).is_eq() {
                return Some(i);
            }
        }

        None
    }
}

fn str_from_u8_nul_ascii(utf8_src: &[u8]) -> &str {
    if !utf8_src.is_ascii() {
        return unsafe { ::std::str::from_utf8_unchecked(&utf8_src[..0]) };
    }
    let mut nul_range_end = 1usize;
    for c in utf8_src {
        if *c == 0 {
            break;
        }
        nul_range_end += 1;
    }

    unsafe { ::std::str::from_utf8_unchecked(&utf8_src[0..nul_range_end]) }
}
fn strip_non_ascii(s: &str) -> &str {
    if s.is_ascii() {
        return s;
    }

    let mut result = Vec::<u8>::with_capacity(s.len());

    for c in s.chars() {
        result.push(if c >= 128 as char { '?' } else { c } as u8);
    }

    unsafe { ::std::str::from_utf8_unchecked(&result) }
}

fn str_to_ascii_u8_32(s: &str) -> [u8; 32] {
    let mut result = [0u8; 32];

    let mut s = strip_non_ascii(s);
    let s = if s.len() > 32 { &s[..32] } else { s };

    result[..s.len()].copy_from_slice(s.as_bytes());

    result
}
