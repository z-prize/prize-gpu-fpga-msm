# Benchmark harness for FPGA MSM implementations in the ZPRIZE competition

This repository contains the code to benchmark FPGA implementations.

## Submission

A submission is recieved in the form of a URL pointing to a git repository with the following structure:

    root
     ├── cl/ (custom logic)
     └── driver
          ├── Cargo.toml
          └── src/

We use the master branch, and perform a shallow clone.

### Custom Logic

The custom logic will be compiled with the AWS HDK environment.
We will use HDK version `1.4.24`.
This means that the `cl` directory must contain a `build/scripts` subdirectory with a script named `aws_build_dcp_from_cl.sh`.
The output will be taken from `build/checkpoints/to_aws/<date>.Developer_CL.tar`.

### Driver

The driver package must be named `zprize_fpga_msm`, and export (at least) functions with the following signatures:
```rust
pub fn multi_scalar_mult_init<G: AffineCurve>(points: &[G]) -> MultiScalarMultContext;

pub fn multi_scalar_mult<G: AffineCurve>(
    context: &mut MultiScalarMultContext,
    points: &[G],
    scalars: &[<G::ScalarField as PrimeField>::BigInt],
) -> G::Projective
```

The driver will be compiled with the latest rust compiler (version 1.61.0).

## This repository

In this repository you will find:
* A script to compile FPGA images
* The benchmark harness
* Test vector generation utilities
* A script that evaluates a submission
