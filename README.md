# Description

Keeper is an experiment to further decentralization of off chain jobs. It currently has 2 instructions, register_job 
and perform_job. 
- register_job - registering a job, a job describes a program which contains a specific permission-less instruction which need
to be called by off chain jobs. It also contains a credits vault, a protocol which wants to incentivise keepers to call this 
instruction should top-up this vault with credits belonging to a mint of choice.
- perform_job - a keeper is supposed to call this instruction with the required accounts needed by the specified job. Internally the program performs the job by cpi'ing into the target program. In exchange for triggering the job, the keeper is credited with credits belonging to the mint configured while registering the job. 

# Development

## Rust
* Built and developed using - rust stable(`rustc 1.59.0-nightly (c5ecc1570 2021-12-15)`)
* Run rust based tests - `cargo test-bpf`