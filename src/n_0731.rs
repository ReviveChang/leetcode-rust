use std::collections::BTreeMap;

struct MyCalendarTwo {
    delta : BTreeMap<i32,i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {

    fn new() -> Self {
        MyCalendarTwo{
            delta:BTreeMap::new()
        }
    }
    

    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.delta.entry(start).or_insert(0)+=1;
        *self.delta.entry(end).or_insert(0)-=1;
        
        let mut active = 0i32;
        for (_k,i) in self.delta.iter(){
            active += i;
            if active >= 3 {
                *self.delta.entry(start).or_default()-=1;
                *self.delta.entry(end).or_default()+=1;
                if self.delta.get(&start).unwrap()==&0 {
                    self.delta.remove(&start);
                }
                return false;
            }
        }

        true
    }
}