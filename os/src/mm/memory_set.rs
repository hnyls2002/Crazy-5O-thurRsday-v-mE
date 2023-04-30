use core::fmt::Debug;

use alloc::{collections::BTreeMap, vec::Vec};
use bitflags::bitflags;

use super::{frame_alloc, FrameTracker, PTEFlags, Page, PageTable, VPRange};

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
    pub source: BTreeMap<Page, FrameTracker>,
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
    pub fn new(vp_range: VPRange, map_type: MapType, map_perm: MapPerm) -> Self {
        let mut source: BTreeMap<Page, FrameTracker> = BTreeMap::new();
        for it in vp_range.iter() {
            let pp = frame_alloc().unwrap();
            let vp = it.value();
            source.insert(vp, pp);
        }
        MapArea {
            vp_range,
            source,
            map_perm,
            map_type,
        }
    }
}

pub struct MemorySet {
    pub map_areas: Vec<MapArea>,
    pub page_table: PageTable,
}

impl MemorySet {
    // only page table root is set
    pub fn new() -> Self {
        MemorySet {
            map_areas: Vec::new(),
            page_table: PageTable::new(),
        }
    }

    pub fn add_map_area(&mut self, map_area: MapArea) {
        let vp_range = &map_area.vp_range;
        let pte_flags = PTEFlags::from_bits(map_area.map_perm.bits()).unwrap();
        // TODO : PTE flags may have other flags to be set
        for it in vp_range.iter() {
            let vp = it.value();
            let pp = map_area.source.get(&vp).unwrap();
            let res = self.page_table.map_one(vp, pp.0, pte_flags);
            if let Err(_) = res {
                panic!("virtual page mapping to physical page failed");
            }
        }
        self.map_areas.push(map_area);
    }

    pub fn release_map_area(&mut self, vp_range: &VPRange) {
        let index = self
            .map_areas
            .iter()
            .position(|x| x.vp_range == *vp_range)
            .unwrap();
        let map_area = self.map_areas.remove(index);
        for it in map_area.vp_range.iter() {
            let vp = it.value();
            let res = self.page_table.unmap_one(vp);
            if let Err(()) = res {
                panic!("virtual page unmapping failed");
            }
        }
    }
}
