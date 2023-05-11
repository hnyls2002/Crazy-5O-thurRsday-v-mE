use core::{cmp::min, fmt::Debug};

use alloc::collections::BTreeMap;
use bitflags::bitflags;

use crate::{
    config::{TRAMPOLINE_VIRT_ADDR, VIRT_ADDR_MAX},
    trap::trampoline_frame,
};

use super::{frame_alloc, Frame, FrameTracker, Page, VARange, VPRange, VirtAddr};

bitflags! {
    pub struct MapPerm : usize{
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

pub enum MapType {
    Identical,
    Framed(BTreeMap<Page, FrameTracker>),
    Target(Frame),
}

impl Debug for MapType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Identical => write!(f, "Identical"),
            Self::Framed(_) => write!(f, "Framed"),
            Self::Target(_) => write!(f, "Target"),
        }
    }
}

pub struct MapArea {
    pub vp_range: VPRange,
    pub map_perm: MapPerm,
    pub map_type: MapType,
}

impl Debug for MapArea {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MapArea")
            .field("vp_range", &self.vp_range)
            .field("map_perm", &self.map_perm)
            .field("map_type", &self.map_type)
            .finish()
    }
}

impl PartialEq for MapArea {
    fn eq(&self, other: &Self) -> bool {
        self.vp_range == other.vp_range
    }
}

impl MapArea {
    /// ### no bounded to physical frames
    fn new_bare(vp_range: VPRange, map_type: MapType, map_perm: MapPerm) -> Self {
        // trace!("new_bare: vp_range={:x?}", vp_range);
        MapArea {
            vp_range,
            map_perm,
            map_type,
        }
    }

    /// #### bound each page to a physical frame
    /// then this map_area can manage the physical frames
    /// frames being allocated in this function
    fn bound_frames(&mut self) {
        if let MapType::Framed(ref mut mem_src) = self.map_type {
            for it in self.vp_range.iter() {
                let page = it.value();
                let frame = frame_alloc().unwrap();
                mem_src.insert(page, frame);
            }
        } else {
            panic!("map_type is not Framed when binding frames");
        }
    }

    /// assume that start and end are not aligned
    /// data's va_range can be smaller than map_area's va_range
    fn fill_with_data(&mut self, fill_data: FillData) {
        let mut cur_va = fill_data.fill_va_range.start;
        let mut cur_offset = 0 as usize;

        while cur_va < fill_data.fill_va_range.end {
            // get the current slice len in this page
            let cur_slice_len = min(
                cur_va.floor_page().next_page().0,
                fill_data.fill_va_range.end.0,
            ) - cur_va.0;

            let src = &fill_data.data[cur_offset..cur_offset + cur_slice_len];
            let dst = &mut self.mapped_to(cur_va.floor_page()).get_bytes_array_mut()
                [cur_va.get_offset()..cur_va.get_offset() + cur_slice_len];

            dst.copy_from_slice(src);

            cur_va.0 += cur_slice_len;
            cur_offset += cur_slice_len;
        }
    }
}

impl MapArea {
    pub fn new(
        vp_range: VPRange,
        map_type: MapType,
        map_perm: MapPerm,
        fill_data: Option<FillData>,
    ) -> Self {
        // debug!("new: vp_range={:#X?}", vp_range);
        // debug!("new: map_type={:#X?}", map_type);
        // debug!("new: map_perm={:#X?}", map_perm);
        let mut ret = Self::new_bare(vp_range, map_type, map_perm);
        if let MapType::Framed(_) = ret.map_type {
            ret.bound_frames();
        }
        if let Some(data) = fill_data {
            // trace!("filling data");
            ret.fill_with_data(data);
        }
        ret
    }

    /// ### get the physical frame of a virtual page
    /// 1. if map_type is identical, then vp == pp
    /// 2. if map_type is framed, then vp is the key of `mem_frames` : we assume that this pp has been allocated before
    pub fn mapped_to(&self, vp: Page) -> Frame {
        match self.map_type {
            MapType::Identical => vp.into(),
            MapType::Framed(ref mem_frames) => mem_frames.get(&vp).expect("frame not found").0,
            MapType::Target(frame) => frame,
        }
    }
}

pub struct FillData<'a> {
    pub fill_va_range: VARange,
    pub data: &'a [u8],
}

impl<'a> FillData<'a> {
    pub fn new(start_va: VirtAddr, end_va: VirtAddr, data: &'a [u8]) -> Self {
        FillData {
            fill_va_range: VARange::new(start_va, end_va),
            data,
        }
    }
}

impl MapArea {
    /// create a trampoline map_area
    pub fn new_trampoline() -> Self {
        Self::new(
            VPRange::new(TRAMPOLINE_VIRT_ADDR, VIRT_ADDR_MAX),
            MapType::Target(trampoline_frame()),
            MapPerm::X | MapPerm::R,
            None,
        )
    }
}
