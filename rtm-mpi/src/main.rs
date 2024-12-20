use mpi::traits::*;
use rtm::arg::*;
use rtm::rtm_run;
use std::env;

fn main() {
    let cli: MainArg = arg();
    // mpi
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let root_rank = 0;
    rtm_run(&cli);
    world.barrier();
    if rank == root_rank {
        println!("{} done", env!("CARGO_PKG_NAME"));
    }
}
