use std::fmt;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index(u64);

impl Index {
    pub fn new(i: u64) -> Index {
        Index(i)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description(String);

impl Description {
    pub fn new(s: &str) -> Description {
        Description(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag(String);

impl Tag {
    pub fn new(s: &str) -> Tag {
        Tag(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn from_strings(ss: Vec<&str>) -> Vec<Tag> {
        ss.clone().into_iter().map(|s| Tag::new(s)).collect()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub index: Index,
    pub description: Description,
    pub tags: Vec<Tag>,
    pub done: bool,
}

impl TodoItem {
    pub fn new(index: Index, description: Description, tags: Vec<Tag>, done: bool) -> TodoItem {
        TodoItem {
            index,
            description,
            tags,
            done,
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tag_list: Vec<String> = self.tags.iter().map(|t| format!("#{}", t.value())).collect();
        write!(f, "{} \"{}\" {}", self.index, self.description, tag_list.join(" "))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoList {
    top_index: Index,
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            top_index: Index::new(0),
            items: vec![],
        }
    }

    pub fn push(&mut self, description: Description, tags: Vec<Tag>) -> TodoItem {
        let item = TodoItem {
            index: self.top_index,
            description,
            tags,
            done: false,
        };
        self.top_index.0 += 1;
        self.items.push(item.clone());
        item
    }

    pub fn done_with_index(&mut self, idx: Index) -> Option<Index> {
        for i in &mut self.items {
            if i.index == idx {
                i.done = true;
                return Some(idx);
            }
        }
        None
    }

    pub fn search(&self, sp: SearchParams) -> Vec<&TodoItem> {
        let mut results = Vec::new();
        // invalidating anyother input
        if sp.words.is_empty() && sp.tags.is_empty() {
            eprintln!("Search parameters are empty. Please provide at least one word or tag.");
            return results;
        }

        for i in &self.items {
            let matches_tags = if !sp.tags.is_empty() {
                sp.tags.iter().any(|tag| {
                    i.tags.iter().any(|item_tag| item_tag.value().contains(&tag.value()))
                })
            } else {
                true
            };
    
            let matches_words = if !sp.words.is_empty() {
                sp.words.iter().any(|word| {
                    i.description.value().contains(&word.0)
                })
            } else {
                true
            };
    
            // Only include items if the match satisfies both conditions (tags and words) when provided
            if matches_tags && matches_words && !i.done {
                results.push(i);
            }
        }
    
        results
    }    
    
}
