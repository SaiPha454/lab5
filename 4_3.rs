fn main() {
    let test_list = [1.0,-1.0,2.0,-2.0];

    let res = split_non_negatives(&test_list);


    dbg!(res);


}


fn split_non_negatives(input_list : &[f32])-> (Vec<f32>, Vec<f32>) {
    
    if(input_list.len() == 0 ) {
        return (Vec::new(), Vec::new());
    }

    let mut ng_list = Vec::new();
    let mut non_ng_list = Vec::new();

    for f in input_list {
        if *f < 0.0 {
            ng_list.push(*f);
        }else {
            non_ng_list.push(*f);
        }
    }

    ( non_ng_list, ng_list)
}



#[test]
fn test_split_non_negatives() {

    assert_eq!(split_non_negatives(&[]), (Vec::new(), Vec::new()));

    assert_eq!(
        split_non_negatives(& vec![0.8, -5.1, 1.6, -6.5, 10.5]),
        (
            vec![0.8, 1.6, 10.5],
            vec![-5.1, -6.5]
        )
    );
}
