use criterion::{black_box, criterion_group, criterion_main, Criterion};

const TEST_DATA: &[u8; 1828] = include_bytes!("../../geozero/tests/data/tile.mvt");

fn mvt_benchmark(c: &mut Criterion) {
    c.bench_function("decode MVT using prost", |b| {
        b.iter(|| {
            use geozero::mvt::Message;
            use geozero::mvt::Tile;
            let data = black_box(TEST_DATA);
            let tile: Tile = Tile::decode(data).unwrap();
            tile
        })
    });

    c.bench_function("decode MVT using protobuf", |b| {
        b.iter(|| {
            use geozero::mvt::vector_tile2::Tile;
            let data = black_box(TEST_DATA);
            let tile: Tile = protobuf::Message::parse_from_bytes(data).unwrap();
            tile
        })
    });

    c.bench_function("decode MVT using quick protobuf", |b| {
        b.iter(|| {
            use geozero::mvt::vector_tile3::Tile;
            // use quick_protobuf::message::MessageRead;
            use quick_protobuf::{BytesReader, MessageRead};

            // we can build a bytes reader directly out of the bytes
            let data = black_box(TEST_DATA);
            let mut reader = BytesReader::from_bytes(&data);
            let tile = Tile::from_reader(&mut reader, &data).expect("Cannot read FooBar");
            tile
        })
    });
}

criterion_group!(benches, mvt_benchmark);
criterion_main!(benches);
