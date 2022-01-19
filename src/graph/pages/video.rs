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
use crate::prelude::file_analyze::FileResultServer;
#[cfg(any(feature = "reqwest_async"))]
use ::reqwest::multipart::Form;
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "seed_async"))]
use web_sys::{Blob, File, FormData};
//use seed::fetch::FormData;

/// Facebook video api accepts different parameters that could be passed to the
/// post request while uploading the video. this struck will have the possible
/// parameters that a user might need to pass along the video while publishing.
/// Note : video_title, file_name, and title will not appear in your feed. use
/// "description" to describe your video  which will appear at the top of the
/// post.
#[derive(Clone, Deserialize, Serialize)]
pub struct VideoParams {
    /// The video_title parameter will not be display on your post feed
    pub video_title: String,
    /// The description parameter is used to describe your video  which will
    /// appear at the top of the post
    pub description: String,

    /// Format: BMP, GIF, JPEG, PNG,TIFF
    /// File Size: 10MB or less.
    /// There are no image dimension requirements, but it should share the same
    /// aspect ratio as your video.
    pub thum: String,

    /// Enum for different categories that the uploaded video will belong to
    /// as defined on facebook  graph api documentation. Choose any from the
    /// list, if no data is supplied a default value of  "OTHER" is chosen.
    pub content_category: ContentCategory,

    pub title: String,

    pub file_path: String,
}

pub struct UploadFile {
    #[cfg(any(feature = "seed_async"))]
    pub file: File,
    pub file_path: String,
}

/// UsageType is an enum that is use to indication where  you are using the method,  Client or Server
#[derive(Deserialize, Debug, Serialize)]
pub enum UsageType {
    ///Client is when you are using it for at frontend or wasm
    Client,
    ///Server is when you are using this method at the backend
    Server,
}
/// Response for successful uploading of video using non_resumable method
#[derive(Deserialize, Debug, Serialize)]
pub struct PostResponse {
    id: String,
}

/// Enum for different categories that the uploaded video will belong to as
/// defined on facebook graph api documentation. Choose any from the list, if no
/// data is supplied a default value of "OTHER" is chosen.
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

/// This struct is the response gotten when initializing the resumable uploading
/// method process.
#[derive(Deserialize, Debug, Clone, Serialize)]
struct InitializeUploadResponse {
    pub video_id: String,
    pub end_offset: String,
    pub upload_session_id: String,
}

/// This struct is the response upon successful  upload of video using resumable
/// method. The struct is constructed  using different data gotten from
/// different responses while using the resumable  method. if the success
/// parameter in the struct is true then the video was uploaded successfully
/// Note: for video uploaded using the video_ur method, only the video_id
/// parameter will have a value other will be empty  since other parameter are
/// not used.
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct FinalResponeResumableUpload {
    // this struct will be data constructed from all the different uploads
    success: bool,
    upload_session_id: String, // will developer need this
    video_id: String,
}

// this will be used to update this method upon each round of chunked upload
impl FinalResponeResumableUpload {
    /// This method is used to update  the struct during a resumabl upload
    /// method since each chunk upload different data are sent back.
    fn update_params(
        mut self,
        video_id: String,
        upload_session_id: String,
    ) -> FinalResponeResumableUpload {
        self.upload_session_id = upload_session_id;
        self.video_id = video_id;
        self
    }

    /// This method is used to get the response data for the first request
    /// during a resumable upload
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

    /// This method is used to get the response data for the final request
    /// during a resumable upload the response will either be true or false.
    fn update_success(mut self, success: bool) -> FinalResponeResumableUpload {
        self.success = success;
        self
    }

    /// This method will return the struct of all the paramters
    pub fn response(self) -> FinalResponeResumableUpload {
        self
    }
}

/// After complete uploading of the video through resumable, facebook will send
/// reponse which will be either true or false. true means, the video was
/// uplaoded successfull
#[derive(Deserialize, Serialize)]
struct ResumableUploadFinal {
    // response from facebook, true or false
    success: bool,
}

/// During each chunk blob file uploaded, facebook will send a response back,
/// This struct is the response gotten for each video chunk  sent
#[derive(Deserialize, Clone, Serialize)]
struct ChunksUploadResponse {
    start_offset: String, // Value for second chunk
    end_offset: String,
}

impl ChunksUploadResponse {
    /// creating an instance of the chunking paramters
    fn new(start_offset: String, end_offset: String) -> ChunksUploadResponse {
        ChunksUploadResponse {
            start_offset,
            end_offset,
        }
    }

    fn end_offset(self) -> String {
        self.end_offset
    }

    fn start_offset(self) -> String {
        self.start_offset
    }
}

/// While using the resumable upload method, there are 4 enums which are used to
/// track the  progress/stage and status of the uploading processs which are
/// send along each upload. "start" is send when initialzing the upload process,
/// "transfer" is send when uploading is in progress, "finished is send when the
/// chunk files are finish uploading, and "cancel" is send when for you decide
/// to stop the uploading.
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
            title: " ".to_string(),
            file_path: "".to_string(),
        }
    }
}

impl VideoParams {
    pub fn new() -> VideoParams {
        VideoParams::default()
    }

    /// This method will update an existing data in your VideoPramas
    pub fn update_video_params(self, video_params: VideoParams) -> Self {
        Self { ..video_params }
    }

    /// This method will return the existing data in your VideoPramas
    pub fn video_params(self) -> VideoParams {
        self
    }
    pub fn set_file_path(&mut self, file_path: String) {
        self.file_path = file_path;
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

    /// This method is used by creating FormData for Non_resumable uplaod method
    #[cfg(any(feature = "seed_async"))]
    fn create_form_data(file: File, video_params: VideoParams) -> FormData {
        let mut form_data = FormData::new().unwrap();

        form_data.append_with_blob("source", &file); // appped  the  current chunked file   to the form

        if !video_params.video_title.is_empty() {
            form_data.append_with_str("video_title", &video_params.video_title);
        }

        if !video_params.description.is_empty() {
            form_data.append_with_str("description", &video_params.description);
        }

        if !video_params.thum.is_empty() {
            form_data.append_with_str("thum", &video_params.thum);
        };

        form_data
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
        #[cfg(any(feature = "seed_async"))] file: File,
        //  #[cfg(any(feature = "reqwest_async"))] file: String,
    ) -> Result<PostResponse, ClientErr> {
        let uploaded_file = file.clone();

        #[cfg(any(feature = "reqwest_async"))]
        let file_result = FileResultServer::file_analyze(video_params.file_path.clone());

        #[cfg(any(feature = "seed_async"))]
        let file_result = FileResult::file_analyze(file);

        // check if the uploading method
        if file_result.uploading_method() == "non_resumable" {
            let form_data = VideoApi::create_form_data(uploaded_file, video_params);

            let base_url = self.base_url.replace("EDGE", "videos");
            let url = base_url + "?access_token=" + &self.page_access_token;

            #[cfg(any(feature = "seed_async"))]
            let resp = HttpConnection::video_post::<PostResponse>(url, form_data).await?;

            #[cfg(any(feature = "reqwest_async"))]
            let resp =
                HttpConnection::video_post::<PostResponse>(url, video_params.clone()).await?;
            Ok(resp)
        } else {
            Err(ClientErr::FacebookError(
                "The uplaoded file is above 1 gb, use Resumable method ".to_string(),
            )) // try to generate a customer
               // error
        }
    }
}
#[cfg(any(feature = "seed_async"))]
impl VideoApi {
    /// This method is used by creating FormData for resumable uplaod method
    #[cfg(any(feature = "seed_async"))]
    fn resumable_form_data(
        self,
        upload_phase: UploadPhase,
        current_blob_file: Blob,
        uploaded_file: File,
        upload_session_id: String,
        start_offset: String,
        video_params: VideoParams,
    ) -> FormData {
        // phase is expected to be of an enum of either , start, transfer, and end
        // depending on the  uplaoding stage
        let mut current_upload_phase = "";
        let mut form_data = FormData::new().unwrap();

        match upload_phase {
            UploadPhase::start => {
                current_upload_phase = "start";
                form_data.append_with_str(
                    "file_size",
                    FileResult::file_analyze(uploaded_file)
                        .file_size_byte_string()
                        .as_str(),
                ); // add the video size
            }

            UploadPhase::transfer => {
                current_upload_phase = "transfer";
                form_data.append_with_str("start_offset", &start_offset);
                form_data.append_with_blob("video_file_chunk", &current_blob_file);
            }

            UploadPhase::finish => {
                current_upload_phase = "finish";

                form_data.append_with_str("upload_session_id", &upload_session_id);
                if !video_params.video_title.is_empty() {
                    form_data.append_with_str("video_title", &video_params.video_title);
                }

                if !video_params.description.is_empty() {
                    form_data.append_with_str("description", &video_params.description);
                }

                if !video_params.thum.is_empty() {
                    form_data.append_with_str("thum", &video_params.thum);
                };
            }
            // this method has not been implimented yet.
            UploadPhase::cancel => {
                form_data.append_with_str("upload_session_id", &upload_session_id);
                form_data.append_with_str("start_offset", &start_offset);
            }
        }

        form_data.append_with_str("upload_session_id", &upload_session_id);
        form_data.append_with_str("upload_phase", current_upload_phase);
        form_data.append_with_str("access_token", &self.page_access_token);

        form_data
    }

    /// This method is used for uploading large video files, it does that by
    /// chunking the file and uplaoding them individually until is complete.
    /// The method takes two parameter( file, video parameter struct).
    /// the waiting time depend  on the video size uplaoded
    ///
    /// Note there is an issue with chunking method that only chunk smaller size
    /// so extra time than usuall is expect until the issue is fixed.
    ///  
    /// for more infromation  check  https://developers.facebook.com/docs/video-api/guides/publishing
    pub async fn resumable_post(
        &self,
        file: File,
        video_param: VideoParams,
    ) -> Result<FinalResponeResumableUpload, ClientErr> {
        let uploaded_file = file.clone();
        let mut start_offset = Some("0".to_string()); // this  data will be updated  frpm the respones
        let mut end_offset = Some("0".to_string()); // this  data will be updated  frpm the respones
        let video_params = video_param.clone();

        let self_data = self.clone();
        let base_url = self.base_url.replace("EDGE", "videos").clone();
        let mut form_data = self_data.resumable_form_data(
            UploadPhase::start,
            Blob::new().unwrap(),
            uploaded_file.clone(),
            "".to_string(),
            "0".to_string(),
            video_param.clone(),
        );
        let url = base_url + "?access_token=" + &self.page_access_token;

        let response =
            HttpConnection::post::<InitializeUploadResponse, String>(url, "".to_string()).await?;

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

                // check if there is a response data from the initialzition request
                if let Some(start_offset_status) = start_offset.clone() {
                    //   start_offset_status = tee.clone();
                    let self_data = self.clone();

                    if end_offset_status != start_offset_status {
                        let base_url = self.base_url.replace("EDGE", "videos");
                        let form_datas = self_data.resumable_form_data(
                            UploadPhase::transfer,
                            FileResult::file_analyze(file.clone())
                                .chunk_file(start_chunk, current_chunk_size)
                                .chunked_file(),
                            file.clone(),
                            upload_session_id.to_string(),
                            start_offset_status,
                            video_params.clone(),
                        );

                        let response = HttpConnection::video_post::<ChunksUploadResponse>(
                            base_url,
                            form_datas.clone(),
                        )
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
                        let self_data = self.clone();
                        let base_url = self.base_url.replace("EDGE", "videos");

                        // There is an issue with the formdata for blob chunked file, it can only
                        // take small sized of blob file, this making the
                        // upload to take longer than expect. if larger blob
                        // file is appeded to the formData, the data will not be posted along the
                        // request causing an erorr.
                        // allthough the formData can take large file that are not chunked, thefore
                        // it is not certain   where the issue is coming
                        //

                        let form_data = self_data.resumable_form_data(
                            UploadPhase::finish,
                            Blob::new().unwrap(), // not important in the uplaod phase
                            uploaded_file.clone(), // this file is no longer important
                            upload_session_id.to_string(),
                            "".to_string(),
                            video_param.clone(),
                        );

                        let resp =
                            HttpConnection::video_post::<ResumableUploadFinal>(base_url, form_data)
                                .await?;
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

#[derive(Deserialize, Debug, Default, Serialize)]
struct FeedPostSuccess {
    id: String,
}

impl VideoApi {
    /// this Method is used for posting video hosted online (video url ) to the
    ///  page feed.
    pub async fn post_by_link(
        &self,
        file_url: &str,
    ) -> Result<FinalResponeResumableUpload, ClientErr> {
        let base_url = self.base_url.replace("EDGE", "videos");

        let url = base_url + "?file_url=" + file_url + "&access_token=" + &self.page_access_token;
        let video_id = HttpConnection::post::<FeedPostSuccess, String>(url, "".to_string()).await?;
        if video_id.id.is_empty() {
            Err(ClientErr::FacebookError(
                "The video posting by url was not suceessfull ".to_string(),
            )) // try to generate a customer
        } else {
            Ok(FinalResponeResumableUpload::default().update_params(video_id.id, "".to_string()))
        }
    }
}
