pub struct Solution;
use std::collections::HashMap;

struct WordsFrequency {
    map:HashMap<String,i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordsFrequency {

    fn new(book: Vec<String>) -> Self {
        let mut map = HashMap::new();
        for word in book{
            let counter = map.entry(word).or_insert(0);
            *counter+=1;
        }
        WordsFrequency{map}
    }
    
    fn get(&self, word: String) -> i32 {
        match self.map.get(&word){
            Some(&i)=>i,
            None=>0,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::WordsFrequency;

    #[test]
    fn test(){
        let wf = WordsFrequency::new(vec!["i".to_string(), "have".to_string(), "an".to_string(), "apple".to_string(), "he".to_string(), "have".to_string(), "a".to_string(), "pen".to_string()]);
        assert_eq!(wf.get("have".to_string()),2);
    }
}