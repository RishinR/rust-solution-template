use crate::*;

pub fn run_line(line: &str, tl: &mut TodoList) {
    match parser::query(line) {
        Ok((_, q)) => {
            match run_query(q, tl) {
                Ok(r) => {
                    println!("{}", r);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Err(_) => {
            // If there's an error while parsing the query, return a QueryError with a message.
            let error = QueryError("Invalid query format or content".to_string());
            eprintln!("{}", error);
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
