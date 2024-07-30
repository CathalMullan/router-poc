#[rustfmt::skip]
pub const ROUTES: [&str; 29] = [
    "/",

    "/api/{version}/data/{data_path:*}",
    "/api/{version}/items/{id}",
    "/api/{version}/search/{query:*}",
    "/api/{version}/users/{user_id}/posts/{post_path:*}",
    "/api/v1/long/path/with/many/segments/{id}",

    "/articles/{category}/{article_path:*}",
    "/blog/{year}-{month}-{day}/{slug}",
    "/courses/{course_id}/{lesson_path:*}",
    "/events/{year}-{month}-{day}/{event_details:*}",

    "/files/{file_path:*}/preview",
    "/files/{path:*}",

    "/products/{category}/{subcategories:*}",
    "/products/{sku}/{color}/{size}",

    "/projects/{project_id}/tasks/{task_id}/{subtasks:*}",
    "/reports/{year}-{month}-{day}/{report_type}/{details:*}",
    "/search/{query}/{page}/{limit}/{sort}",
    "/services/{service_type}/{params:*}/execute",
    "/static/{file}/{ext}",
    "/store/{store_id}/{department}/{product_path:*}",

    "/users/{id}",
    "/users/{user_id}/albums/{album_id}/{photo_path:*}",
    "/users/{user_id}/profile",

    "/{category}/{subcategory}",
    "/{x}/{y}/{z}",
    "/{a}/{b}/{c}/{d}/{e}/{f}/{g}/{h}/{i}/{j}",

    "/{path:*}",
    "/{path:*}/suffix",
    "/prefix/{path:*}",
];

#[rustfmt::skip]
pub const ROUTES_REGEX: [&str; 29] = [
    r"^/$",

    r"^/api/(?P<version>[^/]+)/data/(?P<data_path>.+)$",
    r"^/api/(?P<version>[^/]+)/items/(?P<id>[^/]+)$",
    r"^/api/(?P<version>[^/]+)/search/(?P<query>.+)$",
    r"^/api/(?P<version>[^/]+)/users/(?P<user_id>[^/]+)/posts/(?P<post_path>.+)$",
    r"^/api/v1/long/path/with/many/segments/(?P<id>[^/]+)$",

    r"^/articles/(?P<category>[^/]+)/(?P<article_path>.+)$",
    r"^/blog/(?P<year>[^/]+)-(?P<month>[^/]+)-(?P<day>[^/]+)/(?P<slug>[^/]+)$",
    r"^/courses/(?P<course_id>[^/]+)/(?P<lesson_path>.+)$",
    r"^/events/(?P<year>[^/]+)-(?P<month>[^/]+)-(?P<day>[^/]+)/(?P<event_details>.+)$",

    r"^/files/(?P<file_path>.+)/preview$",
    r"^/files/(?P<path>.+)$",

    r"^/products/(?P<category>[^/]+)/(?P<subcategories>.+)$",
    r"^/products/(?P<sku>[^/]+)/(?P<color>[^/]+)/(?P<size>[^/]+)$",

    r"^/projects/(?P<project_id>[^/]+)/tasks/(?P<task_id>[^/]+)/(?P<subtasks>.+)$",
    r"^/reports/(?P<year>[^/]+)-(?P<month>[^/]+)-(?P<day>[^/]+)/(?P<report_type>[^/]+)/(?P<details>.+)$",
    r"^/search/(?P<query>[^/]+)/(?P<page>[^/]+)/(?P<limit>[^/]+)/(?P<sort>[^/]+)$",
    r"^/services/(?P<service_type>[^/]+)/(?P<params>.+)/execute$",
    r"^/static/(?P<file>[^/]+)/(?P<ext>[^/]+)$",
    r"^/store/(?P<store_id>[^/]+)/(?P<department>[^/]+)/(?P<product_path>.+)$",

    r"^/users/(?P<id>[^/]+)$",
    r"^/users/(?P<user_id>[^/]+)/albums/(?P<album_id>[^/]+)/(?P<photo_path>.+)$",
    r"^/users/(?P<user_id>[^/]+)/profile$",

    r"^/(?P<category>[^/]+)/(?P<subcategory>[^/]+)$",
    r"^/(?P<x>[^/]+)/(?P<y>[^/]+)/(?P<z>[^/]+)$",
    r"^/(?P<a>[^/]+)/(?P<b>[^/]+)/(?P<c>[^/]+)/(?P<d>[^/]+)/(?P<e>[^/]+)/(?P<f>[^/]+)/(?P<g>[^/]+)/(?P<h>[^/]+)/(?P<i>[^/]+)/(?P<j>[^/]+)$",

    r"^/(?P<path>.+)$",
    r"^/(?P<path>.+)/suffix$",
    r"^/prefix/(?P<path>.+)$",
];

#[rustfmt::skip]
pub const SEARCHES: [&str; 14] = [
    "/",

    "/api/v1/items/123",
    "/api/v2/search/best-restaurants-in-new-york",
    "/api/v1/users/123/posts/2023/05/my-blog-post",

    "/articles/technology/ai/machine-learning/comments",
    "/blog/2023-07-15/summer-vacation-ideas",
    "/events/2023-08-20/annual-tech-conference",

    "/files/documents/project/report.pdf/preview",

    "/products/electronics/computers/laptops/item/456",
    "/projects/789/tasks/101/subtask1/subtask2",
    "/services/data-analysis/format/csv/column/5/execute",
    "/static/image/png",

    "/users/42/profile",
    "/some/very/long/path/that/should/match/catch/all",
];
