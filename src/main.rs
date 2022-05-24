use am::{
    constant::{AM_DEV_TOKEN, AM_USER_TOKEN},
    domain::AMPlaylistTrack,
    AppleMusicAPI,
};
use sp::{constant::SP_USER_TOKEN, SearchTrackArgument, SearchTrackInfo, SpotifyAPI};

use crate::sp::model::SearchTrackItem;

mod am;
mod sp;

fn main() {
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
        .collect::<Vec<Option<SearchTrackItem>>>();

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
