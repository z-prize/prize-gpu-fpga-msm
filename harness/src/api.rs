use ark_ec::AffineCurve;
use ark_ff::PrimeField;
use std::os::raw::c_void;

#[repr(C)]
pub struct MultiScalarMultContext {
    context: *mut c_void,
}

pub fn multi_scalar_mult_init<G: AffineCurve>(_points: &[G]) -> MultiScalarMultContext {
    MultiScalarMultContext {
        context: std::ptr::null_mut(),
    }
}

pub fn multi_scalar_mult<G: AffineCurve>(
    _context: &mut MultiScalarMultContext,
    points: &[G],
    scalars: &[<G::ScalarField as PrimeField>::BigInt],
) -> Vec<G::Projective> {
    let npoints = points.len();
    if scalars.len() % npoints != 0 {
        panic!("length mismatch")
    }

    scalars
        .chunks_exact(npoints)
        .map(|batch| ark_ec::msm::VariableBaseMSM::multi_scalar_mul(points, batch))
        .collect()
}
