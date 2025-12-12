use crate::air::constraints_templates::{new_add_constraint, new_bit_constraints};

use lambdaworks_math::field::{
    element::FieldElement,
    fields::fft_friendly::{
        babybear_u32::Babybear31PrimeField, quartic_babybear_u32::Degree4BabyBearU32ExtensionField,
    },
};
use stark_platinum_prover::{
    constraints::{boundary::BoundaryConstraints, transition::TransitionConstraint},
    context::AirContext,
    proof::options::ProofOptions,
    trace::TraceTable,
    traits::AIR,
};

type FE = FieldElement<Babybear31PrimeField>;

type CPUTraceTable = TraceTable<Babybear31PrimeField, Degree4BabyBearU32ExtensionField>;

/// AIR for the `CPU` trace.
///
/// Enforces two types of constraints:
/// - **Bit constraints**: selected columns must be binary.
/// - **32-bit add constraint** (limb-decomposed): enforces `lhs + rhs = res` when the ADD instruction is executed
pub struct CPUTableAIR {
    context: AirContext,
    constraints:
        Vec<Box<dyn TransitionConstraint<Babybear31PrimeField, Degree4BabyBearU32ExtensionField>>>,
    trace_length: usize,
}

impl AIR for CPUTableAIR {
    type Field = Babybear31PrimeField;
    type FieldExtension = Degree4BabyBearU32ExtensionField;
    type PublicInputs = ();

    const STEP_SIZE: usize = 1;

    fn new(
        trace_length: usize,
        _pub_inputs: &Self::PublicInputs,
        proof_options: &ProofOptions,
    ) -> Self {
        // Bit constraints:
        // Enforce that these columns are binary. They include:
        // - decode flags like `write_register`, `signed`, `mp_selector`, `muldiv_selector`
        // - the one-hot instruction flags (ADD, SUB, ..., EBREAK)
        let bit_columns_index_to_constraint = [
            7, 8, 9, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
        ];
        let bit_constraints = new_bit_constraints(&bit_columns_index_to_constraint, 0);

        let next_index = bit_columns_index_to_constraint.len();
        // Add constraint
        // Enforces that lhs (Word4L) + rhs (Word4L) = res (Word4L), with carry bits constrained.
        // It is enforced only on rows where the selected instruction flags are active.
        let add_constraints = new_add_constraint(
            vec![15, 26, 27], // flags_idx,
            34,               // lhs_start_idx,
            44,               // rhs_start_idx,
            48,               // res_start_idx,
            next_index,       // constraint_idx_start,
        );

        let mut constraints = bit_constraints;
        constraints.extend(add_constraints);

        let num_transition_constraints = constraints.len();

        let context = AirContext {
            proof_options: proof_options.clone(),
            trace_columns: 54,
            transition_offsets: vec![0],
            num_transition_constraints,
        };

        Self {
            context,
            trace_length,
            constraints,
        }
    }

    fn transition_constraints(
        &self,
    ) -> &Vec<Box<dyn TransitionConstraint<Self::Field, Self::FieldExtension>>> {
        &self.constraints
    }

    fn boundary_constraints(
        &self,
        _rap_challenges: &[FieldElement<Self::FieldExtension>],
    ) -> BoundaryConstraints<Self::FieldExtension> {
        BoundaryConstraints::from_constraints(vec![])
    }

    fn context(&self) -> &AirContext {
        &self.context
    }

    fn composition_poly_degree_bound(&self) -> usize {
        self.trace_length * 2
    }

    fn trace_length(&self) -> usize {
        self.trace_length
    }

    fn trace_layout(&self) -> (usize, usize) {
        (54, 0)
    }

    fn pub_inputs(&self) -> &Self::PublicInputs {
        &()
    }
}

// Assumption: the number of rows is a power of two
pub fn build_cpu_trace(columns: Vec<Vec<FE>>) -> CPUTraceTable {
    TraceTable::from_columns_main(columns, 1)
}
