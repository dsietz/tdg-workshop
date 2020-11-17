use super::*;
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web::http::{StatusCode};
use test_data_generation::data_sample_parser::DataSampleParser;
use test_data_generation::{Profile, shared};

static WORKSPACE_LOCAL_STORAGE: &str = "../profiles";

pub fn get_service_root() -> String {
    format!("/tdg/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/"
}

pub fn get_service_path_topic() -> String {
    get_service_root() + "/{topic}"
}

#[get("/tdg/v1/{topic}")] 
pub fn profile(web::Path(topic): web::Path<String>) -> HttpResponse {
    let profile_file = shared::string_to_static_str(format!("{}/{}", WORKSPACE_LOCAL_STORAGE, topic));
    let mut profile = Profile::from_file(&profile_file);

    HttpResponse::build(StatusCode::OK)
    .body(profile.generate().to_string())
}


pub fn index(_req: HttpRequest) -> HttpResponse {
    let dsp_file = &format!("{}/{}", WORKSPACE_LOCAL_STORAGE, "sample-01-dsp");
    let mut dsp = DataSampleParser::from_file(&dsp_file);

    HttpResponse::build(StatusCode::OK)
    .body(dsp.generate_record()[0].to_string())
}

#[cfg(test)]
mod tests {
   use super::*;
   #[allow(unused_imports)]
   use actix_web::{test};

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/tdg/{}", VER));
    }
    
    #[test]
    fn test_get_service_path() {
        assert_eq!(get_service_path(), format!("/tdg/{}/", VER));
    }
    
    #[test]
    fn test_get_topic_service_path() {
        assert_eq!(get_service_path_topic(), format!("/tdg/{}/{{topic}}", VER));
    }
    
    #[test]
    fn ok_response() {
        let req = test::TestRequest::with_header("content-type", "text/plain")
        .to_http_request();

        let resp = index(req);
        assert_eq!(resp.status(), StatusCode::OK);
    }
}