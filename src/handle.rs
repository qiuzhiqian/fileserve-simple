use httparse::Request;
use mime_guess::Mime;
use std::fs::{DirEntry, File};
use std::io::prelude::*;
use std::net::*;
use std::path::Path;
use std::path::PathBuf;

pub enum HttpError {
    FailedRead(std::io::Error),
    FailedParse(httparse::Error),
    MissingField(HttpField),
    FailedWrite(std::io::Error),
}

pub enum HttpField {
    Version,
    Method,
    Path,
}

pub fn handle_connection(mut stream: &mut TcpStream, c: crate::config::Config/*directory: String*/) -> Result<(), HttpError> {
    let mut buf = [0; 1024];
    stream.read(&mut buf).map_err(HttpError::FailedRead)?;

    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut req = Request::new(&mut headers);
    let _status = req.parse(&buf).map_err(HttpError::FailedParse)?;

    handle_request(&req, &mut stream, &c)?;

    Ok(())
}

fn handle_request(
    req: &Request,
    mut stream: &mut TcpStream,
    c: &crate::config::Config
    /*directory: &str,*/
) -> Result<(), HttpError> {
    let (version, method, path) = all_fields(&req)?;
    println!(
        "Request:\n{} {} {} from {:?}",
        version,
        method,
        path,
        stream.peer_addr()
    );

    match fetch_path(path, &c.path,c.size) {
        Ok(FetchResult::Dir(html)) => {
            stream
                .write(
                    format!(
                        "HTTP/1.1 200 Ok\n\
                Content-Type: text/html; charset=utf-8\n\n{}",
                        html
                    )
                    .as_bytes(),
                )
                .map_err(HttpError::FailedWrite)?;
        }
        Ok(FetchResult::File(mut file, mime)) => {
            send_file(&mut stream, &mut file, mime).map_err(HttpError::FailedWrite)?;
        }
        Err(FetchError::FileNotFound) => {
            stream
                .write(
                    String::from(
                        "HTTP/1.1 404 Not Found\n\
        Content-Type: text/html; charset=utf-8\n\n\
        <h1> Error: File Not Found",
                    )
                    .as_bytes(),
                )
                .map_err(HttpError::FailedWrite)?;
        }
        Err(FetchError::IOError(_)) => {
            stream
                .write(
                    String::from(
                        "HTTP/1.1 500 Server Error\n\
            Content-Type: text/html; charset=utf-8\n\n\
            <h1> 500 Intenal Error",
                    )
                    .as_bytes(),
                )
                .map_err(HttpError::FailedWrite)?;
        }
    };
    Ok(())
}

fn send_file(stream: &mut TcpStream, file: &mut File, mime: Mime) -> Result<(), std::io::Error> {
    let start = format!("HTTP/1.1 200 Ok\nContent-Type: {mime}\n\n");
    let _sent = stream.write(start.as_bytes())?;

    let mut buf: [u8; 8192] = [0; 8192];
    loop {
        let amount = file.read(&mut buf)?;
        if amount > 0 {
            let _sent = stream.write(&buf[0..amount])?;
        } else {
            break Ok(());
        }
    }
}

fn all_fields<'r>(req: &'r Request) -> Result<(u8, &'r str, &'r str), HttpError> {
    let version = req
        .version
        .ok_or(HttpError::MissingField(HttpField::Version))?;
    let method = req
        .method
        .ok_or(HttpError::MissingField(HttpField::Method))?;
    let path = req.path.ok_or(HttpError::MissingField(HttpField::Path))?;
    Ok((version, method, path))
}

enum FetchError {
    FileNotFound,
    IOError(std::io::Error),
}

enum FetchResult {
    Dir(String),
    File(File, Mime),
}

fn fetch_path(path_str: &str, directory: &PathBuf,size: u32) -> Result<FetchResult, FetchError> {
    let path = directory.join(Path::new(path_str.trim_start_matches("/"))).canonicalize().map_err(FetchError::IOError)?;
    println!("path: {} {} {}",directory.to_str().unwrap(), path_str, path.to_str().unwrap());
    if path.is_dir() {
        let title = format!("Directory listing for {}", path_str);
        let start = format!(
            "<!DOCTYPE HTML><html><head><title>{}</title></head><body><h1>{}</h1><hr><ul>",
            title, title
        );
        let end = "</ul></body></html>";

        let mut page = start;
        let mut entries: Vec<DirEntry> = path
            .read_dir()
            .map_err(FetchError::IOError)?
            .flatten()
            .collect::<Vec<DirEntry>>();
        let cmp = |a: &DirEntry, b: &DirEntry| {
            let check = (a.path().is_dir(), b.path().is_dir());
            match check {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            }
        };
        entries.sort_by(cmp);
        add_filename_to_page("..", &mut page, size);
        for entry in entries {
            let filename_osstr = entry.file_name();
            let filename =
                filename_osstr.to_string_lossy() + if entry.path().is_dir() { "/" } else { "" };
            add_filename_to_page(&filename, &mut page, size);
        }
        page.push_str(end);
        Ok(FetchResult::Dir(page))
    } else {
        match File::open(&path) {
            Ok(file) => Ok(FetchResult::File(
                file,
                mime_guess::from_path(path).first_or_octet_stream(),
            )),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Err(FetchError::FileNotFound),
                _ => Err(FetchError::IOError(e)),
            },
        }
    }
}

fn add_filename_to_page(filename: &str, page: &mut String,size: u32) {
    page.push_str(format!("<li style='font-size:{}px'><a href={}>{}</a></li>", size, filename, filename).as_str());
}
