//! Endpoints regarding games

#[doc(inline)]
pub use get_games::GetGamesRequest;

#[doc(inline)]
pub use get_top_games::GetTopGamesRequest;

#[doc(inline)]
pub use types::TwitchCategory as Game;

use crate::{helix, types};
use serde::{Deserialize, Serialize};

/// Gets game information by game ID or name.
/// [`get-games`](https://dev.twitch.tv/docs/api/reference#get-games)
///
/// # Accessing the endpoint
///
/// ## Request: [GetGamesRequest]
///
/// To use this endpoint, construct a [`GetGamesRequest`] with the [`GetGamesRequest::builder()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::games::get_games;
/// let request = get_games::GetGamesRequest::builder()
///     .id(vec!["4321".to_string()])
///     .build();
/// ```
///
/// ## Response: [Game](types::TwitchCategory)
///
/// Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, games::get_games};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
/// let request = get_games::GetGamesRequest::builder()
///     .id(vec!["4321".to_string()])
///     .build();
/// let response: Vec<get_games::Game> = client.req_get(request, &token).await?.data;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
pub mod get_games {
    use super::*;

    /// Query Parameters for [Get Games](super::get_games)
    ///
    /// [`get-games`](https://dev.twitch.tv/docs/api/reference#get-games)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetGamesRequest {
        /// Game ID. At most 100 id values can be specified.
        #[builder(default)]
        pub id: Vec<types::CategoryId>,
        /// Game name. The name must be an exact match. For instance, “Pokemon” will not return a list of Pokemon games; instead, query the specific Pokemon game(s) in which you are interested. At most 100 name values can be specified.
        #[builder(default)]
        pub name: Vec<String>,
    }

    /// Return Values for [Get Games](super::get_games)
    ///
    /// [`get-games`](https://dev.twitch.tv/docs/api/reference#get-games)
    pub type Game = types::TwitchCategory;

    impl helix::Request for GetGamesRequest {
        type Response = Vec<Game>;

        const PATH: &'static str = "games";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetGamesRequest {}

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetGamesRequest::builder()
            .id(vec!["493057".to_string()])
            .build();

        // From twitch docs
        let data = br#"
{
    "data": [
        {
            "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/Fortnite-52x72.jpg",
            "id": "33214",
            "name": "Fortnite"
        },
        {
            "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/Fortnite-52x72.jpg",
            "id": "33214",
            "name": "Fortnite"
        }
    ],
    "pagination": {
    "cursor": "eyJiIjpudWxsLCJhIjp7IkN"
  }
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/games?id=493057"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Gets games sorted by number of current viewers on Twitch, most popular first.
/// [`get-top-games`](https://dev.twitch.tv/docs/api/reference#get-top-games)
///
/// # Accessing the endpoint
///
/// ## Request: [GetTopGamesRequest]
///
/// To use this endpoint, construct a [`GetTopGamesRequest`] with the [`GetTopGamesRequest::builder()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::games::get_top_games;
/// let request = get_top_games::GetTopGamesRequest::builder()
///     .first(100)
///     .build();
/// ```
///
/// ## Response: [Game](types::TwitchCategory)
///
/// Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, games::get_top_games};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
/// let request = get_top_games::GetTopGamesRequest::builder()
///     .build();
/// let response: Vec<get_top_games::Game> = client.req_get(request, &token).await?.data;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
pub mod get_top_games {
    use super::*;

    /// Query Parameters for [Get Games](super::get_games)
    ///
    /// [`get-top-games`](https://dev.twitch.tv/docs/api/reference#get-top-games)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetTopGamesRequest {
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        pub after: Option<helix::Cursor>,
        /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        pub before: Option<helix::Cursor>,
        /// Maximum number of objects to return. Maximum: 100. Default: 20.
        #[builder(default)]
        #[builder(setter(strip_option))]
        pub first: Option<usize>,
    }

    /// Return Values for [Get Games](super::get_games)
    ///
    /// [`get-top-games`](https://dev.twitch.tv/docs/api/reference#get-top-games)
    pub type Game = types::TwitchCategory;

    impl helix::Request for GetTopGamesRequest {
        type Response = Vec<Game>;

        const PATH: &'static str = "games/top";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetTopGamesRequest {}

    impl helix::Paginated for GetTopGamesRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetTopGamesRequest::builder().build();

        // From twitch docs
        let data = br#"
{
    "data": [
      {
        "id": "493057",
        "name": "PLAYERUNKNOWN'S BATTLEGROUNDS",
        "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/PLAYERUNKNOWN%27S%20BATTLEGROUNDS-{width}x{height}.jpg"
      },
      {
        "id": "493057",
        "name": "PLAYERUNKNOWN'S BATTLEGROUNDS",
        "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/PLAYERUNKNOWN%27S%20BATTLEGROUNDS-{width}x{height}.jpg"
      }
    ],
    "pagination":{"cursor":"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6MjB9fQ=="}
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/games/top?");

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}
