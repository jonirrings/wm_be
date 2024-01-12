use axum::Json;
use crate::models::room::{RoomId,Room};
use crate::web::api::v1::responses::OkResponseData;


pub fn mutated_room(room_id:RoomId)->Json<OkResponseData<RoomId>>{
    Json(OkResponseData{data:room_id})
}
pub fn get_room(room:Room)->Json<OkResponseData<Room>>{
    Json(OkResponseData{data:room})
}
