use super::api_function::ObjectAPIFunction;
use super::object::ObjectAPI;
use super::object::ObjectManager;
use serde_derive::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

#[derive(Deserialize, Serialize)]
struct Location {
    location: String,
}

#[derive(Deserialize, Serialize)]
struct Policy {
    policy: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Ok {
    aa: String,
}

#[derive(Deserialize, Serialize)]
struct Notification {
    notification: String,
}

pub(crate) fn register_s3_route(
    s: ObjectManager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let root = warp::path::end()
        .and(with_storage(s.clone()))
        .and_then(ObjectAPIFunction::get_bucket_list);

    let bucket = warp::path::param::<String>();
    let object = warp::path::param::<String>(); // 路径参数: object

    // bucket 操作
    let bucket_put = bucket
        .and(warp::put())
        .and(with_storage(s.clone()))
        .and_then(ObjectAPIFunction::put_bucket);

    let bucket_delete = bucket
        .and(warp::delete())
        .and(with_storage(s.clone()))
        .and_then(ObjectAPIFunction::delete_bucket);

    // get bucket location
    let bucket_location = bucket
        .and(warp::get())
        .and(warp::query::<Location>())
        .map(|bucket, q| "location");

    // get bucket policy
    let bucket_policy = bucket
        .and(warp::get())
        .and(warp::query::<Policy>())
        .map(|bucket, q| "policy");

    // get bucket notification
    let bucket_notification = bucket
        .and(warp::get())
        .and(warp::query::<Notification>())
        .map(|bucket, q| "notification");

    // object 操作
    let bucket_put_object = bucket
        .and(object)
        .and(warp::put())
        .and(with_storage(s.clone()))
        .and_then(ObjectAPIFunction::put_object);

    root.or(bucket_put_object)
        .or(bucket_put)
        .or(bucket_delete)
        .or(bucket_location)
        .or(bucket_policy)
        .or(bucket_notification)
}

fn with_storage(
    s: ObjectManager,
) -> impl Filter<Extract = (ObjectManager,), Error = Infallible> + Clone {
    warp::any().map(move || s.clone())
}

pub(crate) fn register_rpc_server() {}
