mod cncpt;
mod exmpl;
mod util;

fn main() {
    println!("Im a root crate");
    println!("{}", util::time::static_time(8));
    //cncpt::ctrl_flow::loops::_my_age_static();
    //exmpl::guessing_game::game();
    //exmpl::checking();
    //cncpt::ctrl_flow::ifs::if_statements(0.45);
    //cncpt::ctrl_flow::ifs::again_lets_if(6);
    util::time::loop_time(8);
}
