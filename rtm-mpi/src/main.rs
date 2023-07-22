use rtm_lib::rtm_run;
use mpi::traits::*;

fn main() {
    // mpi
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let root_rank = 0;
    world.barrier();
}


