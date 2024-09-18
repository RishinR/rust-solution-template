use crate::*;

pub fn run_line(line: &str, tl: &mut TodoList) {
    if let Ok((_, q)) = parser::query(line) {
        match run_query(q, tl) {
            Ok(r) => {
                println!("{}", r);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    } else {
        eprintln!("Error: Failed to parse the command.");
    }
}

fn run_query(q: Query, tl: &mut TodoList) -> Result<QueryResult, QueryError> {
    match q {
        Query::Add(desc, tags) => {
            let list_item = tl.push(desc, tags);
            Ok(QueryResult::Added(list_item))
        }
        Query::Done(idx) => {
            if tl.done_with_index(idx).is_some() {
                Ok(QueryResult::Done)
            } else {
                Err(QueryError("Invalid index".to_string()))
            }
        }
        Query::Search(params) => {
            let results = tl.search(params);
            Ok(QueryResult::Found(results))
        }
    }
}
