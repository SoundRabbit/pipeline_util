use pipeline;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn data_new_u32vec() {
    assert_eq!(pipeline::Data::new(vec![1, 2, 3]).data, vec![1, 2, 3]);
}

// #[test]
// fn data_adapt_closure_u32vec_to_u32() {
//     assert_eq!(
//         pipeline::Data::new(vec![1, 2, 3])
//             .adapt(|v: Vec<u32>| { v[0] })
//             .data,
//         1
//     );
// }

// #[test]
// fn data_adapt_method_u32vec_to_u32() {
//     struct GetByIndex {
//         index: usize,
//     }
//     impl pipeline::Processor<Vec<u32>, u32> GetByIndex {
//         fn process(&self, v: Vec<u32>) -> u32 {
//             v[self.index]
//         }
//     }

//     assert_eq!(
//         pipeline::Data::new(vec![1, 2, 3])
//             .adapt(GetByIndex { index: 0 })
//             .data,
//         1
//     );
// }
