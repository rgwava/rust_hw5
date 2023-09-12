namespace rs miniredis

struct GetItemRequest {
    1: required i32 tyep,
    2: required string key,
    3: required string value,
}

struct GetItemResponse {
    1: required i32 tyep,
    2: required string key,
    3: required string value,
    4: required bool success,
}

service ItemService {
    GetItemResponse GetItem (1: GetItemRequest req),
}

