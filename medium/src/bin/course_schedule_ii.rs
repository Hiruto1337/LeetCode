fn main() {
    let prerequisites = vec![[1,0].to_vec(),[1,2].to_vec(),[0,1].to_vec()];
    let num_courses = 7;

    println!("{:?}", find_order(num_courses, prerequisites));
}

fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut prereq_courses: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];
    let mut in_degree: Vec<i32> = vec![0; num_courses as usize];
    
    for prereq in prerequisites {
        prereq_courses[prereq[1] as usize].push(prereq[0]);
        in_degree[prereq[0] as usize] += 1;
    }

    let mut queue: Vec<i32> = vec![];

    for i in 0..in_degree.len() {
        if in_degree[i] == 0 {
            queue.push(i as i32);
        }
    }

    let mut completed: Vec<i32> = vec![];

    while queue.len() != 0 {
        let course = queue.remove(0);

        completed.push(course);

        for dep in prereq_courses[course as usize].to_owned() {
            in_degree[dep as usize] -= 1;
            if in_degree[dep as usize] == 0 {
                queue.push(dep);
            }
        }
    }

    if completed.len() as i32 == num_courses {
        return completed;
    } else {
        return vec![];
    }
}
