use std::mem::size_of;

use codepage::to_encoding;
use encoding_rs::mem::{convert_utf8_to_utf16, convert_utf16_to_utf8};

use crate::{plugin::adjust_endianess16, Context};

use super::Dup;

pub struct MLU {
    context_id: &'static Context,
    allocated_entries: usize,
    used_entries: usize,
    entries: Vec<Entry>,
    pool_size: usize,
    pool_used: usize,
    mem_pool: Vec<u16>,
}

#[derive(Clone)]
struct Entry {
    pub language: u16,
    pub country: u16,
    pub str_w: usize,
    pub len: usize,
}

impl<'a> MLU {
    pub fn new(context_id: &'static Context, n_items: usize) -> MLU {
        MLU {
            context_id,
            allocated_entries: n_items,
            used_entries: 0,
            entries: Vec::<Entry>::with_capacity(n_items as usize),
            pool_size: 0,
            pool_used: 0,
            mem_pool: Vec::<u16>::new(),
        }
    }

    fn grow_pool(&mut self) -> Result<(), String> {
        let size = if self.pool_size == 0 {
            256
        } else {
            self.pool_size * 2
        };

        // Check for overflow
        if size < self.pool_size {
            return Err("Overflow in MLU::grow_pool".into());
        };

        // "Reallocate" the pool
        self.mem_pool.resize(size as usize, 0);
        self.pool_size = size;

        Ok(())
    }

    fn grow_table(&mut self) -> Result<(), String> {
        let alloc = self.allocated_entries * 2;

        // Check for overflow
        if alloc / 2 != self.allocated_entries {
            return Err("Overflow in MLU::grow_table".into());
        }

        self.entries.resize(
            alloc as usize,
            Entry {
                language: 0,
                country: 0,
                str_w: 0,
                len: 0,
            },
        );
        self.allocated_entries = alloc;

        Ok(())
    }

    fn search_entry(&self, lang_code: u16, cntr_code: u16) -> i32 {
        // Iterate whole table
        for i in 0..self.used_entries {
            if self.entries[i as usize].country == cntr_code
                && self.entries[i as usize].language == lang_code
            {
                return i as i32;
            }
        }

        // Not found
        -1
    }

    fn add_block(&mut self, block: &[u16], lang_code: u16, cntr_code: u16) -> Result<(), String> {
        let size = block.len();

        // Is there any room available?
        if self.used_entries >= self.allocated_entries {
            let result = self.grow_table();
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        // Only one ASCII string
        if self.search_entry(lang_code, cntr_code) >= 0 {
            return Err("ASCII string already exists in MLU::add_block".into());
        }

        // Check for size
        while ((self.pool_size - self.pool_used) as usize) < ((size + 1) * size_of::<u16>()) {
            let result = self.grow_pool();
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        let offset = self.pool_used;

        let ptr = &mut self.mem_pool[(offset as usize)..];

        // Set the entry
        ptr.copy_from_slice(&block);
        ptr[size as usize] = 0;
        self.pool_used += size * size_of::<u16>();

        self.entries.push(Entry {
            language: lang_code,
            country: cntr_code,
            str_w: offset * size_of::<u16>(),
            len: (size * size_of::<u16>()),
        });

        Ok(())
    }

    pub fn set_ascii(
        &mut self,
        lang_code: [u8; 2],
        cntr_code: [u8; 2],
        ascii_str: &[u8],
    ) -> Result<(), String> {
        let len = ascii_str.len();
        let lang = str_to_16(&lang_code);
        let cntr = str_to_16(&cntr_code);

        // len == 0 would prevent operation, so we set an empth string pointing to zero
        if len == 0 {
            let block = [0u16];
            return self.add_block(&block, lang, cntr);
        }

        let encoding = to_encoding(437).unwrap();
        let (utf8_str, _, _) = encoding.decode(ascii_str);

        let mut w_str = vec![0 as u16; len + 1];

        let chars_written = convert_utf8_to_utf16(utf8_str.as_bytes(), &mut w_str);
        w_str.truncate(chars_written);

        self.add_block(&w_str.as_slice(), lang, cntr)
    }

    pub fn set_wide(
        &mut self,
        lang_code: [u8; 2],
        cntr_code: [u8; 2],
        wide_str: &[u16],
    ) -> Result<(), String> {
        let len = wide_str.len();
        let lang = str_to_16(&lang_code);
        let cntr = str_to_16(&cntr_code);

        // len == 0 would prevent operation, so we set an empth string pointing to zero
        if len == 0 {
            let wide_str = [0u16];
            return self.add_block(&wide_str, lang, cntr);
        }

        self.add_block(wide_str, lang, cntr)
    }

    pub fn free(mut self) {
        self.entries.clear();

        drop(self);
    }

    pub(crate) fn _get_wide(&'a self, lang_code: u16, cntr_code: u16) -> Option<(&'a [u16], u16, u16)> {
        let mlu = self;

        let mut best = -1;

        if mlu.allocated_entries <= 0 {
            return None;
        }

        for i in 0..(mlu.used_entries as usize) {
            let v = &mlu.entries[i];

            if v.language == lang_code {
                if best == -1 {
                    best = i as i32;
                }

                if v.country == cntr_code {
                    best = i as i32;
                    break;
                }
            }
        }

        // No string found? Return first one
        if best == -1 {
            best = 0;
        }

        let v = &mlu.entries[best as usize];

        Some((
            &mlu.mem_pool[((v.str_w as usize) / size_of::<u16>())..]
                [..((v.len as usize) / size_of::<u16>())],
            v.language,
            v.country,
        ))
    }

    pub fn get_ascii(&self, lang_code: [u8; 2], cntr_code: [u8; 2], buffer: &mut [u8]) -> Option<usize> {
        let lang = str_to_16(&lang_code);
        let cntr = str_to_16(&cntr_code);
        let buf_size = buffer.len();

        // GetWideChar
        let (wide, _, _) = self._get_wide(lang, cntr)?;
        if wide.len() == 0 {
            return Some(0);
        }

        let mut utf8_str = vec![0u8; wide.len() * 3];

        let chars_written = convert_utf16_to_utf8(wide, &mut utf8_str);

        let encoding = to_encoding(437).unwrap();
        let (ascii_str, _, _) = encoding.encode(std::str::from_utf8(&utf8_str[..chars_written]).unwrap());

        let mut ascii_len = ascii_str.len();

        if buf_size == 0 {
            return Some(ascii_len);
        }

        // Some clipping may be required
        if buf_size < ascii_len {
            ascii_len = buf_size
        }

        buffer[..ascii_len].copy_from_slice(&ascii_str);

        Some(ascii_len)
    }

    pub fn get_wide(&self, lang_code: [u8; 2], cntr_code: [u8; 2], buffer: &mut [u16]) -> Option<usize> {
        let lang = str_to_16(&lang_code);
        let cntr = str_to_16(&cntr_code);
        let buf_size = buffer.len();

        // GetWideChar
        let (wide, _, _) = self._get_wide(lang, cntr)?;
        if wide.len() == 0 {
            return Some(0);
        }

        let mut wide_len = wide.len();

        if buf_size == 0 {
            return Some(wide_len);
        }

        // Some clipping may be required
        if buf_size < wide_len {
            wide_len = buf_size
        }

        buffer[..wide_len].copy_from_slice(&wide);

        Some(wide_len)
    }

    pub fn get_translation(&self, lang_code: [u8; 2], cntr_code: [u8; 2]) -> Option<([u8; 2], [u8; 2])> {
        let mlu = self;
        let lang = str_to_16(&lang_code);
        let cntr = str_to_16(&cntr_code);

        let (_, obt_lang, obt_cntr) = mlu._get_wide(lang, cntr)?;

        Some((str_from_16(obt_lang), str_from_16(obt_cntr)))
    }

    pub fn get_translations_count(&self) -> usize {
        self.used_entries
    }
}

impl Dup for MLU {
    fn dup(&self, context_id: &'static Context) -> Result<Self, String>
    where
        Self: Sized,
    {
        let mut new_mlu = Self::new(context_id, self.used_entries);

        // Should never happen
        if new_mlu.allocated_entries < self.used_entries {
            return Err("Corrupted construction of MLU in MLU::dup.".into());
        }

        for entry in &self.entries {
            new_mlu.entries.push(entry.clone());
        }
        new_mlu.mem_pool.resize(self.pool_used as usize, 0);

        new_mlu.pool_size = self.pool_size;

        new_mlu.mem_pool.copy_from_slice(&self.mem_pool);

        new_mlu.pool_used = self.pool_used;

        Ok(new_mlu)
    }
}

impl<'a> Clone for MLU {
    fn clone(&self) -> Self {
        match self.dup(self.context_id) {
            Ok(result) => result,
            Err(msg) => panic!("{}", msg),
        }
    }
}

fn str_to_16(s: &[u8]) -> u16 {
    adjust_endianess16(u16::from_ne_bytes([s[0], s[1]]))
}

fn str_from_16(n: u16) -> [u8; 2] {
    u16::to_ne_bytes(adjust_endianess16(n))
}
