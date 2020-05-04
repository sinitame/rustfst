use std::iter::Enumerate;
use std::iter::Map;
use std::iter::Skip;
use std::iter::Take;
use std::iter::Zip;
use std::ops::Range;
use std::slice;

use anyhow::Result;
use itertools::Itertools;
use itertools::{izip, repeat_n, RepeatN};

use crate::fst_impls::const_fst::data_structure::ConstState;
use crate::fst_impls::ConstFst;
use crate::fst_traits::FstIterData;
use crate::fst_traits::{FstIntoIterator, FstIterator, StateIterator, TrIterator};
use crate::semirings::Semiring;
use crate::StateId;
use crate::Tr;

impl<W> ConstFst<W> {
    fn state_range(&self) -> Range<usize> {
        0..self.states.len()
    }

    fn tr_range(&self, state: &ConstState<W>) -> Range<usize> {
        state.pos..state.pos + state.narcs
    }
}

impl<'a, W: 'static + Semiring> TrIterator<'a> for ConstFst<W> {
    type Iter = slice::Iter<'a, Tr<W>>;
    fn tr_iter(&'a self, state_id: StateId) -> Result<Self::Iter> {
        let state = self
            .states
            .get(state_id)
            .ok_or_else(|| format_err!("State {:?} doesn't exist", state_id))?;
        Ok(self.trs[self.tr_range(state)].iter())
    }

    unsafe fn tr_iter_unchecked(&'a self, state_id: usize) -> Self::Iter {
        let state = self.states.get_unchecked(state_id);
        self.trs[self.tr_range(state)].iter()
    }
}

impl<W: Semiring> FstIntoIterator for ConstFst<W>
where
    W: 'static,
{
    type TrsIter = std::vec::IntoIter<Tr<W>>;

    // TODO: Change this to impl once the feature has been stabilized
    // #![feature(type_alias_impl_trait)]
    // https://github.com/rust-lang/rust/issues/63063)
    type FstIter = Box<dyn Iterator<Item = FstIterData<W, Self::TrsIter>>>;

    fn fst_into_iter(mut self) -> Self::FstIter {
        // Here the contiguous trs are moved into multiple vectors in order to be able to create
        // iterator for each states.
        // TODO: Find a way to avoid this allocation.
        let mut trs = Vec::with_capacity(self.states.len());
        for const_state in &self.states {
            trs.push(self.trs.drain(0..const_state.narcs).collect_vec())
        }

        Box::new(
            izip!(self.states.into_iter(), trs.into_iter())
                .enumerate()
                .map(|(state_id, (const_state, arcs_from_state))| FstIterData {
                    state_id,
                    trs: arcs_from_state.into_iter(),
                    final_weight: const_state.final_weight,
                    num_trs: const_state.narcs,
                }),
        )
    }
}

impl<'a, W> StateIterator<'a> for ConstFst<W> {
    type Iter = Range<StateId>;
    fn states_iter(&'a self) -> Self::Iter {
        self.state_range()
    }
}

impl<'a, W: Semiring + 'static> FstIterator<'a> for ConstFst<W> {
    type TrsIter = Take<Skip<std::slice::Iter<'a, Tr<W>>>>;
    type FstIter = Map<
        Enumerate<Zip<std::slice::Iter<'a, ConstState<W>>, RepeatN<&'a Vec<Tr<W>>>>>,
        Box<
            dyn FnMut(
                (StateId, (&'a ConstState<W>, &'a Vec<Tr<W>>)),
            ) -> FstIterData<&'a W, Self::TrsIter>,
        >,
    >;
    fn fst_iter(&'a self) -> Self::FstIter {
        let it = repeat_n(&self.trs, self.states.len());
        izip!(self.states.iter(), it).enumerate().map(Box::new(
            |(state_id, (fst_state, trs)): (StateId, (&'a ConstState<W>, &'a Vec<Tr<W>>))| {
                FstIterData {
                    state_id,
                    trs: trs.iter().skip(fst_state.pos).take(fst_state.narcs),
                    final_weight: fst_state.final_weight.as_ref(),
                    num_trs: fst_state.narcs,
                }
            },
        ))
    }
}
