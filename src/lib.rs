mod acviewer_error;
mod fetch;
mod parse;
mod problem;
use crate::acviewer_error::AcViewerError;

use crate::problem::Problem;

pub async fn run(contest_id: String, problem_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut problem = Problem::new(contest_id, problem_id);
    problem.get_next_list().await?;

    if problem.list.len() == 0 {
        Err(AcViewerError::NoSubmission("No Submission Found"))?
    }

    loop {
        problem.submission_select().await?;
    }
}
