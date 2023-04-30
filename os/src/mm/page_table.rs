use alloc::vec::Vec;
use bitflags::bitflags;
use riscv::addr::BitField;

use crate::config::{PAGE_SIZE_BITS, PPN_RANGE};

use super::{frame_alloc, Frame, FrameTracker, Page};

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
    pub fn bound_frame(&mut self, phys_page: Frame, flags: PTEFlags) {
        self.0.set_bits(PPN_RANGE, phys_page.get_ppn());
        self.set_flags(flags)
    }

    pub fn get_frame(&self) -> Frame {
        Frame(self.0.get_bits(PPN_RANGE) << PAGE_SIZE_BITS)
    }

    pub fn get_flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.0).unwrap()
    }

    pub fn set_flags(&mut self, flags: PTEFlags) {
        self.0 |= flags.bits;
    }

    pub fn clear_flags(&mut self, flags: PTEFlags) {
        self.0 &= !flags.bits
    }

    // some methods for convinence
    pub fn is_valid(&self) -> bool {
        (self.get_flags() & PTEFlags::V) != PTEFlags::empty()
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
    pub fn map_one(&mut self, vp: Page, pp: Frame, flags: PTEFlags) -> Result<(), ()> {
        let mut cur_frame = self.entry.clone();
        let indices = vp.get_indices();

        for i in 0..2 {
            let pte = &mut cur_frame.get_pte_array_mut()[indices[i]];
            // not valid, create a new page table
            if !pte.is_valid() {
                let new_frame = frame_alloc().unwrap();
                pte.bound_frame(new_frame.0, PTEFlags::V);
                self.pt_frames.push(new_frame);
            }
            cur_frame = pte.get_frame();
        }

        let last_pte = &mut cur_frame.get_pte_array_mut()[indices[2]];
        if last_pte.is_valid() {
            return Err(());
        }
        last_pte.bound_frame(pp, flags | PTEFlags::V);
        Ok(())
    }
    pub fn unmap_one(&mut self, vp: Page) -> Result<(), ()> {
        let mut cur_frame = self.entry.clone();
        let indices = vp.get_indices();
        for i in 0..2 {
            let pte = &cur_frame.get_pte_array_mut()[indices[i]];
            cur_frame = pte.get_frame();
        }
        let last_pte = &mut cur_frame.get_pte_array_mut()[indices[2]];
        if last_pte.is_valid() {
            last_pte.clear_flags(PTEFlags::V);
            Ok(())
        } else {
            Err(())
        }
    }
}
