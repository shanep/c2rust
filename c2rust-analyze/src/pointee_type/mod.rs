use crate::context::AnalysisCtxt;
use crate::pointer_id::{
    GlobalPointerTable, LocalPointerTable, NextGlobalPointerId, NextLocalPointerId, PointerId,
    PointerTable,
};
use rustc_middle::mir::Body;
use std::mem;

mod constraint_set;
mod solve;
mod type_check;

pub use self::constraint_set::{CTy, Constraint, ConstraintSet, VarTable};
pub use self::solve::{solve_constraints, PointeeTypes};

pub fn generate_constraints<'tcx>(
    acx: &AnalysisCtxt<'_, 'tcx>,
    mir: &Body<'tcx>,
    vars: &mut VarTable<'tcx>,
) -> ConstraintSet<'tcx> {
    type_check::visit(acx, mir, vars)
}
