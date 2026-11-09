//! Unit tests for the pieces that live at the top of the crate (link parsing, backups).

mod links {
    use crate::*;

    /// Video and playlist addresses are detected; ordinary searches are not.
    #[test]
    fn pasted_links_are_recognised() {
        let pl = |q: &str| matches!(classify_link(q), Link::Playlist(ref id) if id == "PL2aBZuCeDwlQvNtHhNJtarm9OewQwVntv");
        assert!(pl("https://www.youtube.com/playlist?list=PL2aBZuCeDwlQvNtHhNJtarm9OewQwVntv"));
        assert!(pl("https://youtube.com/playlist?list=PL2aBZuCeDwlQvNtHhNJtarm9OewQwVntv&si=abc"));
        let v = |q: &str| matches!(classify_link(q), Link::Video(ref id) if id == "dQw4w9WgXcQ");
        assert!(v("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
        assert!(v("https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=PLabcdefghijk"));
        assert!(v("https://youtu.be/dQw4w9WgXcQ?t=42"));
        assert!(v("https://www.youtube.com/shorts/dQw4w9WgXcQ"));
        // ordinary searches are not links
        assert!(matches!(classify_link("youtube tutorial"), Link::None));
        assert!(matches!(classify_link("how to watch?v=1"), Link::None));
    }

    /// Favorites and Watch later cannot be deleted.
    #[test]
    fn builtin_lists_are_protected() {
        assert!(is_builtin_list("favorites") && is_builtin_list("watchlater"));
        assert!(!is_builtin_list("pl-123"));
    }
}

mod backup {
    use crate::*;

    /// A backup from another app or with odd file names is refused.
    #[test]
    fn restore_rejects_foreign_files() {
        assert!(restore_bundle("not json").is_err());
        assert!(restore_bundle(r#"{"app":"other","files":{}}"#).is_err());
        // names outside the known set are ignored, so nothing is written
        assert!(restore_bundle(r#"{"app":"tubeglass","files":{"../../evil.json":{}}}"#).is_err());
    }
}

#[test]
fn video_age_is_read_from_its_text() {
    use crate::util::age_secs;
    assert_eq!(age_secs("1.2M  •  3d ago"), 3 * 86_400);
    assert_eq!(age_secs("N views  •  Streamed 2 months ago".replace('N', "5").as_str()), 2 * 2_592_000);
    assert_eq!(age_secs("4K  •  12\u{a0}min ago"), 720);
    assert_eq!(age_secs("9K  •  1 year ago"), 31_536_000);
    assert_eq!(age_secs("2w ago"), 2 * 604_800);
    assert!(age_secs("12 watching") == u64::MAX);
    assert!(age_secs("2h ago") < age_secs("1d ago"));
    assert!(age_secs("3w ago") < age_secs("1mo ago"));
}

#[test]
fn list_tools_read_their_inputs() {
    use crate::util::{duration_secs, playlist_ids, view_count};
    assert_eq!(view_count("Chill Music Lab  •  15K  •  2d ago"), Some(15_000));
    assert_eq!(view_count("1.2M  •  3d ago"), Some(1_200_000));
    assert_eq!(view_count("845 views  •  Streamed 2mo ago"), Some(845));
    assert_eq!(view_count("1,234 views"), Some(1234));
    assert_eq!(view_count("MrBeast  •  3d ago"), None);
    assert_eq!(duration_secs("1:02:03"), 3723);
    assert_eq!(duration_secs("4:05"), 245);
    assert_eq!(duration_secs(""), 0);
    let t = "https://www.youtube.com/playlist?list=PLabcdefghijklmnop&si=x https://youtu.be/x?list=PLzzzzzzzzzzzzzz, PLabcdefghijklmnop, hello";
    assert_eq!(playlist_ids(t), vec!["PLabcdefghijklmnop".to_string(), "PLzzzzzzzzzzzzzz".to_string()]);
}

mod playback_shortcuts {
    use crate::Repeat;
    use crate::ui::keys::{chapter_target, next_repeat, percentage_target, player_key_allowed};
    use eframe::egui::{Key, Modifiers};

    #[test]
    fn repeat_cycles_in_order() {
        let one = next_repeat(Repeat::Off);
        assert!(one == Repeat::One);
        let all = next_repeat(one);
        assert!(all == Repeat::All);
        assert!(next_repeat(all) == Repeat::Off);
    }

    #[test]
    fn percentages_and_invalid_media() {
        for digit in 0..10 {
            assert_eq!(
                percentage_target(200.0, digit, false),
                Some(f64::from(digit) * 20.0)
            );
        }
        for duration in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(percentage_target(duration, 5, false), None);
        }
        assert_eq!(percentage_target(200.0, 5, true), None);
        assert_eq!(percentage_target(200.0, 10, false), None);
        assert!(percentage_target(f64::MAX, 9, false).unwrap().is_finite());
    }

    #[test]
    fn chapter_boundaries_and_missing_targets() {
        let target = |position, next| chapter_target([0.0, 10.0, 20.0].into_iter(), position, next);
        assert_eq!(target(0.0, true), Some(10.0));
        assert_eq!(target(10.0, true), Some(20.0));
        assert_eq!(target(20.0, true), None);
        assert_eq!(target(13.0, false), Some(0.0));
        assert_eq!(target(13.001, false), Some(10.0));
        assert_eq!(target(10.0, false), Some(0.0));
        assert_eq!(target(3.0, false), None);
        assert_eq!(target(3.001, false), Some(0.0));
        assert_eq!(target(-1.0, false), None);
        assert_eq!(target(f64::NAN, true), None);
        assert_eq!(chapter_target([].into_iter(), 5.0, false), None);
        assert_eq!(chapter_target([10.0].into_iter(), 5.0, false), None);
        assert_eq!(
            chapter_target([20.0, 0.0, 10.0, 10.0, f64::NAN].into_iter(), 13.0, false),
            Some(0.0)
        );
    }

    #[test]
    fn modifiers_only_allow_supported_combinations() {
        for key in [
            Key::B,
            Key::R,
            Key::S,
            Key::Num5,
            Key::Escape,
            Key::ArrowLeft,
        ] {
            assert!(player_key_allowed(key, Modifiers::NONE));
            for modifiers in [
                Modifiers::CTRL,
                Modifiers::COMMAND,
                Modifiers::ALT,
                Modifiers::MAC_CMD,
            ] {
                assert!(!player_key_allowed(key, modifiers));
                assert!(!player_key_allowed(key, modifiers | Modifiers::SHIFT));
            }
        }
        for key in [Key::ArrowLeft, Key::ArrowRight, Key::Questionmark] {
            assert!(player_key_allowed(key, Modifiers::SHIFT));
        }
        for key in [Key::B, Key::R, Key::S, Key::Num5, Key::Space, Key::Escape] {
            assert!(!player_key_allowed(key, Modifiers::SHIFT));
        }
    }
}

mod global_shortcuts {
    use crate::ui::keys::{
        EscapeContext, SEARCH_SHORTCUT, escape_context, search_destination, search_key,
    };
    use crate::*;

    #[test]
    fn search_requires_only_platform_command() {
        use egui::{Key, Modifiers as M};
        for platform in [M::CTRL | M::COMMAND, M::MAC_CMD | M::COMMAND] {
            assert!(search_key(Key::K, platform));
            assert!(!search_key(Key::K, platform | M::SHIFT));
            assert!(!search_key(Key::K, platform | M::ALT));
            assert!(!search_key(Key::O, platform));
            assert!(!crate::ui::keys::player_key_allowed(Key::K, platform));
        }
        assert!(!search_key(Key::K, M::NONE));
        assert!(!search_key(Key::K, M::CTRL | M::MAC_CMD | M::COMMAND));
        assert!(crate::ui::keys::player_key_allowed(Key::K, M::NONE));
    }

    #[test]
    fn search_routing_keeps_grid_and_music_context() {
        for page in [
            Sel::Home,
            Sel::Search,
            Sel::History,
            Sel::Channel("channel".into()),
            Sel::Music,
            Sel::MusicAlbum("album".into()),
            Sel::MusicArtist("artist".into()),
        ] {
            assert!(search_destination(&page, false) == page);
        }
        for page in [
            Sel::Settings,
            Sel::Downloads,
            Sel::Stats,
            Sel::Clips,
            Sel::Podcasts,
            Sel::Podcast("feed".into()),
        ] {
            assert!(search_destination(&page, false) == Sel::Home);
        }
        assert!(search_destination(&Sel::Music, true) == Sel::Home);
    }

    #[test]
    fn escape_chooses_one_topmost_context() {
        use EscapeContext::*;
        let contexts = [
            Popup,
            Help,
            Group,
            Import,
            List,
            Notes,
            Suggestions,
            Editing,
            Fullscreen,
            Player,
            History,
        ];
        assert_eq!(escape_context(&[false; 11]), None);
        for (index, expected) in contexts.into_iter().enumerate() {
            let mut active = [true; 11];
            active[..index].fill(false);
            assert_eq!(escape_context(&active), Some(expected));
        }
    }

    #[test]
    fn consumed_escape_cannot_reach_another_handler() {
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        };
        let _ = ctx.run_ui(input, |ui| {
            let ctx = ui.ctx();
            assert!(ctx.input(|i| i.key_pressed(egui::Key::Escape)));
            crate::ui::keys::consume_escape(ctx);
            assert!(!ctx.input(|i| i.key_pressed(egui::Key::Escape)));
        });
    }

    #[test]
    fn shortcut_reference_matches_readme() {
        let readme = include_str!("../README.md");
        assert!(readme.contains(SEARCH_SHORTCUT.0));
        assert!(readme.contains(SEARCH_SHORTCUT.1));
    }
}

#[test]
fn search_does_not_steal_other_editors_or_modal_input() {
    use crate::ui::keys::search_allowed;
    assert!(search_allowed(false, false, false, false, false));
    assert!(search_allowed(false, false, false, true, true));
    assert!(!search_allowed(false, false, false, true, false));
    assert!(!search_allowed(true, false, false, false, false));
    assert!(!search_allowed(false, true, false, false, false));
    assert!(!search_allowed(false, false, true, true, true));
}
