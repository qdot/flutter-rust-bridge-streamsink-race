use flutter_rust_bridge::StreamSink;
use anyhow::Result;
use once_cell::sync::OnceCell;
use lazy_static::lazy_static;

lazy_static! { static ref LIBRARY_STREAM_SINK: OnceCell<StreamSink<String>> = OnceCell::new(); }

pub fn try_sink() {
  println!("In try sink!");
  if let Some(stream) = LIBRARY_STREAM_SINK.get() {
    println!("STREAM AVAILABLE ON CALL");
    stream.add("Testing 1 2 3!".to_owned());
  } else {
    println!("NO STREAM AVAILABLE ON CALL");
  }
}

pub fn set_sink(sink: StreamSink<String>) -> Result<()> {
  sink.add("Testing 1 2 3 on creation!".to_owned());
  LIBRARY_STREAM_SINK.set(sink);
  if let Some(stream) = LIBRARY_STREAM_SINK.get() {
    println!("STREAM AVAILABLE ON CREATION");
  } else {
    println!("NO STREAM AVAILABLE ON CREATION");
  }
  Ok(())
}
