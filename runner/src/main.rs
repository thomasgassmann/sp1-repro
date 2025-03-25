use std::{fs::File, io::Read};

use sp1_prover::components::CpuProverComponents;
use sp1_sdk::{SP1Context, SP1Prover, SP1Stdin};
use sp1_stark::SP1ProverOpts;

fn main() {
    let mut elf = File::open("/home/thomas/git/thesis/benchmarks/bin/loop-sum/sp1/baseline").unwrap();
    let mut buf = Vec::new();
    elf.read_to_end(&mut buf).unwrap();
    let prover = SP1Prover::<CpuProverComponents>::new();
    let (_, pk_d, program, vk) = prover.setup(&buf);
    let opts = SP1ProverOpts::auto();
    let stdin = SP1Stdin::new();
    prover
        .prove_core(&pk_d, program, &stdin, opts, SP1Context::default())
        .unwrap();
}
