# Description

Keeper is an experiment to further decentralization of off-chain jobs. It currently has 2 instructions, register_job 
and perform_job. 
- register_job - registering a job, a job describes a program which contains a specific permission-less instruction which needs
to be called by off chain jobs. It also contains a credits vault, a protocol which wants to incentivise keepers to call this 
instruction should top-up this vault with credits belonging to a mint of choice.
- perform_job - a keeper is supposed to call this instruction with the required accounts needed by the specified job. Internally the program performs the job by cpi'ing into the target program. In exchange for triggering the job, the keeper is credited with credits belonging to the mint configured while registering the job.

## Open questions
- currently, the registering and execution of job, configure and verify an instruction tag. This is expected to be the first 4 bytes of the receiving program. It is basically locking the job down to a specific instruction on the receiving programs side. I am unsure of how much value this adds, and if this check is necessary. Also program authors are free to decode incoming data to their liking and point it to an instruction of choice.
- credits are transferred after every job, this is the simplest implementation. Its the responsibility of the job, to verify that it has sufficient calls within a period, and fail redundant calls. This technique also saves having more complex housekeeping and separate distribution of awarded credit.

## Remaining work
- top-up credits vault
- ...

# Development

## Rust
* Built and developed using - rust stable(`rustc 1.59.0-nightly (c5ecc1570 2021-12-15)`)
* Run rust based tests - `cargo test-bpf`