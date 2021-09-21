///
/// This mod will contain different utils methods that could be use in different  mod
///
use seed::{prelude::*, *};
use web_sys::File;
  use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug, Default, Serialize)]
 pub struct FileResult {
    file_size_gb: f64,
    file_size_byte: f64,
    upload_method: String,
}

 impl  FileResult {

///
    /// This method will take the file  and return  a struct of   struc FileResult {  size_gb: f64,   file_byte: f64,   upload_method: String }
    ///
 pub fn file_analize( file: File) -> FileResult{
    let non_resumable_max_size_gb = 0.9; //Gb: facebook recommmended max 1 Gb for none resumabl upload
    let _resumable_max_size_gb = 4.0; //Gb: facebook recommmended max max of 4  Gb for  resumabl uploading video
    let file_size_byte = file.size() as f64; // file size in byte
    let file_size_gb = file_size_byte / 10_f64.powf(9.0); // convert the file to Gb
    let upload_method: String;

    if file_size_gb <= non_resumable_max_size_gb {
        upload_method = "non_resumable".to_string();
    } else {
        // this will be for larger videos
        upload_method = "resumable".to_string();
    }
    FileResult{
       file_size_byte,
         file_size_gb,
        upload_method,
    }
  
}
  pub  fn get_upload_method(self) -> String {
      self.upload_method
  }
}