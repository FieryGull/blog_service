#[cfg(test)]
mod integration_tests {
    use crate::users::*;
    use actix_web::{test::{self, TestRequest}, App, http::header::AUTHORIZATION};
    use serde::{Deserialize, Serialize};
    use dotenv::dotenv;
    use random_string::generate;

    #[actix_rt::test]
    async fn test_endpoints() {
        dotenv().ok();
        let charset = "abcdefghijklmnopqrstuvwxyz";

        #[derive(Serialize, Deserialize)]
        struct Token {
            token: String
        }

        let name = "test_name".to_string();
        let email = format!("{}@mail.com", generate(6, charset)).to_string();
        let password = "test".to_string();

        let register_schema = RegisterUserSchema{
            name: name.clone(),
            email: email.clone(),
            password: password.clone(),
        };

        let login_schema = LoginUserSchema{
            email: email.clone(),
            password: password.clone(),
        };

        let mut app = test::init_service(App::new().configure(init_routes)).await;


        // Register user
        let resp = TestRequest::post().uri("/auth/register").set_json(&register_schema).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to register with register_schema");
        let user: FilteredUser = test::read_body_json(resp).await;

        let resp = TestRequest::post().uri("/auth/register").set_json(&register_schema).send_request(&mut app).await;
        assert!(resp.status().is_client_error(), "Should not be possible to create user with same email twice");


        // Login by credentials
        let resp = TestRequest::post().uri("/auth/login").set_json(&login_schema).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to login with login_schema");
        let token: Token = test::read_body_json(resp).await;


        // Check user by id
        let resp = TestRequest::get().uri(&format!("/users/{}", user.id)).insert_header((AUTHORIZATION, token.token.clone())).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to find user");

        let user: FilteredUser = test::read_body_json(resp).await;
        assert_eq!(register_schema.email, user.email, "Find wrong user");


        // Check user in all users list
        let resp = TestRequest::get().uri(&format!("/users")).insert_header((AUTHORIZATION, token.token.clone())).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to find users");

        let users: Vec<FilteredUser> = test::read_body_json(resp).await;
        assert!(users.contains(&user), "Couldn't find a register user");
    }
}