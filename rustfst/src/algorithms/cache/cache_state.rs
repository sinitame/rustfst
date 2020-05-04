use std::slice::Iter as IterSlice;
use std::slice::IterMut as IterSliceMut;

use crate::Tr;

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq)]
pub struct CacheState<W> {
    trs: Vec<Tr<W>>,
    final_weight: Option<W>,
    expanded: bool,
    has_final: bool,
}

impl<W> CacheState<W> {
    pub fn new() -> Self {
        Self {
            trs: Vec::new(),
            final_weight: None,
            expanded: false,
            has_final: false,
        }
    }

    pub fn has_final(&self) -> bool {
        self.has_final
    }

    pub fn expanded(&self) -> bool {
        self.expanded
    }

    pub fn mark_expanded(&mut self) {
        self.expanded = true;
    }

    pub fn set_final_weight(&mut self, final_weight: Option<W>) {
        self.final_weight = final_weight;
        self.has_final = true;
    }

    pub fn final_weight(&self) -> Option<&W> {
        self.final_weight.as_ref()
    }

    pub fn push_tr(&mut self, tr: Tr<W>) {
        self.trs.push(tr);
    }

    pub fn reserve_trs(&mut self, n: usize) {
        self.trs.reserve(n);
    }

    pub fn num_trs(&self) -> usize {
        self.trs.len()
    }

    pub fn get_tr_unchecked(&self, n: usize) -> &Tr<W> {
        unsafe { self.trs.get_unchecked(n) }
    }

    pub fn get_tr_unchecked_mut(&mut self, n: usize) -> &mut Tr<W> {
        unsafe { self.trs.get_unchecked_mut(n) }
    }

    pub fn tr_iter(&self) -> IterSlice<Tr<W>> {
        self.trs.iter()
    }

    pub fn tr_iter_mut(&mut self) -> IterSliceMut<Tr<W>> {
        self.trs.iter_mut()
    }
}
