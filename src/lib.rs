pub mod pipeline;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

	#[test]
	fn adaptor_u32_new() {
		let adaptor_u32 = pipeline::Adaptor::new(vec![1, 2, 3, 4]);
	}
}
