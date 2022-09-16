#!/bin/bash

export SUBMISSION_URL=$1
shift

exec 2>evaluate.log
set -ex

function info {
    echo -e "
\033[32m===============================\033[0m
[\033[96m INFO \033[0m] $@
\033[32m===============================\033[0m
"
}

# Clone into submission directory
info Cloning the repository

git clone \
    --branch master \
    --depth 1 \
    --single-branch \
    --recurse-submodules \
    --shallow-submodules \
    $SUBMISSION_URL submission

info Setting up HDK
set +e
source aws-fpga/hdk_setup.sh
set -e

# Compile logic
info Compiling CL
export CL_DIR=$(pwd)/submission/cl
vivado -mode batch
pushd $CL_DIR/build/scripts
./aws_build_dcp_from_cl.sh -foreground
popd

# Create AFI
info Creating AFI
source ./build_image.sh -dcp $CL_DIR/build/checkpoints/to_aws/*Developer_CL.tar
AGFI_ID=$(grep agfi image-id.json | cut -d'"' -f 4)

# Load the image (twice, due to weird bugs in aws)
info Loading AFI
sudo fpga-load-local-image -S 0 -I $AGFI_ID
sudo fpga-load-local-image -S 0 -I $AGFI_ID

# Run correctness tests
info Testing correctness
pushd harness
cargo test --release

# Run benchmark
info Benchmarking
cargo bench
popd

# The output should now be in `harness/target/criterion/**`
