use ark_ec::msm::FixedBaseMSM;
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::fields::PrimeField;
use ark_std::{One, UniformRand, Zero};
use rand::rngs::StdRng;

pub fn generate_srs<P: AffineCurve>(beta: &P::ScalarField, length: usize) -> Vec<P> {
    let scalar_bits = P::ScalarField::size_in_bits();
    let mut scalars = Vec::with_capacity(length as _);
    scalars.push(P::ScalarField::one());
    scalars.push(*beta);
    let mut last = *beta;
    for _ in 2..length {
        last = last * beta;
        scalars.push(last);
    }
    let g = P::prime_subgroup_generator();
    let g_window = FixedBaseMSM::get_mul_window_size(length);
    let g_table =
        FixedBaseMSM::get_window_table::<P::Projective>(scalar_bits, g_window, g.into_projective());
    let res_jacobian =
        FixedBaseMSM::multi_scalar_mul::<P::Projective>(scalar_bits, g_window, &g_table, &scalars);
    P::Projective::batch_normalization_into_affine(&res_jacobian)
}

pub fn generate<P: AffineCurve>(
    mut rng: StdRng,
    length: usize,
    batches: usize,
) -> (
    Vec<P>,
    Vec<<P::ScalarField as PrimeField>::BigInt>,
    Vec<P::Projective>,
) {
    let g = P::prime_subgroup_generator();
    let beta = P::ScalarField::rand(&mut rng);
    eprintln!("using beta: {}", beta);

    let srs = generate_srs::<P>(&beta, length);

    let mut scalars = Vec::with_capacity(length * batches);
    let mut results = vec![P::Projective::zero(); batches];

    for i in 0..batches {
        let coeff: Vec<_> = (0..length)
            .map(|_| P::ScalarField::rand(&mut rng))
            .collect();

        let mut b = P::ScalarField::one();
        let mut prod = P::ScalarField::zero();
        for &c in &coeff {
            prod += c * b;
            b *= beta;
        }
        let res = g.mul(prod);

        let coeff_non_montgomery: Vec<_> = coeff.iter().map(P::ScalarField::into_repr).collect();

        // if validate_result?
        if cfg!(debug_assertions) {
            assert_eq!(
                res,
                ark_ec::msm::VariableBaseMSM::multi_scalar_mul(&srs, &coeff_non_montgomery)
            );
        }
        scalars.extend(coeff_non_montgomery);
        results[i] = res;
    }

    (srs, scalars, results)
}
