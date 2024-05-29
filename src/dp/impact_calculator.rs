

pub struct ImpactCalculator {
    pub total_time : usize,
    pub num_medicine : usize,
    pub out : Vec<Vec<usize>>

} 
impl ImpactCalculator{
    pub fn calculate_impact(&self, x : usize, y : usize) -> usize {
        self.out[x][y]
    }

}

