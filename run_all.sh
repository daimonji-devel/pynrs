#!/bin/bash

project_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

echo "### 1. trying all numbers until sqrt #############################"
python3 $project_dir/pynrs_python/is_prime1.py
cargo run --release --manifest-path $project_dir/pynrs_rust/Cargo.toml --bin is_prime1
echo

echo "### 2. using a bool array ########################################"
python3 $project_dir/pynrs_python/is_prime2.py
cargo run --release --manifest-path $project_dir/pynrs_rust/Cargo.toml --bin is_prime2
echo

echo "### 3. using an integer array ####################################"
python3 $project_dir/pynrs_python/is_prime3.py
cargo run --release --manifest-path $project_dir/pynrs_rust/Cargo.toml --bin is_prime3
echo

echo "### 3b. combining Python and Rust ################################"
venv_dir=$project_dir/venv

if [ -d $venv_dir ]; then
    source $venv_dir/bin/activate
else
    python3 -m venv $venv_dir
    source $venv_dir/bin/activate
    pip install maturin
    maturin build -r --manifest-path $project_dir/pynrs/Cargo.toml
    pip install $project_dir/pynrs/target/wheels/*.whl
fi

python3 $project_dir/pynrs/is_prime3.py
deactivate
