use std::collections::BTreeMap;

use easy_scraper::Pattern;

pub fn get_problem_name(list_xml: &str) -> Result<String, String> {
    let pattern_str = r#"<option selected>{{problem_name}}</option>"#;

    // Patternオブジェクトを作り、xmlから検索する
    let pat = Pattern::new(&pattern_str)?;
    let ms = pat.matches(&list_xml);

    if ms.len() == 0 {
        let err = "Can't parse the problem name".to_string();
        return Err(err);
    }
    let ret = ms[0].get("problem_name").unwrap().to_string();

    Ok(ret)
}

pub fn get_submission_list(list_xml: &str) -> Result<Vec<BTreeMap<String, String>>, String> {
    let pattern_str = r#"
        <table><tbody>
          <tr>
            <td class="no-break"><time>{{time}}</time></td>
            <td><a>{{title}}</a></td>
            <td><a>{{user_name}}</a><a><span></span></a></td>
            <td><a>{{lang}}</a></td>
            <td>{{score}}</td>
            <td>{{source_length}}</td>
            <td><span>{{status}}</span></td>
            <td>{{sec}}</td>
            <td>{{memory}}</td>
            <td><a href="{{submission_url}}">Detail</a></td>
          </tr>
        </tbody></table>
    "#;

    let pat = Pattern::new(pattern_str)?;

    // xmlからPatternを検索する
    let ms = pat.matches(&list_xml);

    Ok(ms)
}

pub fn get_code(xml: &str) -> Result<String, String> {
    let pattern_str = r#"<pre id="submission-code">{{code}}</pre>"#;
    let pat = Pattern::new(pattern_str)?;
    let code = pat.matches(&xml)[0].get("code").unwrap().to_string();

    Ok(code)
}
