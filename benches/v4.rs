#![feature(test)]
extern crate test;

#[cfg(feature = "v4")]
mod v4 {
    use test::Bencher;
    use uuid::Uuid;

    #[bench]
    fn create(b: &mut Bencher) {
        b.iter(|| {
            let _ = test::black_box(Uuid::new_v4());
        });
    }

    #[bench]
    fn create_using_rand_threadrng(b: &mut Bencher) {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        b.iter(|| {
            let mut bytes = [0u8; 16];
            rng.fill_bytes(&mut bytes);

            let _ = uuid::Builder::from_bytes(test::black_box(bytes))
                .set_variant(uuid::Variant::RFC4122)
                .set_version(uuid::Version::Random)
                .build();
        });
    }
}
