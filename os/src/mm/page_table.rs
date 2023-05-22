use core::cmp::min;

use alloc::{string::String, vec::Vec};
use bitflags::bitflags;
use riscv::addr::BitField;

use crate::config::{PAGE_BYTES_BITS, PTE_FLAGS_MASK, PTE_PPN_RANGE};

use super::{frame_alloc, Frame, FrameTracker, Page, VirtAddr};

bitflags! {
    pub struct PTEFlags : usize{
        const V = 1 << 0; // valid
        const R = 1 << 1; // readable
        const W = 1 << 2; // writable
        const X = 1 << 3; // executable
        const U = 1 << 4; // user
        const G = 1 << 5; // global
        const A = 1 << 6; // accessed
        const D = 1 << 7; // dirty
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PTE(usize);

impl PTE {
    pub fn map_frame(&mut self, phys_page: Frame, flags: PTEFlags) {
        self.0.set_bits(PTE_PPN_RANGE, phys_page.get_ppn());
        self.set_flags(flags);
    }

    pub fn get_frame(&self) -> Frame {
        Frame(self.0.get_bits(PTE_PPN_RANGE) << PAGE_BYTES_BITS)
    }

    pub fn get_flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.0 & PTE_FLAGS_MASK).unwrap()
    }

    pub fn set_flags(&mut self, flags: PTEFlags) {
        // warn!("set_flags: {:#x?}", flags);
        self.0 |= flags.bits;
        // warn!("bits : {:#x?}", self.0);
        // warn!("get_flags: {:#x?}", self.get_flags());
    }

    pub fn clear_flags(&mut self, flags: PTEFlags) {
        self.0 &= !flags.bits
    }

    // some methods for convinence
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        (self.get_flags() & PTEFlags::V) != PTEFlags::empty()
    }

    #[allow(dead_code)]
    pub fn is_readable(&self) -> bool {
        (self.get_flags() & PTEFlags::R) != PTEFlags::empty()
    }

    #[allow(dead_code)]
    pub fn is_writable(&self) -> bool {
        (self.get_flags() & PTEFlags::W) != PTEFlags::empty()
    }

    #[allow(dead_code)]
    pub fn is_executable(&self) -> bool {
        (self.get_flags() & PTEFlags::X) != PTEFlags::empty()
    }

    #[allow(dead_code)]
    pub fn is_user(&self) -> bool {
        (self.get_flags() & PTEFlags::U) != PTEFlags::empty()
    }
}

pub struct PageTable {
    pub entry: Frame,
    // all the frames are stored in nodes including root
    pub pt_frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let rt_ft = frame_alloc().unwrap();
        let rt_frame = rt_ft.0;
        let mut srcs = Vec::new();
        srcs.push(rt_ft);
        PageTable {
            entry: rt_frame,
            pt_frames: srcs,
        }
    }

    pub fn find_create_pte_mut(&mut self, vp: Page) -> Option<&'static mut PTE> {
        let mut cur_frame = self.entry.clone();
        let indices = vp.get_indices();

        // debug!("find_create_pte_mut: {:x?}", indices);
        for i in 0..2 {
            let pte = &mut cur_frame.get_pte_array_mut()[indices[i]];
            // not valid, create a new page table
            // debug!("cur frame: {:X?}", cur_frame);
            if !pte.is_valid() {
                let new_frame = frame_alloc().unwrap();
                // debug!("new frame: {:X?}", new_frame.0);
                pte.map_frame(new_frame.0, PTEFlags::V);
                // debug!("valid after map : {:?}", pte.is_valid());
                self.pt_frames.push(new_frame);
            }
            cur_frame = pte.get_frame();
        }

        let last_pte = &mut cur_frame.get_pte_array_mut()[indices[2]];
        Some(last_pte)
    }

    pub fn find_pte_mut(&self, vp: Page) -> Option<&'static mut PTE> {
        let mut cur_frame = self.entry.clone();
        let indices = vp.get_indices();
        for i in 0..2 {
            let pte = &cur_frame.get_pte_array_mut()[indices[i]];
            if !pte.is_valid() {
                return None;
            }
            cur_frame = pte.get_frame();
        }
        let last_pte = &mut cur_frame.get_pte_array_mut()[indices[2]];
        Some(last_pte)
    }

    pub fn find_pte(&self, vp: Page) -> Option<&'static PTE> {
        self.find_pte_mut(vp).map_or(None, |pte| Some(pte))
    }

    pub fn map_one(&mut self, vp: Page, pp: Frame, flags: PTEFlags) -> Result<(), ()> {
        // debug!("map_one: {:x?} {:x?} {:x?}", vp, pp, flags);
        let pte = self.find_create_pte_mut(vp).unwrap();
        if pte.is_valid() {
            Err(())
        } else {
            pte.map_frame(pp, flags | PTEFlags::V);
            Ok(())
        }
    }

    pub fn unmap_one(&mut self, vp: Page) -> Result<(), ()> {
        let pte = self.find_pte_mut(vp).unwrap();
        if pte.is_valid() {
            pte.clear_flags(PTEFlags::V);
            Ok(())
        } else {
            Err(())
        }
    }
}

impl PageTable {
    pub fn satp_token(pt_entry: Frame) -> usize {
        8usize << 60 | pt_entry.get_ppn()
    }

    pub fn translate_vp(&self, vp: Page) -> Option<Frame> {
        let pte = self.find_pte(vp)?;
        if pte.is_valid() {
            Some(pte.get_frame())
        } else {
            None
        }
    }

    pub fn translate_str(&self, start: *const u8) -> Option<String> {
        let mut ret = String::new();
        let mut ptr = start;
        loop {
            let va = VirtAddr(ptr as usize);
            let pp = self.translate_vp(va.floor_page())?;
            let c = pp.get_bytes_array_mut()[va.get_offset()];
            match c {
                0 => break,
                _ => ret.push(c as char),
            }
            ptr = unsafe { ptr.add(1) };
        }
        Some(ret)
    }

    pub fn translate_byte_buffer_mut(
        &self,
        buf: usize,
        len: usize,
    ) -> Option<Vec<&'static mut [u8]>> {
        let mut ret = Vec::new();
        let mut cur_va = VirtAddr(buf);
        let mut rem_len = len;
        while rem_len > 0 {
            let cur_slice_len = min(cur_va.floor_page().next_page().0 - cur_va.0, rem_len);
            let cur_frame = self.translate_vp(cur_va.floor_page())?;
            let slice = &mut cur_frame.get_bytes_array_mut()
                [cur_va.get_offset()..cur_va.get_offset() + cur_slice_len];
            ret.push(slice);
            cur_va.0 += cur_slice_len;
            rem_len -= cur_slice_len;
        }
        Some(ret)
    }
}
