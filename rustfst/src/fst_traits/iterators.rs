use anyhow::Result;

use crate::fst_traits::CoreFst;
use crate::tr::Tr;
use crate::StateId;

/// Trait to iterate over the states of a wFST.
pub trait StateIterator<'a> {
    /// Iterator used to iterate over the `state_id` of the states of an FST.
    type Iter: Iterator<Item = StateId>;

    /// Creates an iterator over the `state_id` of the states of an FST.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst, StateIterator};
    /// # use rustfst::fst_impls::VectorFst;
    /// # use rustfst::semirings::{BooleanWeight, Semiring};
    /// let mut fst = VectorFst::<BooleanWeight>::new();
    ///
    /// let s1 = fst.add_state();
    /// let s2 = fst.add_state();
    ///
    /// for state_id in fst.states_iter() {
    ///     println!("State ID : {:?}", state_id);
    /// }
    ///
    /// let states : Vec<_> = fst.states_iter().collect();
    /// assert_eq!(states, vec![s1, s2]);
    /// ```
    fn states_iter(&'a self) -> Self::Iter;
}

/// Trait to iterate over the outgoing trs of a particular state in a wFST
pub trait TrIterator<'a>: CoreFst
where
    Self::W: 'a,
{
    /// Iterator used to iterate over the trs leaving a state of an FST.
    type Iter: Iterator<Item = &'a Tr<Self::W>> + Clone;

    fn tr_iter(&'a self, state_id: StateId) -> Result<Self::Iter>;
    unsafe fn tr_iter_unchecked(&'a self, state_id: StateId) -> Self::Iter;
}

pub struct FstIterData<W, I> {
    pub state_id: StateId,
    pub final_weight: Option<W>,
    pub trs: I,
    pub num_trs: usize,
}

pub trait FstIntoIterator: CoreFst {
    type TrsIter: Iterator<Item = Tr<Self::W>>;
    type FstIter: Iterator<Item = FstIterData<Self::W, Self::TrsIter>>;
    fn fst_into_iter(self) -> Self::FstIter;
}

pub trait FstIterator<'a>: CoreFst
where
    Self::W: 'a,
{
    type TrsIter: Iterator<Item = &'a Tr<Self::W>>;
    type FstIter: Iterator<Item = FstIterData<&'a Self::W, Self::TrsIter>>;
    fn fst_iter(&'a self) -> Self::FstIter;
}

pub trait FstIteratorMut<'a>: CoreFst
where
    Self::W: 'a,
{
    type TrsIter: Iterator<Item = &'a mut Tr<Self::W>>;
    type FstIter: Iterator<Item = FstIterData<&'a mut Self::W, Self::TrsIter>>;
    fn fst_iter_mut(&'a mut self) -> Self::FstIter;
}
