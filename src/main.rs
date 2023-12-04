mod cncpt;
mod exmpl;
mod util;

fn main() {
    println!("Im a root crate");
    println!("{}", util::time::static_time(8));
    cncpt::ctrl_flow::loops::_my_age_static();
    //Sub-module/funciton re-exported in exmpl.rs/util.rs for cleaner access to example programs
    exmpl::guessing_game();
    exmpl::rng_range();
    cncpt::ctrl_flow::ifs::if_statements(0.45);
    cncpt::ctrl_flow::ifs::again_lets_if(6);
    util::time::loop_time(8);
}
