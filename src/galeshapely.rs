
pub(crate) mod matching;
pub fn is_stable_matching(problem : matching::Matching) -> bool {
        let stud_match = problem.stud_match;
        let size = stud_match.len();
        let uni_pref = problem.uni_pref;
        let stud_pref = problem.stud_pref;
        for i in 0..size {
            let uni = stud_match[i];
            if uni != -1 {
                let pref_list = &uni_pref[uni as usize];
                for j in 0..size {
                    if stud_match[j] == -1 && prefers(pref_list, i, j) {
                        return false;
                    }
                }
            }
            
        }
        for i in 0..size {
            let uni = stud_match[i];
            if uni != -1 {
                let pref_list = &uni_pref[uni as usize];
                for j in 0..size{
                    let uni2 = stud_match[j];
                    if uni2 != -1 {
                        let pref_list1 = &stud_pref[j];
                        if prefers(pref_list,i,j) && prefers(&pref_list1,uni as usize,uni2 as usize) {
                            return false;
                        }
                    }
                }
            }
            
        }
        return true;

    }
    pub fn stable_matching_uni_opt(problem : matching::Matching) -> matching::Matching {
        let mut avail_uni = problem.total_uni_pos();
        let mut uni_pos = problem.uni_pos.clone();
        let uni_cnt = problem.m;
        let stud_cnt = problem.n;
        let mut student_matching =  vec![-1 as i32; stud_cnt];
        let uni_pref = problem.uni_pref.clone();
        let stud_pref = problem.stud_pref.clone();
        let mut quick_pref = vec![vec![0; uni_cnt]; stud_cnt];
        for i in 0..stud_cnt{
            for j in 0..uni_cnt{
                quick_pref[i][stud_pref[i][j] as usize] = j;
            }
        }
        let mut current_uni = 0;
        while avail_uni > 0 {
            let mut slots = uni_pos[current_uni];
            if slots != 0 { 
                for &prefrence in uni_pref[current_uni].iter(){
                    let stud_match = student_matching[prefrence];
                    if stud_match != current_uni as i32 {
                        if stud_match == -1 {
                            avail_uni -= 1;
                            student_matching[prefrence] = current_uni as i32;
                            slots -= 1;
                            break;
                        }else if quick_pref[prefrence][current_uni] < quick_pref[prefrence][stud_match as usize]{
                            student_matching[prefrence] = current_uni as i32;
                            uni_pos[stud_match as usize] = uni_pos[stud_match as usize] + 1; 
                            slots-= 1;
                            break;
                        }
                    }
                }
            }
            uni_pos[current_uni] = slots;
            if current_uni == uni_cnt -1 {
                current_uni = 0;
            }
            else{
                current_uni += 1;
            }
        }
        matching::Matching {
            stud_match : student_matching,
            uni_pos,
            ..problem
        }
    }
    fn stable_matching_stud_opt(problem : matching::Matching) -> matching::Matching {
        let mut avail_uni = problem.total_uni_pos();
        let mut uni_pos = problem.uni_pos.clone();
        let uni_cnt = problem.m;
        let stud_cnt = problem.n;
        let mut student_matching =  vec![-1 as i32; stud_cnt];
        let uni_pref = problem.uni_pref.clone();
        let stud_pref = problem.stud_pref.clone();
        let mut quick_pref = vec![vec![0; uni_cnt]; stud_cnt];
        for i in 0..stud_cnt{
            for j in 0..uni_cnt{
                quick_pref[i][stud_pref[i][j] as usize] = j;
            }
        }
        matching::Matching {
                stud_match : student_matching,
                uni_pos,
                ..problem
            }
        }

    
    fn prefers(list : &Vec<usize>, a : usize, b : usize) -> bool {
        if a == b {
            return false;
        }
        for i in 0..list.len() {
            if list[i] == a {
                return true;
            }
            if list[i] == b {
                return false;
            }
        }
        return false;
    }