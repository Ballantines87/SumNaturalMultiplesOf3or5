fn main() {
    
    let mut user_input = String::new();

    println!("Please input a number:");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read input");

    let user_num:i32 = user_input
        .trim()
        .parse()
        .expect("failed to parse string into i32");

    let mut running_vector_of_mult_of_3 = vec![0];
    let mut counter = 0;

    let mut nat_multiples_of_3 = loop {
        counter += 1;
        if(counter*3 < user_num) {
            running_vector_of_mult_of_3.push(counter*3);
        }
        else {
            break running_vector_of_mult_of_3;
        }
        
    };

   println!("{:?}", nat_multiples_of_3);

    let mut running_vector_of_mult_of_5 = vec![0];
    let mut counter = 0;

    let mut nat_multiples_of_5 = loop {
        counter += 1;
        if(counter*5 < user_num) {
            running_vector_of_mult_of_5.push(counter*5);
        }
        else {
            break running_vector_of_mult_of_5;
        }
        
    };

    nat_multiples_of_3.remove(0);
    nat_multiples_of_5.remove(0);

    let mut mult= [nat_multiples_of_5, nat_multiples_of_3].concat();
    mult.sort();
    mult.dedup();
    let final_sum:i32 = mult.iter().sum();
    
    println!("{}", final_sum);



}
