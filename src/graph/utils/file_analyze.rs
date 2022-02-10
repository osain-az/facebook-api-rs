/// This mod will contain different utils methods that could be use in different
/// mod
use serde::{Deserialize, Serialize};
use serde_json::ser::CharEscape::FormFeed;
//use web_sys::Blob;
//use web_sys::File;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Read, Seek};

//#[derive(Deserialize, Debug, Serialize)]
#[derive(Debug)]

pub struct FileResultServer {
    file_size_gb: f64,
    file_size_byte: f64,
    upload_raw_file: File,
    upload_method: String,
    chunk_upload_size: u64,
}

impl FileResultServer {
    /// This method will take the file  and return  a struct of   struc
    /// FileResult {  size_gb: f64,   file_byte: f64,   upload_method: String }
    pub fn file_analyze(file_path: String) -> FileResultServer {
        let non_resumable_max_size_gb = 1.0; // Gb: facebook recommmended max 1 Gb for none resumabl upload
        let _resumable_max_size_gb = 4.0; // Gb: facebook recommmended max max of 4  Gb for  resumabl uploading video
        let file = File::open(file_path).unwrap();

        let file_size_byte = file.metadata().unwrap().len();
        //f.sync_all()?;
        println!("avalize {:?}", file_size_byte.clone());
        // let file_size_byte = file.size() as f64; // file size in byte
        let file_size_gb = file_size_byte as f64 / 10_f64.powf(9.0); // convert the file to Gb
        println!("avalize size {:?}", file_size_gb.clone());
        let upload_method: String;
        if file_size_gb < non_resumable_max_size_gb {
            upload_method = "non_resumable".to_string();
        } else {
            // this will be for larger videos
            upload_method = "resumable".to_string();
        }
        println!("avalize  upload type {:?}", upload_method.clone());

        FileResultServer {
            file_size_byte: file_size_byte as f64,
            file_size_gb,
            upload_method,
            upload_raw_file: file,
            chunk_upload_size: 0,
        }
    }

    pub fn chunk_file(mut self, start: u8) -> [u8; 2] {
        // start_offset is the result from facebook response
        // The chunked size of the video to be uploaded  was initially supposed to be
        // determined by the sized of the  uploaded video but there are some
        // issue in send ing the request with the form data when large chunk blob file
        // is append to the FormDat untill this fixed, a pre-defined chunked
        // size of 5mb will be used  as default.

        let chunk_size: u64 = 120000000; // 15 mb

        let file_size = self.file_size_byte as u8;
        let mut buffer = [0, 0];

        if start as u64 + chunk_size >= file_size as u64 {
            buffer = [start, file_size]
        } else {
            buffer = [start, chunk_size.try_into().unwrap()]
        }

        self.upload_raw_file.read_exact(&mut buffer).unwrap();

        // self.chunk_upload_size = chunk_size;
        //self.chunk_upload_size = test_chunk_size;

        buffer
    }

    pub fn upload_chunking_size(self) -> u64 {
        self.chunk_upload_size
    }

    pub fn uploading_method(self) -> String {
        self.upload_method
    }

    pub fn file_size_byte_f64(self) -> f64 {
        self.file_size_byte
    }

    pub fn file_size_byte_string(self) -> String {
        let file_size = self.file_size_byte as u64;
        file_size.to_string()
    }
}
