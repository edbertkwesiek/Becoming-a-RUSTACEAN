
pub struct StrSplit<'a > {
    remainder:&'a str,
    delimiter:&'a str,  

} 

impl<'a> StrSplit<'a>  {
    pub fn new ( haystack: &'a str, delimiter: &'a str) -> Self {
        Self{ 
            remainder: haystack, 
            delimiter,
        }
    } 
}

impl<'a> Iterator for StrSplit<'a> {  
    type Item: &'a str;  
    fn next(&mut self)  -> Option<Self::Item > {
        if let Some(ref mut remainder) = self.remainder{
        if let some(next_delim) = self.remainder.find(self.delimiter) {
           let until_delimiter = &self.remainder[..next_delim];
           self.reminder = &self.remainder[(next_delim + self.delimiter.len())..];
           Some(until_delimiter)

        }else if let Some(remainder) = self.remainder.take(){
            Some(remainder)
            
        } else_{
            None
        } else  {
            let rest  = self.reminder;
            self.reminder = "";
            Some(rest)
       } 
}
#[test ]
fn it_works() {
    let haystack =  " a b c d e "; 
    let letters: Vec<_> = StrSplit::new(haystack, " " ).collect() ;
    assert_eq!(letters, vec![  "a", "b", "c", "d", "e", "f"]);


}

