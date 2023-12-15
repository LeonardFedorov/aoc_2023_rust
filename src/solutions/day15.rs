pub fn day15_solution(input_data: String) -> (String, String) {

    //Prepare the input data as a vector so each part can create their own iterator over it
    let source = input_data.split(',').map(
        |block| block.as_bytes().to_vec()
    ).collect::<Vec<Vec<u8>>>();

    //Do part 1
    let mut p1_result = 0;
    for block in source.iter() {
        p1_result += hash(&block);
    }

    //Build the lens array for part 2
    let box_count = 256;
    let mut lens_array: Vec<Vec<(&[u8], usize)>> = vec![Vec::new(); box_count];

    for block in source.iter() {

        //Find the operator split
        let mut split_pos = 0;
        for i in 0 .. block.len() {
            match block[i] {
                b'=' | b'-' => {split_pos = i; break;}
                _ => () //Do nothing
            }
        }

        //Parse out the instruction details
        let label = &block[0 .. split_pos];
        let operator = block[split_pos];
        let focal = match operator {
            b'=' => (block[split_pos + 1] - b'0') as usize, //Since the focal lengths are known to be single digit
            b'-' => 0,
            _ => panic!("Invalid operator"),
        };
        let address = hash(label);

        //"Delete" lens by just setting it to a null object. Uses more space but will save time with trying to shuffle vectors around
        if operator == b'-' {
            for i in 0 .. lens_array[address].len() {
                if lens_array[address][i].0 == label {
                    lens_array[address][i] = (&[], 0);
                }
            }
        } else {
            //Try to find the lens first
            let mut lens_found = false;
            for i in 0 .. lens_array[address].len() {
                if lens_array[address][i].0 == label {
                    lens_array[address][i].1 = focal;
                    lens_found = true;
                    break;
                }
            }
            if !lens_found {
                lens_array[address].push((label, focal));
            }
        }

    }

    //Now total up the focal powers
    let mut p2_result = 0;

    for i in 0 .. lens_array.len() {
        let mut box_lens_count = 0usize;
        for j in 0 .. lens_array[i].len() {
            if lens_array[i][j].1 > 0 {
                box_lens_count += 1;
                p2_result += (i + 1) * box_lens_count * lens_array[i][j].1;
            }
        }
    }

    return (p1_result.to_string(), p2_result.to_string());
}

//Computes the hash value of the given string
fn hash(label: &[u8]) -> usize {
    let mut result = 0;
    for i in 0 .. label.len() {
        result = ((result + (label[i] as usize)) * 17) % 256;
    }     
    return result;
}