use std::{alloc::System, fs::File, io::{BufRead, BufReader}, path::Path, str::Lines, vec};

use galeshapely::matching::Matching;

mod galeshapely;
struct Driver{
    pub filename: String,
    pub testGS_c : bool,
    pub testGS_i : bool,
    pub testBF: bool
}
 // Make the matching module public

impl Driver {
    fn main_driver(&self){
        if (self.testBF){
            
        }
    }
    fn parse_matching_problem_with_example(input_file : Vec<String>) -> galeshapely::matching::Matching{
        let m : usize;
        let n : usize;
        let university_prefs : Vec<Vec<usize>>;
        let student_prefs : Vec<Vec<usize>>;
        let uni_pos : Vec<usize>;
        let input_sizes: Vec<&str> = input_file[0].split_whitespace().collect();
        m = input_sizes[0].parse::<usize>().unwrap();
        n = input_sizes[1].parse::<usize>().unwrap();
        uni_pos = Self::read_positions_list(&input_file[2], m); 
        university_prefs = Self::read_preference_lists(&input_file[2..m+2], m);
        student_prefs = Self::read_preference_lists(&input_file[m+2..m+2+n], n);
        let example_matching = vec![0; m];
        Matching{
            m,
            n,
            uni_pref : university_prefs,
            stud_pref : student_prefs,
            uni_pos,
            stud_match : example_matching
        }
    }
    fn parse_matching_problem(input_file : Vec<String>) -> galeshapely::matching::Matching{
        let m : usize;
        let n : usize;
        let uni_pref : Vec<Vec<usize>>;
        let stud_pref : Vec<Vec<usize>>;
        let uni_pos : Vec<usize>;
        let input_sizes: Vec<&str> = input_file[0].split_whitespace().collect();
        m = input_sizes[0].parse::<usize>().unwrap();
        n = input_sizes[1].parse::<usize>().unwrap();
        uni_pos = Self::read_positions_list(&input_file[1], m); 
        uni_pref = Self::read_preference_lists(&input_file[2..m+2], m);
        stud_pref = Self::read_preference_lists(&input_file[m+2..m+2+n], n);
        let stud_match = vec![0;m];
        return Matching{
            m,
            n,
            uni_pref,
            stud_pref,
            uni_pos,
            stud_match

        };
    }
    fn read_positions_list(file : &String, m : usize) -> Vec<usize>{
        let mut pos : Vec<usize> = Vec::new();
        let line: Vec<&str> = file.split_whitespace().collect();
        for i in 0..m {
            pos.push(line[i].parse().unwrap());
        }
        return pos;
    } 
    fn read_preference_lists(file : &[String], m : usize) -> Vec<Vec<usize>>{
        let mut pos : Vec<Vec<usize>> = Vec::new();
        for i in 0..m{
            let line: Vec<usize> = file[i].split_whitespace().map(|num| {num.parse::<usize>().unwrap()}).collect();
            pos.push(line);
        }
        return pos;
    }
    fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }
    fn test_run(&self,problem: galeshapely::matching::Matching){
        let is_stable : bool;
        if self.testGS_c == true {
            let gsmatching = galeshapely::stable_matching_uni_opt(problem);
            //println!(GSMatching);
            let matching = gsmatching.stud_match.clone();
            is_stable = galeshapely::is_stable_matching(gsmatching);
            println!("Is stable {is_stable}");
            for (stud,item) in matching.clone().into_iter().enumerate(){
                println!("Student {stud} Uni {item}");
            }
        }
        

    }
    

    
}

fn main() {
    let filename = "src/inputs/1-8-8.in".to_owned();
    let drvr = Driver{
        filename,
        testGS_c: true,
        testGS_i : false,
        testBF : false
    };
    println!("Hello, world!");
    let file = Driver::lines_from_file("src/inputs/1-8-8.in");
    let problem  = Driver::parse_matching_problem(file);
    drvr.test_run(problem);
}   