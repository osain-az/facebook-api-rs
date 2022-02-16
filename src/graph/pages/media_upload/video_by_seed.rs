//! The Facebook Video API allows you to publish Videos on Pages and Groups.
//! Publishing on Users is not supported.
//!
//! The process for publishing Videos involves choosing an upload protocol and
//! sending a POST request to the targeted Page or Group's  to end point /videos
//! edge. The API  supports  postting video by video_url, Resumable and
//! Non-Resumableupload protocols.
//! Facebook recommend that you use the Resumable Upload protocol
//! as it is more  versatile and can gracefully handle connection interruptions.
//! To post to either page or group pass either of the id.  (/event-id/videos,
//! /page-id/videos /group-id/videos
//!
//! For other information check on facebook documentation  for
//! video API  <https://developers.facebook.com/docs/video-api/guides/publishing>
//! For information on different opertaions on facebook page  check  <https://developers.facebook.com/docs/graph-api/reference/page/videos/#Creating>
use crate::graph::utils::FileResult;
use crate::prelude::errors::ClientErr;
use crate::prelude::HttpConnection;
//use seed::fetch::{fetch, FormData};
//use seed::{prelude::*, *};
use crate::prelude::utils::{form_data_seed, resumable_form_data_seed, PostResponse};
use crate::prelude::utils::{ChunksUploadResponse, UploadingData};

use crate::prelude::video::{FinalResponeResumableUpload, UploadPhase, VideoParams};
use serde::{Deserialize, Serialize};
use web_sys::{Blob, File, FormData};
//use seed::fetch::FormData;

/// Facebook video api accepts different parameters that could be passed to the
/// post request while uploading the video. this struck will have the possible
/// parameters that a user might need to pass along the video while publishing.
/// Note : video_title, file_name, and title will not appear in your feed. use
/// "description" to describe your video  which will appear at the top of the
/// post.

pub struct UploadFile {
    #[cfg(any(feature = "seed_async"))]
    pub file: File,
    pub file_path: String,
}

#[derive(Deserialize, Serialize)]
struct ResumableUploadFinal {
    // response from facebook, true or false
    success: bool,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
struct InitializeUploadResponse {
    pub video_id: String,
    pub end_offset: String,
    pub upload_session_id: String,
}

/// This struct is the response gotten when initializing the resumable uploading
/// method process.

#[derive(Deserialize, Clone, Serialize, Default)]
pub struct VideoApi_seed {
    base_url: String,
    page_access_token: String,
}

impl VideoApi_seed {
    pub fn new(base_url: String, page_access_token: String) -> VideoApi_seed {
        VideoApi_seed {
            base_url,
            page_access_token,
        }
    }

    /// facebook recommend that you upload files using the Resumable Upload
    /// method because it handles connection interruptions more efficiently
    /// and supports larger files. However, if you prefer to upload files
    /// using the Non-Resumable Upload method.
    ///
    /// This method is expecting a video file less than 1 gb, and a video
    /// parameter struct,  if the video file is within this range it post
    /// the video but if the video is not within the range , the post will
    /// not be made but a Fetcherror will be gerated.
    ///
    /// For  more information cehck   https://developers.facebook.com/docs/video-api/guides/publishing

    pub async fn non_resumable_post(
        &self,
        video_params: VideoParams,
        file: File,
    ) -> Result<PostResponse, ClientErr> {
        let uploaded_file = file.clone();
        let file_result = FileResult::file_analyze(file);
        // check if the uploading method
        if file_result.uploading_method() == "non_resumable" {
            let form_data = form_data_seed(uploaded_file, video_params);
            let base_url = self.base_url.replace("EDGE", "videos");
            let url = base_url + "?access_token=" + &self.page_access_token;

            let resp = HttpConnection::video_post::<PostResponse>(url, form_data).await?;
            Ok(resp)
        } else {
            Err(ClientErr::FacebookError(
                "The uplaoded file is above 1 gb, use Resumable method ".to_string(),
            )) // try to generate a customer
               // error
        }
    }
}

impl VideoApi_seed {
    /// This method is used for uploading large video files, it does that by
    /// chunking the file and uplaoding them individually until is complete.
    /// The method takes two parameter( file, video parameter struct).
    /// the waiting time depend  on the video size uplaoded
    ///
    /// Note there is an issue with chunking method that only chunk smaller size
    /// so extra time than usuall is expect until the issue is fixed.
    ///  
    /// for more infromation  check  https://developers.facebook.com/docs/video-api/guides/publishing
    ///
    pub async fn resumable_post(
        &self,
        file: File,
        video_param: VideoParams,
    ) -> Result<FinalResponeResumableUpload, ClientErr> {
        let uploaded_file = file.clone();
        let mut start_offset = Some("0".to_string()); // this  data will be updated  fopm the respones
        let mut end_offset = Some("0".to_string()); // this  data will be updated  fopm the respones
        let video_params = video_param.clone();
        let self_data = self.clone();

        let base_url = self.base_url.replace("EDGE", "videos").clone();
        let mut form_data = resumable_form_data_seed(
            UploadPhase::start,
            Blob::new().unwrap(),
            uploaded_file.clone(),
            "".to_string(),
            "0".to_string(),
            video_param.clone(),
        );

        let url = base_url.clone() + "?access_token=" + &self.page_access_token;

        let response =
            HttpConnection::video_post::<InitializeUploadResponse>(url, form_data).await?;

        let start_phase_data = response;

        end_offset = Some(start_phase_data.end_offset); // update from the facebook response
        let chunked_file_data = FileResult::file_analyze(file.clone()).clone();

        let final_response = FinalResponeResumableUpload::default().update_params(
            start_phase_data.video_id.clone(),
            start_phase_data.upload_session_id.clone(),
        ); // update some of the parameter of final response

        let chunk_size = chunked_file_data
            .clone()
            .chunk_file(0.0, 0.0)
            .upload_chunking_size(); // get the size of each chunk:  Note: the zero passed in  is just a dommy data
                                     // when estimating the chunking size
                                     // let uploaded_file = file.clone();
        let chunk_size = chunked_file_data.file_size_byte_f64();
        // get the size of each chunk:  Note: the zero passed in  is just a dommy data
        // when estimating the chunking size
        // let uploaded_file = file.clone();

        if !start_phase_data.upload_session_id.is_empty() {
            // check if the  first request was sucessfull, if there is an  upload_session_id
            // it means it was successfull
            let mut final_response_status = false;
            let mut current_chunk_size = chunk_size;
            let mut start_chunk = 0.0;
            //let uploaded_file = file.clone();
            let upload_session_id = &start_phase_data.upload_session_id;

            // loop and upload the chunked files until is completed then end the loop

            while let Some(end_offset_status) = end_offset.clone() {
                #[cfg(any(feature = "seed_async"))]
                if current_chunk_size > file.size() {
                    // update the current chunking sizing
                    current_chunk_size = file.size()
                };

                // check if there is a response data from the initialzition request
                if let Some(start_offset_status) = start_offset.clone() {
                    //   start_offset_status = tee.clone();
                    let self_data = self.clone();

                    if end_offset_status != start_offset_status {
                        let base_url = self.base_url.replace("EDGE", "videos");
                        let url =
                            base_url.clone() + "?access_token=" + &self.page_access_token.clone();

                        let form_data = resumable_form_data_seed(
                            UploadPhase::transfer,
                            FileResult::file_analyze(file.clone())
                                .chunk_file(start_chunk, current_chunk_size)
                                .chunked_file(),
                            file.clone(),
                            upload_session_id.to_string(),
                            start_offset_status.clone(),
                            video_params.clone(),
                        );

                        let response =
                            HttpConnection::video_post::<ChunksUploadResponse>(url, form_data)
                                .await?;

                        let chunk_upload_response = response;

                        let result = ChunksUploadResponse::new(
                            chunk_upload_response.start_offset,
                            chunk_upload_response.end_offset,
                        )
                        .clone();
                        start_offset = Some(result.start_offset); // == start_offset_status
                        end_offset = Some(result.end_offset);

                        start_chunk = current_chunk_size; // update the start chunk file
                        current_chunk_size += chunk_size as f64;
                    } else {
                        let base_url = self.base_url.replace("EDGE", "videos");
                        let url = base_url.clone() + "?access_token=" + &self.page_access_token;

                        // There is an issue with the formdata for blob chunked file, it can only
                        // take small sized of blob file, this making the
                        // upload to take longer than expect. if larger blob
                        // file is appeded to the formData, the data will not be posted along the
                        // request causing an erorr.
                        // allthough the formData can take large file that are not chunked, thefore
                        // it is not certain   where the issue is coming
                        //
                        let form_data = resumable_form_data_seed(
                            UploadPhase::finish,
                            Blob::new().unwrap(), // not important in the uplaod phase
                            file.clone(),
                            upload_session_id.to_string(),
                            "".to_string(),
                            video_params.clone(),
                        );

                        let resp =
                            HttpConnection::video_post::<ResumableUploadFinal>(url, form_data)
                                .await?;
                        let uploadind_data = UploadingData::default();

                        final_response_status = resp.success.clone();
                        end_offset = None // end the while loop
                    }
                }
            }

            // after the chunk file upload is completed,  trigger and Ok response to send
            // the data.
            Ok(final_response.update_success(final_response_status))
        } else {
            Err(ClientErr::FacebookError("The video upload initialization was not sucessfull, try upload again  or try with another video  ".to_string()))
            // try to generate a customer
            // error
        }
    }
}
