use core::fmt::Debug;

use alloc::collections::BTreeMap;
use bitflags::bitflags;

use super::{frame_alloc, Frame, FrameTracker, Page, VPRange};

bitflags! {
    pub struct MapPerm : usize{
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

#[derive(PartialEq, Eq)]
pub enum MapType {
    Identical,
    Framed,
    Linear,
}

impl Debug for MapType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Identical => write!(f, "Identical"),
            Self::Framed => write!(f, "Framed"),
            Self::Linear => write!(f, "Linear"),
        }
    }
}

pub struct MapArea {
    pub vp_range: VPRange,
    pub mem_frames: BTreeMap<Page, FrameTracker>,
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
    pub fn new_bare(vp_range: VPRange, map_type: MapType, map_perm: MapPerm) -> Self {
        MapArea {
            mem_frames: BTreeMap::new(),
            vp_range,
            map_perm,
            map_type,
        }
    }
    /// #### bound each page to a physical frame
    /// then this map_area can manage the physical frames
    pub fn bound_frames(&mut self) {
        assert!(self.mem_frames.is_empty(), "mem_frames is not empty");
        for it in self.vp_range.iter() {
            let page = it.value();
            let frame = frame_alloc().unwrap();
            self.mem_frames.insert(page, frame);
        }
    }

    pub fn get_framed(&self, vp: Page) -> Frame {
        match self.map_type {
            MapType::Identical => vp.into(),
            MapType::Framed => self.mem_frames.get(&vp).expect("frame not found").0,
            MapType::Linear => vp.into(), // TODO : linear mapping
        }
    }
}
