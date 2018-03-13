extern crate mockito;
extern crate reqwest;

#[cfg(test)]
mod tests {

    use lib::{
        ClientHandler,
        ResponseHandler,
        HasBaseUrl,
    };

    use reqwest::{
        Client,
        Response,
    };

    use tests_post::mockito::mock;

    use std::collections::HashMap;

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

        fn post_resource(&self, json: &HashMap<&str, &str>) -> Response;
    }

    impl ResourceHandler for Client {

        /// Example of "per resource implementation" method.
        ///
        /// # Arguments:
        ///
        /// `json` - the json data to send
        fn post_resource(
            &self,
            json: &HashMap<&str, &str>,
        ) -> Response {

            self.post_json(
                &format!("{}/resource", self.get_base_url()),
                json,
            )
        }
    }

    #[test]
    fn test_post() {

        const API: &str = "/resource";
        let _m = mock("POST", API)
            .with_status(201)
            .with_body("OK")
            .create();

        let mut json: HashMap<&str, &str> = HashMap::new();
        json.insert("key", "value");

        let client = Client::new();
        let response = client.post_resource(&json);

        response.assert_201();
    }
}
