use super::object::ObjectManager;
use serde_derive::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Rejection;
use warp::hyper::Body;
use warp::{Filter, http::Response};
use warp::{Reply, http::StatusCode, reject, reply::json};
#[derive(Debug, Deserialize, Serialize)]
struct Success {
    s: String,
}

pub(crate) struct ObjectAPIFunction;

impl ObjectAPIFunction {
    pub(crate) async fn get_bucket_list(s: ObjectManager) -> Result<impl Reply, Rejection> {
        Ok(warp::reply::json(&Success {
            s: "get_bucket_list".to_string(),
        }))
    }

    pub(crate) async fn put_bucket(
        bucket: String,
        s: ObjectManager,
    ) -> Result<impl Reply, Rejection> {
        println!("{}", bucket);
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty()))
    }

    pub(crate) async fn put_object(
        bucket: String,
        object: String,
        s: ObjectManager,
    ) -> Result<impl Reply, Rejection> {
        //s.make_bucket(&bucket);
        //  warp::reply::with_header(reply, name, value)
        println!("{},{}", bucket, object);
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty()))
        //Ok(StatusCode::OK)
    }

    pub(crate) async fn delete_bucket(
        bucket: String,
        s: ObjectManager,
    ) -> Result<impl Reply, Rejection> {
        Ok(warp::reply::json(&Success {
            s: "delete_bucket".to_string(),
        }))
    }

    pub fn write_success_response() {}
}
