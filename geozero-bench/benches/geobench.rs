use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn mvt_benchmark(c: &mut Criterion) {
    c.bench_function("decode MVT using prost", |b| {
        b.iter(|| {
            use geozero::mvt::Message;
            use geozero::mvt::Tile;
            let data = &include_bytes!("../../geozero/tests/data/tile.mvt")[..];
            let tile: Tile = Tile::decode(black_box(data)).unwrap();
            tile
        })
    });

    c.bench_function("decode MVT using protobuf", |b| {
        b.iter(|| {
            use geozero::mvt::vector_tile2::Tile;
            let data = &include_bytes!("../../geozero/tests/data/tile.mvt")[..];
            let tile: Tile = protobuf::Message::parse_from_bytes(black_box(data)).unwrap();
            tile
        })
    });

    c.bench_function("decode MVT using quick protobuf", |b| {
        b.iter(|| {
            use geozero::mvt::vector_tile3::Tile;
            // use quick_protobuf::message::MessageRead;
            use quick_protobuf::{BytesReader, MessageRead};

            let data = &include_bytes!("../../geozero/tests/data/tile.mvt")[..];

            // we can build a bytes reader directly out of the bytes
            let mut reader = BytesReader::from_bytes(&data);
            let tile = Tile::from_reader(&mut reader, &data).expect("Cannot read FooBar");
            tile
        })
    });
}

criterion_group!(benches, mvt_benchmark);
criterion_main!(benches);
