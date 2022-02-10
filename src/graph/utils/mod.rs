/// This mod will contain different utils methods that could be use in different
/// mod
use serde::{Deserialize, Serialize};
use serde_json::ser::CharEscape::FormFeed;
#[cfg(any(feature = "seed_async"))]
use web_sys::Blob;
#[cfg(any(feature = "seed_async"))]
use web_sys::File;

#[cfg(any(feature = "reqwest_async"))]
pub mod file_analyze;

//#[derive(Deserialize, Debug, Serialize)]
#[derive(Clone, Debug)]
#[cfg(any(feature = "seed_async"))]
pub struct FileResult {
    file_size_gb: f64,
    file_size_byte: f64,
    pub(crate) upload_raw_file: File,
    upload_method: String,
    chunked_file: Blob,
    chunk_upload_size: u64,
}
#[cfg(any(feature = "seed_async"))]
impl FileResult {
    /// This method will take the file  and return  a struct of   struc
    /// FileResult {  size_gb: f64,   file_byte: f64,   upload_method: String }
    pub fn file_analyze(file: File) -> FileResult {
        let non_resumable_max_size_gb = 1.0; // Gb: facebook recommmended max 1 Gb for none resumabl upload
        let _resumable_max_size_gb = 4.0; // Gb: facebook recommmended max max of 4  Gb for  resumabl uploading video

        let file_size_byte = file.size() as f64; // file size in byte
        let file_size_gb = file_size_byte / 10_f64.powf(9.0); // convert the file to Gb

        let upload_method: String;
        if file_size_gb < non_resumable_max_size_gb {
            upload_method = "non_resumable".to_string();
        } else {
            // this will be for larger videos
            upload_method = "resumable".to_string();
        }
        FileResult {
            file_size_byte,
            file_size_gb,
            upload_method,
            upload_raw_file: file,
            chunked_file: Blob::new().unwrap(),
            chunk_upload_size: 0,
        }
    }

    pub fn chunk_file(mut self, start: f64, current_blob_size: f64) -> FileResult {
        // start_offset is the result from facebook response
        // The chunked size of the video to be uploaded  was initially supposed to be
        // determined by the sized of the  uploaded video but there are some
        // issue in send ing the request with the form data when large chunk blob file
        // is append to the FormDat untill this fixed, a pre-defined chunked
        // size of 5mb will be used  as default.

        let test_chunk_size = 5000000; // this will be the test chunked until the issue of large blob in the formdata
                                       // is fixed.
        let gb_to_byte = 1.0_f64.powf(9.0); // equavalent to 1gb
        let half_gb_to_byte = 5.4_f64.powf(8.0); // equavalent to 1gb
        let mut chunk_size: u64;
        let file_size = self.upload_raw_file.size();
        if file_size < half_gb_to_byte {
            // if the file is less the 0.5,  the chunk size should be the size/2
            chunk_size = file_size as u64 / 2;
        } else if file_size < gb_to_byte {
            // if the while is less the greater than 0.5 but less than 1gb, splite the
            // process into 3 chunks
            chunk_size = file_size as u64 / 3;
        } else {
            // if the while is less the greater than 0.5 but less than 1gb, splite the
            // process into 3 chunks
            chunk_size = file_size as u64 / 5;
        }

        // let nunbers_of_chunks = ceil((file_size as u64 / chunk_size) as f64);
        // let start = 0;
        // let chunk_end = min(file_size, (start + chunk_size) as f64); // this is the
        // ending chunk

        // blob_step_start // an incriment value

        let chunked_file = self
            .upload_raw_file
            .slice_with_f64_and_f64(start, current_blob_size)
            .unwrap();

        // self.chunk_upload_size = chunk_size;
        self.chunk_upload_size = test_chunk_size;

        self.chunked_file = chunked_file;
        self
    }

    pub fn chunked_file(self) -> Blob {
        self.chunked_file
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
