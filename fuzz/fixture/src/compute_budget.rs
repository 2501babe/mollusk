//! Compute budget for instructions.

use {
    super::proto::ComputeBudget as ProtoComputeBudget,
    solana_compute_budget::compute_budget::ComputeBudget,
};

impl From<ProtoComputeBudget> for ComputeBudget {
    fn from(value: ProtoComputeBudget) -> Self {
        let ProtoComputeBudget {
            compute_unit_limit,
            log_64_units,
            create_program_address_units,
            invoke_units,
            max_instruction_stack_depth,
            max_instruction_trace_length,
            sha256_base_cost,
            sha256_byte_cost,
            sha256_max_slices,
            max_call_depth,
            stack_frame_size,
            log_pubkey_units,
            max_cpi_instruction_size,
            cpi_bytes_per_unit,
            sysvar_base_cost,
            secp256k1_recover_cost,
            syscall_base_cost,
            curve25519_edwards_validate_point_cost,
            curve25519_edwards_add_cost,
            curve25519_edwards_subtract_cost,
            curve25519_edwards_multiply_cost,
            curve25519_edwards_msm_base_cost,
            curve25519_edwards_msm_incremental_cost,
            curve25519_ristretto_validate_point_cost,
            curve25519_ristretto_add_cost,
            curve25519_ristretto_subtract_cost,
            curve25519_ristretto_multiply_cost,
            curve25519_ristretto_msm_base_cost,
            curve25519_ristretto_msm_incremental_cost,
            heap_size,
            heap_cost,
            mem_op_base_cost,
            alt_bn128_addition_cost,
            alt_bn128_multiplication_cost,
            alt_bn128_pairing_one_pair_cost_first,
            alt_bn128_pairing_one_pair_cost_other,
            big_modular_exponentiation_cost,
            poseidon_cost_coefficient_a,
            poseidon_cost_coefficient_c,
            get_remaining_compute_units_cost,
            alt_bn128_g1_compress,
            alt_bn128_g1_decompress,
            alt_bn128_g2_compress,
            alt_bn128_g2_decompress,
        } = value;

        Self {
            compute_unit_limit,
            log_64_units,
            create_program_address_units,
            invoke_units,
            max_instruction_stack_depth: max_instruction_stack_depth as usize,
            max_instruction_trace_length: max_instruction_trace_length as usize,
            sha256_base_cost,
            sha256_byte_cost,
            sha256_max_slices,
            max_call_depth: max_call_depth as usize,
            stack_frame_size: stack_frame_size as usize,
            log_pubkey_units,
            max_cpi_instruction_size: max_cpi_instruction_size as usize,
            cpi_bytes_per_unit,
            sysvar_base_cost,
            secp256k1_recover_cost,
            syscall_base_cost,
            curve25519_edwards_validate_point_cost,
            curve25519_edwards_add_cost,
            curve25519_edwards_subtract_cost,
            curve25519_edwards_multiply_cost,
            curve25519_edwards_msm_base_cost,
            curve25519_edwards_msm_incremental_cost,
            curve25519_ristretto_validate_point_cost,
            curve25519_ristretto_add_cost,
            curve25519_ristretto_subtract_cost,
            curve25519_ristretto_multiply_cost,
            curve25519_ristretto_msm_base_cost,
            curve25519_ristretto_msm_incremental_cost,
            heap_size,
            heap_cost,
            mem_op_base_cost,
            alt_bn128_addition_cost,
            alt_bn128_multiplication_cost,
            alt_bn128_pairing_one_pair_cost_first,
            alt_bn128_pairing_one_pair_cost_other,
            big_modular_exponentiation_cost,
            poseidon_cost_coefficient_a,
            poseidon_cost_coefficient_c,
            get_remaining_compute_units_cost,
            alt_bn128_g1_compress,
            alt_bn128_g1_decompress,
            alt_bn128_g2_compress,
            alt_bn128_g2_decompress,
        }
    }
}

impl From<ComputeBudget> for ProtoComputeBudget {
    fn from(value: ComputeBudget) -> Self {
        let ComputeBudget {
            compute_unit_limit,
            log_64_units,
            create_program_address_units,
            invoke_units,
            max_instruction_stack_depth,
            max_instruction_trace_length,
            sha256_base_cost,
            sha256_byte_cost,
            sha256_max_slices,
            max_call_depth,
            stack_frame_size,
            log_pubkey_units,
            max_cpi_instruction_size,
            cpi_bytes_per_unit,
            sysvar_base_cost,
            secp256k1_recover_cost,
            syscall_base_cost,
            curve25519_edwards_validate_point_cost,
            curve25519_edwards_add_cost,
            curve25519_edwards_subtract_cost,
            curve25519_edwards_multiply_cost,
            curve25519_edwards_msm_base_cost,
            curve25519_edwards_msm_incremental_cost,
            curve25519_ristretto_validate_point_cost,
            curve25519_ristretto_add_cost,
            curve25519_ristretto_subtract_cost,
            curve25519_ristretto_multiply_cost,
            curve25519_ristretto_msm_base_cost,
            curve25519_ristretto_msm_incremental_cost,
            heap_size,
            heap_cost,
            mem_op_base_cost,
            alt_bn128_addition_cost,
            alt_bn128_multiplication_cost,
            alt_bn128_pairing_one_pair_cost_first,
            alt_bn128_pairing_one_pair_cost_other,
            big_modular_exponentiation_cost,
            poseidon_cost_coefficient_a,
            poseidon_cost_coefficient_c,
            get_remaining_compute_units_cost,
            alt_bn128_g1_compress,
            alt_bn128_g1_decompress,
            alt_bn128_g2_compress,
            alt_bn128_g2_decompress,
        } = value;

        Self {
            compute_unit_limit,
            log_64_units,
            create_program_address_units,
            invoke_units,
            max_instruction_stack_depth: max_instruction_stack_depth as u64,
            max_instruction_trace_length: max_instruction_trace_length as u64,
            sha256_base_cost,
            sha256_byte_cost,
            sha256_max_slices,
            max_call_depth: max_call_depth as u64,
            stack_frame_size: stack_frame_size as u64,
            log_pubkey_units,
            max_cpi_instruction_size: max_cpi_instruction_size as u64,
            cpi_bytes_per_unit,
            sysvar_base_cost,
            secp256k1_recover_cost,
            syscall_base_cost,
            curve25519_edwards_validate_point_cost,
            curve25519_edwards_add_cost,
            curve25519_edwards_subtract_cost,
            curve25519_edwards_multiply_cost,
            curve25519_edwards_msm_base_cost,
            curve25519_edwards_msm_incremental_cost,
            curve25519_ristretto_validate_point_cost,
            curve25519_ristretto_add_cost,
            curve25519_ristretto_subtract_cost,
            curve25519_ristretto_multiply_cost,
            curve25519_ristretto_msm_base_cost,
            curve25519_ristretto_msm_incremental_cost,
            heap_size,
            heap_cost,
            mem_op_base_cost,
            alt_bn128_addition_cost,
            alt_bn128_multiplication_cost,
            alt_bn128_pairing_one_pair_cost_first,
            alt_bn128_pairing_one_pair_cost_other,
            big_modular_exponentiation_cost,
            poseidon_cost_coefficient_a,
            poseidon_cost_coefficient_c,
            get_remaining_compute_units_cost,
            alt_bn128_g1_compress,
            alt_bn128_g1_decompress,
            alt_bn128_g2_compress,
            alt_bn128_g2_decompress,
        }
    }
}
