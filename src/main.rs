use lib::{self, pe16};

fn main() {

    //RUN ALL PROBLEMS
    //lib::run_all();

    //RUN CURRENT PROBLEM
    //lib::run_current();

    //RUN SPECIFIC PROBLEM BY FUNCTION NAME
    lib::run(&(pe16 as fn() -> i64));

}