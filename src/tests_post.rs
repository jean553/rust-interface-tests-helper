extern crate mockito;
extern crate reqwest;

#[cfg(test)]
mod tests {

    use ClientHandler;
    use ResponseHandler;

    use reqwest::{
        Client,
        Response,
    };

    use tests_post::mockito::mock;

    use std::collections::HashMap;

    trait HasBaseUrl {

        fn get_base_url(&self) -> &str;
    }

    impl HasBaseUrl for Client {

        /// Returns the service base URL.
        ///
        /// # Returns:
        ///
        /// the service base URL.
        fn get_base_url(&self) -> &str {
            "http://localhost:1234"
        }
    }

    trait ResourceHandler {

        fn post_json_resource(&self, json: &HashMap<&str, &str>) -> Response;

        fn post_body_resource(&self, body: &str) -> Response;
    }

    impl ResourceHandler for Client {

        /// Example of "per resource implementation" method to post JSON.
        ///
        /// # Arguments:
        ///
        /// `json` - the json data to send
        fn post_json_resource(
            &self,
            json: &HashMap<&str, &str>,
        ) -> Response {

            self.post_json(
                &format!("{}/resource", self.get_base_url()),
                json,
            )
        }

        /// Example of "per resource implementation" method to post raw body.
        ///
        /// # Arguments:
        ///
        /// `body` - the raw body to send
        fn post_body_resource(
            &self,
            body: &str,
        ) -> Response {

            self.post_body(
                &format!("{}/resource", self.get_base_url()),
                body,
            )
        }
    }

    #[test]
    fn test_post_json_returns_201() {

        const API: &str = "/resource";
        let _m = mock("POST", API)
            .with_status(201)
            .with_body("OK")
            .create();

        let mut json: HashMap<&str, &str> = HashMap::new();
        json.insert("key", "value");

        let client = Client::new();
        let response = client.post_json_resource(&json);

        response.assert_201();
    }

    #[test]
    fn test_post_json_returns_400() {

        const API: &str = "/resource";
        let _m = mock("POST", API)
            .with_status(400)
            .create();

        let json: HashMap<&str, &str> = HashMap::new();

        let client = Client::new();
        let response = client.post_json_resource(&json);

        response.assert_400();
    }

    #[test]
    fn test_post_json_returns_409() {

        const API: &str = "/resource";
        let _m = mock("POST", API)
            .with_status(409)
            .create();

        let mut json: HashMap<&str, &str> = HashMap::new();
        json.insert("key", "value");

        let client = Client::new();
        let response = client.post_json_resource(&json);

        response.assert_409();
    }

    #[test]
    fn test_post_body_returns_201() {

        const API: &str = "/resource";
        let _m = mock("POST", API)
            .with_status(201)
            .with_body("OK")
            .create();

        let client = Client::new();
        let response = client.post_body_resource("raw body");

        response.assert_201();
    }
}
