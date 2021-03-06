#![feature(decl_macro)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;

use lastfm::artist::GetCorrections;
test_fn!(test_artist_getcorrection, GetCorrections, ["guns roses"]);

use lastfm::artist::GetInfo;
test_fn!(
    test_artist_getinfo,
    GetInfo,
    ["iamthemorning", None, Some(1), None, Some("xenzh")]
);

use lastfm::artist::GetSimilar;
test_fn!(test_artist_getsimilar, GetSimilar, ["nadja", None, Some(1), Some(3)]);

use lastfm::structs::artist::GetTags;
test_fn!(
    test_artist_gettags,
    GetTags,
    ["adam lane's full throttle orchestra", None, None, Some("xenzh")]
);

use lastfm::artist::GetTopAlbums;
test_fn!(
    test_artist_gettopalbums,
    GetTopAlbums,
    ["days n' daze", None, Some(1), Some(4), None]
);

use lastfm::artist::GetTopTags;
test_fn!(test_artist_gettoptags, GetTopTags, ["schtimm", None, Some(1)]);

use lastfm::artist::GetTopTracks;
test_fn!(
    test_artist_gettoptracks,
    GetTopTracks,
    ["charles mingus", None, Some(1), Some(4), None]
);

use lastfm::artist::Search;
test_fn!(test_artist_search, Search, ["days", Some(3), Some(2)]);
