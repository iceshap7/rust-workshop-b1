fn main() {
    array_matching();
    word_counter();
}

fn array_matching() {
    let org_arr = [1, 2, 3, 4, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    let mut matches = false;

    for i in 0.. org_arr.len() {
        if matches {
            break;
        }

        if sub_arr[0] == org_arr[i] {
            for j in 1.. sub_arr.len() {
                if sub_arr[j] != org_arr[i + j] {
                    break;
                }

                if j == sub_arr.len() - 1 {
                    matches = true;
                }
            }
        }
    }

    print!("\n{}\n\n", matches);
}

fn word_counter() {
    let str = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    let mut count = 0;

    let mut word = String::new();
    println!("Enter your word: ");
    std::io::stdin().read_line(&mut word).unwrap();
    word.pop();

    let input_vec: Vec<char> = word.chars().collect();
    let str_vec: Vec<char> = str.chars().collect();

    for i in 0.. str_vec.len() {
        if str_vec[i] == input_vec[0] {
            for j in 1.. input_vec.len() {
                if input_vec[j] != str_vec[i + j] {
                    break;
                }

                if j == input_vec.len() - 1 {
                    count += 1;
                }
            }
        }
    }

    print!("\n{}\n\n", count);
}
