mod fetch;
mod parse;
mod problem;

use crate::problem::Problem;

pub async fn run(contest_id: String, problem_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut ploblem = Problem::new(contest_id, problem_id);
    ploblem.get_next_list().await?;

    loop {
        ploblem.submission_select().await?;
    }
}
