fn main() {
    let prerequisites = vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]];

    let num_courses = 5;

    println!("{}", can_finish(num_courses, prerequisites));
}

fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    use std::collections::HashSet;

    fn traverse(prereq: i32, course_prereq: &Vec<Vec<i32>>, completed: &mut HashSet<i32>, visited: &mut HashSet<i32>) -> bool {
        if completed.contains(&prereq) || course_prereq[prereq as usize].len() == 0 {
            completed.insert(prereq);
            return true;
        }

        if visited.contains(&prereq) {
            return false;
        }

        (*visited).insert(prereq);

        for pre_prereq in course_prereq[prereq as usize].to_owned() {
            if traverse(pre_prereq, &course_prereq, completed, visited) == false {
                return false;
            }
        }

        completed.insert(prereq);
        (*visited).remove(&prereq);
        return true;
    }

    let mut course_prereq: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];

    for prereq in prerequisites {
        course_prereq[prereq[0] as usize].push(prereq[1]);
    }

    let mut completed: HashSet<i32> = HashSet::new();

    for course in 0..num_courses as usize {
        for prereq in course_prereq[course].to_owned() {
            let mut visited: HashSet<i32> = HashSet::new();
            if traverse(prereq, &course_prereq, &mut completed, &mut visited) == false {
                return false;
            }
        }
    }
    
    true
}

// Create a graph structure of the courses
// The possibilities are:
// - A list is finite and has an end that doesn't have any requirements
// - A list contains an infinite loop
