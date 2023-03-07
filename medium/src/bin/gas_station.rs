fn main(){
    let mut gas = vec![];
    let mut cost = vec![];

    for _ in 0..99998 {
        gas.push(0);
        cost.push(0);
    }

    gas.push(0);
    gas.push(2);

    cost.push(1);
    cost.push(0);

    println!("Can complete if start index is {}", can_complete_circuit(gas, cost));
}

fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut tank = 0;
    let mut pos = 0;
    let mut missing = 0;
    let len = gas.len();

    for (i, (gas, cost)) in gas.into_iter().zip(cost).enumerate() {
        tank += gas - cost;

        if tank < 0 {
            missing += tank;
            pos = i + 1 % len;
            tank = 0;
        }
    }

    if tank + missing >= 0 {
        return pos as i32;
    } else {
        return -1;
    }
}