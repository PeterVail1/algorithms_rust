#[path = "impact_calculator.rs"]
mod impact_calculator;
use std::{fs::File, io::{BufRead, BufReader}};


use impact_calculator::ImpactCalculator;
pub fn calculate_impact(calculator : &ImpactCalculator) -> (usize,Vec<usize>) {
    let total_time = calculator.total_time;
    let num_medicine = calculator.num_medicine;
    let mut impact_array = vec![vec![0;total_time+1];num_medicine+1];
    let mut included = vec![vec![0;total_time+1]; num_medicine+1];
    let mut treatment_plan = vec![0; num_medicine];
    for x in 0..num_medicine+1{
        for y in 0..total_time+1 {
            if x == 0 || y == 0 {
                impact_array[x][y] = 0;
            }else{
                let (max,val) : (usize,usize) = impact_array[x-1].clone()[..y+1].into_iter().enumerate()
                                                  .fold((0,impact_array[x-1][y]),
                                                     |(accmax,acc), (z,_num)| {
                                                         let med = calculator.calculate_impact(x -1, z)+impact_array[x-1][y-z];
                                                         if med>accmax  {
                                                            (med, z)
                                                         }else {
                                                             (accmax,acc)
                                                         }
                                                     });
                if val != 0 {
                    included[x][y] = val;
                }
                impact_array[x][y] = max;
            }
            
        }
    }
    let mut x = num_medicine;
    let mut y = total_time;
    while x > 0 && y > 0 {
        if impact_array[x][y] == impact_array[x][y-1] {
            y -= 1;
        }else if included[x][y] != 0{
            treatment_plan[x-1] = included[x][y];
            y -= included[x][y];
            x -= 1;
        }else{
            x -= 1;
        }

        
    }
    return (impact_array[num_medicine][total_time],treatment_plan);
}
pub fn runner(){
    let filename = "src/dp/testfiles/large_inputs/512.in";
    let calculator = build(filename);
    let (total,treatment_plan) = calculate_impact(&calculator);
    print_plan(treatment_plan);
    println!("Total is {total}");
    
}
fn build(filename : &str) -> ImpactCalculator{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let lines : Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    let split : Vec<&str> = lines[0].split(" ").collect();
    let num_medicine : usize = split[0].parse().expect("Could not parse numMedicine");
    let total_time : usize = split[1].parse().expect("Could not parse totalTime");
    let mut out :Vec<Vec<usize>> = vec![vec![0;total_time +1]; num_medicine];
    for (x,line) in lines[1..].into_iter().enumerate(){
        let splitline = line.split(" ");
        for (y, item) in splitline.into_iter().enumerate() {
            out[x][y+1] = item.parse().expect("Could not parse val!");
        }
    }
    ImpactCalculator{
        num_medicine,
        total_time,
        out
    }
}
fn print_plan(treatment_plan : Vec<usize>){
    println!("Please administer medicines 1 through n for the following amounts of time:\n");
    for (num, item) in treatment_plan.iter().enumerate() {
        println!("Medicine {num}: {item}");
    }
}