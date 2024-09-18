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
    }
}

fn run_query(q: Query, tl: &mut TodoList) -> Result<QueryResult, QueryError> {
    match q {
        Query::Add(desc, tags) => {
            let listitem = tl.push(desc, tags);
            return Ok(QueryResult::Added(listitem));
        }
        Query::Done(idx) => {
            tl.done_with_index(idx);
            return Ok(QueryResult::Done);
        }
        Query::Search(params) => {
            let mut output = Vec::new();
            let result = tl.search(params);
            for i in result {
                output.push(i.clone());
            }
            // Reverse the output to get descending order
            output.reverse();
            return Ok(QueryResult::Found(output));
        }
    }
}
