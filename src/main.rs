use problem::Problem;

pub mod philosopher;
pub mod problem;
mod stick;

fn main() {
    let problem = Problem::new(5);

    problem.start();
}
