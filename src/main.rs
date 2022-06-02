use am::{
    common::AppleApiObject,
    constant::{AM_DEV_TOKEN, AM_USER_TOKEN},
    domain::AMPlaylistTrack,
    model::SongAttribute,
    AppleMusicAPI,
};
use sp::{constant::SP_USER_TOKEN, SearchTrackArgument, SearchTrackInfo, SpotifyAPI};

use crate::sp::model::Track;

mod am;
mod sp;

fn from_am_to_sp() {
    let am_api = AppleMusicAPI::new(AM_USER_TOKEN, AM_DEV_TOKEN);
    let sp_api = SpotifyAPI::new(SP_USER_TOKEN);

    let user_profile = sp_api.get_user_profile().unwrap();

    let playlists = am_api.get_all_playlists().unwrap();
    let playlist_of_interest = &playlists.data[4];

    let playlist_tracks = am_api
        .get_user_playlist_tracks(playlist_of_interest, vec![], None)
        .map(|track_collections| {
            track_collections
                .iter()
                .flat_map(|collection| {
                    collection
                        .data
                        .iter()
                        .map(|apple_track| AMPlaylistTrack::to_playlist_track(&apple_track))
                        .collect::<Vec<AMPlaylistTrack>>()
                })
                .collect::<Vec<AMPlaylistTrack>>()
        })
        .unwrap();

    println!("Playlist Length: {}", playlist_tracks.len());

    let spotify_tracks = playlist_tracks
        .iter()
        .map(|t| {
            let track_info = SearchTrackInfo {
                name: t.name.clone(),
                album_name: t
                    .album_name
                    .to_owned()
                    .or_else(|| Some(String::new()))
                    .unwrap(),
                artist_name: t.artist_name.clone(),
            };
            let arg = SearchTrackArgument {
                isrc: t.isrc.clone(),
                track_info,
            };

            println!(
                "====> Looking for {} - {} in Spotify catalog",
                &t.name, &t.artist_name
            );

            match sp_api.search_track(arg) {
                Ok(r) => match r {
                    Ok(result) => {
                        let matches = result.tracks.items.len();
                        if matches > 0 {
                            println!("Number of match: {}", &result.tracks.items.len());
                            Some(result.tracks.items[0].clone())
                        } else {
                            println!("No matches");
                            None
                        }
                    }
                    Err(err) => {
                        println!("Error!\n{:?}", err);
                        None
                    }
                },
                Err(err) => {
                    println!("Error!\n{:?}", err);
                    None
                }
            }
        })
        .collect::<Vec<Option<Track>>>();

    let spotify_tracks_uri = spotify_tracks
        .iter()
        .map(|t| match t {
            Some(track) => track.uri.clone(),
            None => String::new(),
        })
        .filter(|uri| uri.len() > 0)
        .collect::<Vec<String>>();

    let add_tracks_reponses = sp_api
        .create_new_playlist(&user_profile.id, &playlist_of_interest.attributes.name)
        .and_then(|new_playlist| {
            sp_api.add_tracks_to_playlist(&new_playlist.id, &spotify_tracks_uri)
        });

    match add_tracks_reponses {
        Ok(responses) => {
            for response in responses {
                println!("Add tracks response status: {}", response.status())
            }
        }
        Err(err) => println!("Add track error: {:?}", err),
    }
}

fn from_sp_to_am() {
    let sp_api = SpotifyAPI::new(SP_USER_TOKEN);
    let am_api = AppleMusicAPI::new(AM_USER_TOKEN, AM_DEV_TOKEN);

    let storefront = am_api.get_user_storefront().unwrap();
    let user_profile = sp_api.get_user_profile().unwrap();
    let user_playlists = sp_api.get_user_playlists(&user_profile.id).unwrap();
    let playlist_of_interest = &user_playlists.items[1];

    let am_songs = sp_api
        .get_user_playlist_tracks(&playlist_of_interest.id, vec![], None)
        .map(|tracks_collection| {
            tracks_collection
                .iter()
                .flat_map(|tracks| {
                    tracks
                        .items
                        .iter()
                        .map(|item| item.track.clone())
                        .collect::<Vec<sp::model::Track>>()
                })
                .filter_map(|t| {
                    println!("====> Looking for {} in Apple Music catalog", &t.name);
                    match am_api.search_track(&storefront.data[0].id, &t.external_ids.isrc) {
                        Ok(result) => {
                            if result.data.len() == 0 {
                                println!("Cannot find a match for this song");
                                None
                            } else {
                                println!("Match found: {}", result.data.len());
                                Some(result.data[0].clone())
                            }
                        }
                        Err(e) => {
                            println!("Error when looking for this track {:?}", e);
                            None
                        }
                    }
                })
                .collect::<Vec<AppleApiObject<SongAttribute>>>()
        });

    match am_songs {
        Ok(x) => {
            println!("Found {} songs in Apple Catalog!", x.len());
            let creation_response = am_api
                .create_new_playlist(
                    &playlist_of_interest.name,
                    x.iter().map(|song| song.id.clone()).collect(),
                )
                .expect("Error");

            for r in creation_response {
                println!("{:?}", &r.status());
            }
        }
        _ => (),
    }
}

fn main() {
    // from_am_to_sp()
    from_sp_to_am()
}
