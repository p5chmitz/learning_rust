mod util;
mod cncpt;
mod exmpl;

fn main() {
    println!("Im a root crate");
    util::time::static_time::lol();
    //util::time::looped_time::looper(8);
    util::time::lol_again::idk();
    cncpt::control_flow::loops::_my_age_static();
    //exmpl::guessing_game::game();
    //exmpl::checking();
    cncpt::control_flow::ifs::if_statements(0.45);
    cncpt::control_flow::ifs::again_lets_if(6);
}
