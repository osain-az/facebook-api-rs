//! Facebook graph api for video uplaoding video by file upload
//!
//! This mod contains struct and methods for uploading video to facebook API.
use seed::fetch::{fetch, FormData};
use seed::prelude::{Method, Request};

use seed::{prelude::*, *};
use web_sys::File;
// use gloo_file::File;
use crate::graph::utils::FileResult;
use serde::{Deserialize, Serialize};
use web_sys::Blob;

// odo need link to official guide on facebook.     done
// https://developers.facebook.com/docs/graph-api/reference/page/videos/#Creating
// guide     https://developers.facebook.com/docs/video-api/guides/publishing
// odo move under page    // done

/// Facebook video api accepts different paramters that could be passed posted
/// while uploading the video. this struck will have the possible pramaters that
/// a user might need to pass along the video upload Note : video_title,
/// file_name, and title will not appera in your feed. use "description"
/// to describe your video  which will appear at the top of the feed
#[derive(Clone)]
pub struct VideoParams {
    ///
    pub video_title: String,
    pub description: String,
    pub thum: String,
    pub content_category: ContentCategory,
    pub file_name: String,
    pub title: String,
    // file: File,// add this file type here
}

pub struct UploadFile {
    pub file: File,
}

/// Enum for the different categories that the uploaded video will belong to as
/// difined on facebook  graph api documentation. Choose any from the list, if
/// no data is supplied a default value of  "OTHER" is choosen.
#[derive(Deserialize, Serialize)]
pub struct PostResponse {
    id: String,
}

/// Enum for the different categories that the uploaded video will belong to as
/// defined on facebook graph api documentation. Choose any from the list, if no
/// data is supplied a default value of "OTHER" is choosen.
#[derive(Deserialize, Copy, Clone, Serialize)]
pub enum ContentCategory {
    EAUTY_FASHION,
    BUSINESS,
    CARS_TRUCKS,
    COMEDY,
    CUTE_ANIMALS,
    ENTERTAINMENT,
    FAMILY,
    FOOD_HEALTH,
    HOME,
    LIFESTYLE,
    MUSIC,
    NEWS,
    POLITICS,
    SCIENCE,
    SPORTS,
    TECHNOLOGY,
    VIDEO_GAMING,
    OTHER,
}

// This struct is the response gotten when initiazing the resumable uploading
// process
#[derive(Deserialize, Debug, Clone, Serialize)]
struct InitialuzeUploadResponse {
    pub video_id: String,
    pub end_offset: String,
    pub upload_session_id: String,
}

/// This struct is the struct that is send back upon sucessfull  upload of the
/// video. The struct is constructed  using different data gotten from different
/// responses while using the resumable  method. if the success parameter in the
/// struct is true then the video was uploaded sucessfully
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct FinalResponeResumableUpload {
    // this struct will be data constructed from all the different uploads
    success: bool,
    upload_session_id: String, // will developer need this
    video_id: String,
}

// this will be used to update this method upon each round of chunked upload
impl FinalResponeResumableUpload {
    fn update_params(
        mut self,
        video_id: String,
        upload_session_id: String,
    ) -> FinalResponeResumableUpload {
        self.upload_session_id = upload_session_id;
        self.video_id = video_id;
        self
    }

    pub fn new(
        video_id: String,
        upload_session_id: String,
        success: bool,
    ) -> FinalResponeResumableUpload {
        FinalResponeResumableUpload {
            video_id,
            upload_session_id,
            success,
        }
    }

    fn update_success(mut self, success: bool) -> FinalResponeResumableUpload {
        self.success = success;
        self
    }

    /// This method will return the struct of all the paramters
    pub fn get_response(self) -> FinalResponeResumableUpload {
        self
    }
}

// After complete uploading of the video through resumable, facebook will send
// reponse which will be either true or false true means, the video was uplaoded
// successfull

#[derive(Deserialize, Serialize)]
struct ResumableUploadFinal {
    // response from facebook, true or false
    success: bool,
}

// During each chunk blob file uploaded, facebook will send a response back,
// This struct is the response gotten for each video chunk  sent

#[derive(Deserialize, Clone, Serialize)]
struct ChunksUploadResponse {
    start_offset: String, // Value for second chunk
    end_offset: String,
}

impl ChunksUploadResponse {
    fn new(start_offset: String, end_offset: String) -> ChunksUploadResponse {
        ChunksUploadResponse {
            start_offset,
            end_offset,
        }
    }

    fn get_end_offset(self) -> String {
        self.end_offset
    }

    fn get_start_offset(self) -> String {
        self.start_offset
    }
}

enum UploadPhase {
    start,
    transfer,
    finish,
    cancel,
}

impl Default for VideoParams {
    /// this default method will create the struct will default
    fn default() -> Self {
        Self {
            video_title: "".to_string(),
            description: "video feed".to_string(),
            thum: "".to_string(),
            content_category: ContentCategory::OTHER,
            file_name: "file".to_string(),
            title: " ".to_string(),
        }
    }
}

impl VideoParams {
    pub fn new() -> VideoParams {
        VideoParams::default()
    }

    pub fn update_video_params(mut self, video_params: VideoParams) -> Self {
        Self { ..video_params }
    }

    pub fn get_video_params(self) -> VideoParams {
        self
    }
}

#[derive(Deserialize, Clone, Serialize)]
pub struct VideoApi {
    base_url: String,
    page_access_token: String,
}

impl VideoApi {
    pub fn new(base_url: String, page_access_token: String) -> VideoApi {
        VideoApi {
            base_url,
            page_access_token,
        }
    }

    // This form method will be used by one Non_resumable uplaod method

    fn create_form_data(file: File, video_params: VideoParams) -> FormData {
        let mut form_data = FormData::new();
        form_data.append_blob("source", &file); // appped  the  current chunked file   to the form

        if !video_params.video_title.is_empty() {
            form_data.append_str("video_title", &video_params.video_title);
        }

        if !video_params.description.is_empty() {
            form_data.append_str("description", &video_params.description);
        }

        if !video_params.thum.is_empty() {
            form_data.append_str("thum", &video_params.thum);
        };

        form_data
    }

    /// This method is expecting a video file less than 1 gb,  if the vide file
    /// is within this range it feed the video but if the video is not
    /// within the range , the feed will not be made but a Fetcherror will be
    /// gerated.
    pub async fn post_video(
        &self,
        video_params: VideoParams,
        file: File,
    ) -> seed::fetch::Result<PostResponse> {
        let uploaded_file = file.clone();
        let upload_method = FileResult::file_analize(file).uploading_method();

        if upload_method == "non_resumable" {
            let form_data = VideoApi::create_form_data(uploaded_file, video_params);
            let base_url = self.base_url.replace("EDGE", "videos");
            let url = base_url + "?access_token=" + &self.page_access_token;
            let request = Request::new(url).method(Method::Post).form_data(form_data);
            fetch(request).await?.json::<PostResponse>().await
        } else {
            let err = JsValue::from_str("the uplaoded file is above 1 gb, use Resumable method ");
            Err(FetchError::RequestError(err)) // try to generate a customer
                                               // error
        }
    }
}

impl VideoApi {
    // creating formData for the  resumable video method
    fn resumable_formData(
        self,
        upload_phase: UploadPhase,
        current_blob_file: Blob,
        uploaded_file: File,
        upload_session_id: String,
        start_offset: String,
        video_params: VideoParams,
    ) -> FormData {
        // phase is expected to be of the this enum , start, transfer and finis. this
        // will control w
        let mut current_upload_phase = "";
        let mut form_data = FormData::new();
        // let chunked_file = file.slice_with_i32_and_i32()

        match upload_phase {
            UploadPhase::start => {
                current_upload_phase = "start";
                form_data.append_str(
                    "file_size",
                    FileResult::file_analize(uploaded_file)
                        .file_size_byte_string()
                        .as_str(),
                ); // add the video size
            }

            UploadPhase::transfer => {
                current_upload_phase = "transfer";
                form_data.append_str("start_offset", &start_offset);
                form_data.append_blob("video_file_chunk", &current_blob_file);
                // form_data.append_blob("video_file_chunk",
                // &current_blob_file);
            }

            UploadPhase::finish => {
                current_upload_phase = "finish";

                form_data.append_str("upload_session_id", &upload_session_id);
                if !video_params.video_title.is_empty() {
                    form_data.append_str("video_title", &video_params.video_title);
                }

                if !video_params.description.is_empty() {
                    form_data.append_str("description", &video_params.description);
                }

                if !video_params.thum.is_empty() {
                    form_data.append_str("thum", &video_params.thum);
                };
            }

            UploadPhase::cancel => {
                form_data.append_str("upload_session_id", &upload_session_id);
                form_data.append_str("start_offset", &start_offset);
            }
        }

        form_data.append_str("upload_session_id", &upload_session_id);
        form_data.append_str("upload_phase", current_upload_phase);
        form_data.append_str("access_token", &self.page_access_token);

        form_data
    }

    /// This method is used for uploading large video files, it does that by
    /// chunking the file and uplaoding them individually until is complete.
    /// The method takes two parameter( file, feed parameter struct).
    /// the waiting time depend  on the video size uplaoded
    ///
    /// Note there is an issue with chunking method that only chunk smaller size
    /// so extra time than usuall expect until the issue is fixed.
    ///  
    pub async fn resumable_post(
        &self,
        file: File,
        video_param: VideoParams,
    ) -> seed::fetch::Result<FinalResponeResumableUpload> {
        let uploaded_file = file.clone();
        let mut start_offset = Some("0".to_string()); // this  data will be updated  frpm the respones
        let mut end_offset = Some("0".to_string()); // this  data will be updated  frpm the respones
        let video_params = video_param.clone();

        //  let video_params =
        // VideoParams::new().update_video_params(video_param.clone()).clone();
        let self_data = self.clone();
        let base_url = self.base_url.replace("EDGE", "videos").clone();
        let mut form_data = self_data.resumable_formData(
            UploadPhase::start,
            Blob::new().unwrap(),
            uploaded_file.clone(),
            "".to_string(),
            "0".to_string(),
            video_param.clone(),
        );
        let url = base_url + "?access_token=" + &self.page_access_token;

        let request = Request::new(url).method(Method::Post).form_data(form_data);

        let response = fetch(request)
            .await?
            .json::<InitialuzeUploadResponse>()
            .await;

        let start_phase_data = response.unwrap();
        end_offset = Some(start_phase_data.end_offset); // update from the facebook response
        let chunked_file_data = FileResult::file_analize(file.clone()).clone();

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

        if !start_phase_data.upload_session_id.is_empty() {
            // check if the  first request was sucessfull, if there is an  upload_session_id
            // it means it was successfull
            let mut final_response_status = false;
            let mut current_chunk_size = chunk_size as f64;
            let mut start_chunk = 0.0;
            let uploaded_file = file.clone();
            let upload_session_id = &start_phase_data.upload_session_id;

            // loop and upload the chunked files until is completed then end the loop

            while let Some(end_offset_status) = end_offset.clone() {
                if current_chunk_size > file.size() {
                    // update the current chunking sizing
                    current_chunk_size = file.size()
                };

                if let Some(start_offset_status) = start_offset.clone() {
                    //   start_offset_status = tee.clone();
                    let self_data = self.clone();

                    if end_offset_status != start_offset_status {
                        let base_url = self.base_url.replace("EDGE", "videos");
                        let form_datas = self_data.resumable_formData(
                            UploadPhase::transfer,
                            FileResult::file_analize(file.clone())
                                .chunk_file(start_chunk, current_chunk_size)
                                .chunked_file(),
                            file.clone(),
                            upload_session_id.to_string(),
                            start_offset_status,
                            video_params.clone(),
                        );

                        let request = Request::new(base_url)
                            .method(Method::Post)
                            .form_data(form_datas);
                        let response = fetch(request).await?.json::<ChunksUploadResponse>().await;

                        let chunk_upload_response = response.unwrap();
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
                        let self_data = self.clone();
                        let base_url = self.base_url.replace("EDGE", "videos");

                        // There is and issue with the formdata for blob chunked file, it can only
                        // take small sized of blob file, this making the
                        // upload to take longer than expect. if larger blob
                        // file is appeded to the formData, the data will not be posted along the
                        // request causing it erorr.
                        // allthough the formData can take large file that are not chunked, thefore
                        // it is not certain   where the issue is coming
                        //

                        let form_data = self_data.resumable_formData(
                            UploadPhase::finish,
                            Blob::new().unwrap(), // not important in the uplaod phase
                            uploaded_file.clone(), // this file is no longer important
                            upload_session_id.to_string(),
                            "".to_string(),
                            video_param.clone(),
                        );

                        let request = Request::new(base_url)
                            .method(Method::Post)
                            .form_data(form_data);
                        final_response_status = fetch(request)
                            .await?
                            .json::<ResumableUploadFinal>()
                            .await
                            .unwrap()
                            .success;

                        end_offset = None // end the while loop
                    }
                }
            }

            // after the chunk file upload has is completed,  trigger and Ok response to
            // send the data.
            Ok(final_response.update_success(final_response_status))
        } else {
            let err = JsValue::from_str("The video upload initialization was not sucessfull, try upload again  or try with another video  ");
            Err(FetchError::RequestError(err)) // try to generate a customer
                                               // error
        }
    }
}

// should we implement a general method that  take file and determine which
// method to use or, user can easily che check the file size and decide which
// method to use

// still under consideration
impl VideoApi {
    pub fn general_video(self, video_params: VideoParams, file: File) {
        let uploading_method = FileResult::file_analize(file.clone()).uploading_method(); // this will return the uploading method based on the size       ;

        if uploading_method == "non_resumable" {
            // this means file can be upload with non  resumable method.
            self.post_video(video_params, file);
        } else {
            self.resumable_post(file, video_params);
        }
    }
}
