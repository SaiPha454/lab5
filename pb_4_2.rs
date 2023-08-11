fn main() {
    let test_list = vec![1.0,-1.0,2.0,-2.0];

    let mut index: usize = 0;

    let mut non_ng_list: Vec<f32> = Vec::new();


    let res = extract_non_negatives(&test_list, &mut index , &mut non_ng_list);




}

fn extract_non_negatives(input_list: &Vec<f32>, index : &mut usize, non_ng_list : &mut Vec<f32>) -> Vec<f32> {


    if input_list.len() ==0  {
        return  Vec::new();
    }

    let f = input_list[*index];
    if f >= 0.0 {
        non_ng_list.push(f);
    }

    *index +=1;
    if (*index < input_list.len()) {
        extract_non_negatives(input_list, index, non_ng_list);
    }

    // for f in input_list {
        
    // }

    non_ng_list.to_vec()
}


#[test]
fn test_extract_non_negatives_rec() {

    let test_list = vec![0.8, -5.1, 1.6, -6.5, 10.5];

    let mut index: usize = 0;

    let mut non_ng_list: Vec<f32> = Vec::new();


    let res = extract_non_negatives(&test_list, &mut index , &mut non_ng_list);

    assert_eq!(
        res,
        [0.8, 1.6, 10.5]
    );
}
