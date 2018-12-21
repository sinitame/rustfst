use crate::algorithms;
use crate::arc::Arc;
use crate::fst_traits::{CoreFst, ExpandedFst, Fst};
use crate::{Result, StateId};
use std::collections::HashMap;

/// Trait defining the methods to modify a wFST
pub trait MutableFst: Fst + for<'a> MutableArcIterator<'a> {
    /// Creates an empty wFST
    fn new() -> Self;

    /// The state with identifier `state_id` is now the start state.
    /// Note that only one start state is allowed in this implementation. Calling this function twice
    /// will mean losing the first start state.
    /// If the `state_id` doesn't exist an error is raised.
    ///
    /// ```
    /// use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst};
    /// use rustfst::fst_impls::VectorFst;
    /// use rustfst::semirings::{BooleanWeight, Semiring};
    /// use rustfst::arc::Arc;
    ///
    /// let mut fst = VectorFst::<BooleanWeight>::new();
    /// let s1 = fst.add_state();
    /// let s2 = fst.add_state();
    ///
    /// assert_eq!(fst.start(), None);
    ///
    /// fst.set_start(s1);
    /// assert_eq!(fst.start(), Some(s1));
    ///
    /// fst.set_start(s2);
    /// assert_eq!(fst.start(), Some(s2));
    /// ```
    fn set_start(&mut self, state_id: StateId) -> Result<()>;

    /// The state with identifier `state_id` is now a final state with a weight `final_weight`.
    /// If the `state_id` doesn't exist an error is raised.
    ///
    /// ```
    /// use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst};
    /// use rustfst::fst_impls::VectorFst;
    /// use rustfst::semirings::{BooleanWeight, Semiring};
    /// use rustfst::arc::Arc;
    ///
    /// let mut fst = VectorFst::<BooleanWeight>::new();
    /// let s1 = fst.add_state();
    /// let s2 = fst.add_state();
    ///
    /// assert_eq!(fst.final_weight(s1), None);
    /// assert_eq!(fst.final_weight(s2), None);
    ///
    /// fst.set_final(s1, BooleanWeight::one());
    /// assert_eq!(fst.final_weight(s1), Some(BooleanWeight::one()));
    /// assert_eq!(fst.final_weight(s2), None);
    ///
    /// fst.set_final(s2, BooleanWeight::one());
    /// assert_eq!(fst.final_weight(s1), Some(BooleanWeight::one()));
    /// assert_eq!(fst.final_weight(s2), Some(BooleanWeight::one()));
    /// ```
    fn set_final(&mut self, state_id: StateId, final_weight: <Self as CoreFst>::W) -> Result<()>;

    /// Adds a new state to the current FST. The identifier of the new state is returned
    ///
    /// # Example
    ///
    /// ```
    /// use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst};
    /// use rustfst::fst_impls::VectorFst;
    /// use rustfst::semirings::{BooleanWeight, Semiring};
    ///
    /// let mut fst = VectorFst::<BooleanWeight>::new();
    ///
    /// assert_eq!(fst.num_states(), 0);
    ///
    /// fst.add_state();
    /// assert_eq!(fst.num_states(), 1);
    ///
    /// fst.add_state();
    /// assert_eq!(fst.num_states(), 2);
    ///
    /// ```
    fn add_state(&mut self) -> StateId;

    /// Removes a state from an FST. It also removes all the arcs starting from another state and
    /// reaching this state. An error is raised if the state `state_id` doesn't exist.
    ///
    /// # Example
    ///
    /// ```
    /// use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst, StateIterator};
    /// use rustfst::fst_impls::VectorFst;
    /// use rustfst::semirings::{BooleanWeight, Semiring};
    ///
    /// let mut fst = VectorFst::<BooleanWeight>::new();
    ///
    /// assert_eq!(fst.states_iter().count(), 0);
    ///
    /// let s1 = fst.add_state();
    ///
    /// assert_eq!(fst.states_iter().count(), 1);
    ///
    /// fst.del_state(s1);
    ///
    /// assert_eq!(fst.states_iter().count(), 0);
    ///
    /// ```
    fn del_state(&mut self, state_id: StateId) -> Result<()>;

    /// Removes multiple states from an FST. If one of the states doesn't exist, an error is raised.
    ///
    /// # Warning
    ///
    /// This method modifies the id of the states that are left in the FST. Id that were used before
    /// calling this function should no longer be used.
    ///
    /// # Example
    ///
    /// ```
    /// use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst, StateIterator};
    /// use rustfst::fst_impls::VectorFst;
    /// use rustfst::semirings::{BooleanWeight, Semiring};
    ///
    /// let mut fst = VectorFst::<BooleanWeight>::new();
    ///
    /// assert_eq!(fst.states_iter().count(), 0);
    ///
    /// let s1 = fst.add_state();
    /// let s2 = fst.add_state();
    ///
    /// assert_eq!(fst.states_iter().count(), 2);
    ///
    /// let states_to_remove = vec![s1, s2];
    /// fst.del_states(states_to_remove.into_iter());
    ///
    /// assert_eq!(fst.states_iter().count(), 0);
    ///
    /// ```
    fn del_states<T: IntoIterator<Item = StateId>>(&mut self, states: T) -> Result<()>;

    /// Adds an arc to the FST. The arc will start in the state `source`.
    /// An error is raised if the state `source` doesn't exist.
    ///
    /// # Warning
    ///
    /// This method modifies the id of the states that are left in the FST. Id that were used before
    /// calling this function should no longer be used.
    ///
    /// # Example
    ///
    /// ```
    /// use rustfst::fst_traits::{CoreFst, MutableFst, ExpandedFst};
    /// use rustfst::fst_impls::VectorFst;
    /// use rustfst::semirings::{BooleanWeight, Semiring};
    /// use rustfst::arc::Arc;
    ///
    /// let mut fst = VectorFst::<BooleanWeight>::new();
    /// let s1 = fst.add_state();
    /// let s2 = fst.add_state();
    ///
    /// assert_eq!(fst.num_arcs(), 0);
    /// fst.add_arc(s1, Arc::new(3, 5, BooleanWeight::new(true), s2));
    /// assert_eq!(fst.num_arcs(), 1);
    /// ```
    fn add_arc(&mut self, source: StateId, arc: Arc<<Self as CoreFst>::W>) -> Result<()>;

    fn add_fst<F: ExpandedFst<W = Self::W>>(
        &mut self,
        fst_to_add: &F,
    ) -> Result<HashMap<StateId, StateId>> {
        // Map old states id to new ones
        let mut mapping_states = HashMap::new();

        // First pass to add the necessary states
        for old_state_id in fst_to_add.states_iter() {
            let new_state_id = self.add_state();
            mapping_states.insert(old_state_id, new_state_id);
        }

        // Second pass to add the arcs
        for old_state_id in fst_to_add.states_iter() {
            for old_arc in fst_to_add.arcs_iter(old_state_id)? {
                self.add_arc(
                    mapping_states[&old_state_id],
                    Arc::new(
                        old_arc.ilabel,
                        old_arc.olabel,
                        old_arc.weight.clone(),
                        mapping_states[&old_arc.nextstate],
                    ),
                )?;
            }
        }

        Ok(mapping_states)
    }

    /// This operation computes the concatenative closure.
    /// If A transduces string `x` to `y` with weight `a`,
    /// then the closure transduces `x` to `y` with weight `a`,
    /// `xx` to `yy` with weight `a ⊗ a`, `xxx` to `yyy` with weight `a ⊗ a ⊗ a`, etc.
    fn closure_plus(&mut self) -> Result<()> {
        algorithms::closure_plus(self)
    }

    /// This operation computes the concatenative closure.
    /// If A transduces string `x` to `y` with weight `a`,
    /// then the closure transduces `x` to `y` with weight `a`,
    /// `xx` to `yy` with weight `a ⊗ a`, `xxx` to `yyy` with weight `a ⊗ a ⊗ a`, etc.
    /// The empty string is transduced to itself with weight `1` as well.
    fn closure_star(&mut self) -> Result<()> {
        algorithms::closure_star(self)
    }
}

pub trait MutableArcIterator<'a>: CoreFst
where
    Self::W: 'a,
{
    type IterMut: Iterator<Item = &'a mut Arc<Self::W>>;
    fn arcs_iter_mut(&'a mut self, state_id: StateId) -> Result<Self::IterMut>;
}
