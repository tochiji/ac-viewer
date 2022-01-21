use bat::{Input, PagingMode, PrettyPrinter};
use requestty::Question;

use crate::fetch::fetch_html;
use crate::parse::{get_code, get_problem_name, get_submission_list};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Problem {
    pub contest_id: String,
    pub problem_id: String,
    pub problem_name: Option<String>,
    pub list_page: u32,
    pub list: Vec<BTreeMap<String, String>>,
    pub select_index: usize,
}

impl Problem {
    pub fn new(contest_id: String, problem_id: String) -> Self {
        Problem {
            contest_id,
            problem_id,
            problem_name: None,
            list_page: 0,
            list: vec![],
            select_index: 0,
        }
    }
    pub fn get_submissions_list_url(&self) -> String {
        let contest_id = &self.contest_id;
        let problem_id = &self.problem_id;
        let page = self.list_page;
        format!("https://atcoder.jp/contests/{}/submissions?f.LanguageName=Rust&f.Status=AC&f.Task={}_{}&f.User=&orderBy=source_length&page={}",contest_id,contest_id,problem_id,page)
    }
    pub async fn get_next_list(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.list_page += 1;
        let url = &self.get_submissions_list_url();
        let xml = fetch_html(url).await?;

        if self.problem_name.is_none() {
            self.problem_name = Some(get_problem_name(&xml)?);
        }

        let mut l = get_submission_list(&xml)?;
        let _ = &self.list.append(&mut l);

        Ok(())
    }
    async fn print_code(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let submission = self.list.get(index).unwrap();

        let domain = "https://atcoder.jp";
        let url = format!("{}{}", domain, submission.get("submission_url").unwrap());
        let xml = fetch_html(&url).await?;
        let code = get_code(&xml)?;

        let title = self.get_submission_title(index);
        let input = Input::from_bytes(code.as_bytes()).name(title);

        PrettyPrinter::new()
            .input(input)
            .language("Rust")
            .true_color(false)
            .line_numbers(true)
            .grid(true)
            .header(true)
            .paging_mode(PagingMode::Always)
            .print()
            .unwrap();

        Ok(())
    }

    /// 下記のような文字列を出力する。
    /// 004  Rust (1.42.0)  2020-08-08 01:16:05(JST)    514 Byte   33 ms magurofly
    fn get_submission_title(&self, index: usize) -> String {
        let list = &self.list;
        let s = list.get(index).unwrap();

        format!(
            "{:03}  {}  {} {:>11} {:>7} {}",
            index + 1,
            s.get("lang").unwrap(),
            s.get("time").unwrap().replace("+0900", "(JST)"),
            s.get("source_length").unwrap(),
            s.get("sec").unwrap(),
            s.get("user_name").unwrap()
        )
    }
    fn get_submission_list(&self) -> Vec<String> {
        let list = &self.list;

        list.iter()
            .enumerate()
            .map(|(i, _)| self.get_submission_title(i))
            .chain(vec![String::from("read more")])
            .collect()
    }
    pub async fn submission_select(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let list = self.get_submission_list();

        let select = Question::select("theme")
            .should_loop(false)
            .message("Select a Submission")
            .choices(list)
            .default(self.select_index)
            .build();

        let answer = requestty::prompt_one(select)?;
        self.select_index = answer.as_list_item().unwrap().index;

        if self.select_index >= self.list.len() {
            self.get_next_list().await?;
        } else {
            self.print_code(self.select_index).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn it_works() {
        let contest_id = String::from("abc200");
        let problem_id = String::from("d");
        let mut p = Problem::new(contest_id, problem_id);
        p.get_next_list().await.unwrap();
        assert_eq!(p.problem_name, Some(String::from("D - Happy Birthday! 2")));
        assert_eq!(p.list.len(), 20);
    }
}
