use crate::presenter::less::LessPresenter;
use crate::presenter::Present;
use crate::reader::linemux_reader::LinemuxReader;
use crate::reader::AsyncLineReader;
use crate::writer::temp_file::TempFileWriter;
use crate::writer::AsyncLineWriter;
use async_trait::async_trait;
use tokio::io;

use tokio::sync::oneshot::Sender;

pub struct Io {
    reader: Box<dyn AsyncLineReader + Send>,
    writer: Box<dyn AsyncLineWriter + Send>,
}

pub struct Presenter {
    presenter: Box<dyn Present + Send>,
}

pub async fn create_io_and_presenter(
    file_path: String,
    number_of_lines: usize,
    reached_eof_tx: Option<Sender<()>>,
) -> (Io, Presenter) {
    let reader = LinemuxReader::new(file_path, number_of_lines, reached_eof_tx)
        .await
        .unwrap();

    let (writer, temp_file_path) = TempFileWriter::new().await;

    let io = Io {
        reader: Box::new(reader),
        writer: Box::new(writer),
    };

    let presenter = LessPresenter::new(temp_file_path, false);
    let presenter = Presenter { presenter };

    (io, presenter)
}

#[async_trait]
impl AsyncLineReader for Io {
    async fn next_line(&mut self) -> io::Result<Option<String>> {
        self.reader.next_line().await
    }
}

#[async_trait]
impl AsyncLineWriter for Io {
    async fn write_line(&mut self, line: &str) -> io::Result<()> {
        self.writer.write_line(line).await
    }
}

impl Present for Presenter {
    fn present(&self) {
        self.presenter.present()
    }
}