use rocket::data::{FromDataSimple, Outcome};
use rocket::http::ContentType;
use rocket::{Data, Request};
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, Repetition,
};
use std::path::PathBuf;

pub struct UploadedFile {
    pub content_type: Option<mime::Mime>,
    pub file_name: Option<String>,
    pub path: PathBuf,
}

pub struct CreatePostMultipart {
    pub name: String,
    pub message: String,
    pub files: Vec<UploadedFile>,
}

impl FromDataSimple for CreatePostMultipart {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> Outcome<Self, String> {
        let req_ct = req.content_type().unwrap();
        let ct = ContentType::new("multipart", "form-data");
        if req_ct == &ct {
            let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
                MultipartFormDataField::text("name").size_limit(255),
                MultipartFormDataField::text("message").size_limit(8000),
                MultipartFormDataField::file("file")
                    .repetition(Repetition::fixed(5))
                    .size_limit(100 * 1024 * 1024), // 100 MB.
            ]);

            let mut form_data = MultipartFormData::parse(req_ct, data, options).unwrap();

            let name = match form_data.texts.remove("name") {
                Some(mut name_fields) => {
                    let name_field = name_fields.remove(0);
                    name_field.text
                }
                None => String::from(""),
            };

            let message = match form_data.texts.remove("message") {
                Some(mut message_fields) => {
                    let message_field = message_fields.remove(0);
                    message_field.text
                }
                None => String::from(""),
            };

            let files = match form_data.files.remove("file") {
                Some(file_fields) => file_fields
                    .into_iter()
                    .map(|field| UploadedFile {
                        content_type: field.content_type,
                        file_name: field.file_name,
                        path: field.path,
                    })
                    .collect(),
                None => Vec::new(),
            };

            Outcome::Success(CreatePostMultipart {
                name,
                message,
                files,
            })
        } else {
            Outcome::Forward(data)
        }
    }
}
