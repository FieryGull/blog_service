#[cfg(test)]
mod integration_tests {
    use std::env;
    use crate::posts::*;
    use crate::common_lib::*;
    use actix_web::{test::{self, TestRequest}, App, http::header::AUTHORIZATION};
    use uuid::Uuid;
    use dotenv::dotenv;

    #[actix_rt::test]
    async fn test_endpoints() {
        dotenv().ok();
        let user_id = Uuid::new_v4().to_string();
        let title = "test_title".to_string();
        let body = "test_body".to_string();

        let new_post = NewPost{
            title,
            body,
        };

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not found.");
        let token = Token::create( user_id, jwt_secret).unwrap();

        let mut app = test::init_service(App::new().configure(init_routes)).await;


        // Create post
        let resp = TestRequest::post().uri(&format!("/posts")).set_json(&new_post).insert_header((AUTHORIZATION, token.clone())).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to create post");
        let created_post: Posts = test::read_body_json(resp).await;


        // Find created post by id
        let resp = TestRequest::get().uri(&format!("/posts/{}", created_post.id)).insert_header((AUTHORIZATION, token.clone())).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to find post");

        let find_post: Posts = test::read_body_json(resp).await;
        assert_eq!(find_post.id, created_post.id,  "Find and created post don't match");


        // Find created post in all posts list
        let resp = TestRequest::get().uri(&format!("/posts")).insert_header((AUTHORIZATION, token.clone())).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to find posts");

        let posts: Vec<Posts> = test::read_body_json(resp).await;
        assert!(posts.contains(&find_post), "The found posts do not contain a new post");


        // Update created post
        let post_for_update = NewPost{
            title: "change title".to_string(),
            body: "change body".to_string(),
        };

        let resp = TestRequest::put().uri(&format!("/posts/{}", created_post.id)).set_json(&post_for_update).insert_header((AUTHORIZATION, token.clone())).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to update posts");

        let updated_post: Posts = test::read_body_json(resp).await;
        assert_eq!(updated_post.id, created_post.id,  "Update wrong post");
        assert_eq!(updated_post.title, post_for_update.title,  "Failed to update title");
        assert_eq!(updated_post.body, post_for_update.body,  "Failed to update body");


        // Delete created post
        let resp = TestRequest::delete().uri(&format!("/posts/{}", created_post.id)).insert_header((AUTHORIZATION, token.clone())).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to delete post");

        let resp = TestRequest::get().uri(&format!("/posts/{}", created_post.id)).insert_header((AUTHORIZATION, token.clone())).send_request(&mut app).await;
        assert_eq!(resp.status(), 404, "Post exists after deletion");
    }
}