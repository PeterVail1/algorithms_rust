pub struct Matching {
    pub m : usize,
    pub n : usize,
    pub uni_pref : Vec<Vec<usize>>,
    pub stud_pref : Vec<Vec<usize>>,
    pub uni_pos : Vec<usize>,
    pub stud_match : Vec<i32>,
    
}
impl Matching {
    pub fn total_uni_pos(&self) -> usize {
        self.uni_pos.iter().sum()
   }
}