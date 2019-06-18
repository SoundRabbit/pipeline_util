use super::pipeline;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn data_new_u32vec() {
    assert_eq!(pipeline::Data::new(vec![1, 2, 3]).data, vec![1, 2, 3]);
}
