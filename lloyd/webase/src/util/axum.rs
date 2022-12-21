use axum::{body::Bytes, BoxError};
use futures::{Stream, TryStreamExt};
use std::io::{self, Error};
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;

pub async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<u64, Error>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    let body_reader = StreamReader::new(body_with_io_error);
    futures::pin_mut!(body_reader);
    let path = std::path::Path::new(path);
    let mut file = BufWriter::new(File::create(path).await?);
    tokio::io::copy(&mut body_reader, &mut file).await
}
