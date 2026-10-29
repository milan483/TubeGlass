//! Keyboard shortcuts while something is playing.

use crate::*;

pub(crate) fn next_repeat(repeat: Repeat) -> Repeat {
    match repeat {
        Repeat::Off => Repeat::One,
        Repeat::One => Repeat::All,
        Repeat::All => Repeat::Off,
    }
}

pub(crate) fn percentage_target(duration: f64, digit: u8, live: bool) -> Option<f64> {
    (!live && duration.is_finite() && duration > 0.0 && digit < 10)
        .then(|| duration * (f64::from(digit) / 10.0))
}

pub(crate) fn chapter_target(
    starts: impl Iterator<Item = f64>,
    position: f64,
    next: bool,
) -> Option<f64> {
    if !position.is_finite() {
        return None;
    }
    let mut current: Option<f64> = None;
    let mut previous: Option<f64> = None;
    let mut following: Option<f64> = None;
    for start in starts.filter(|s| s.is_finite() && *s >= 0.0) {
        if start > position {
            following = Some(following.map_or(start, |s| s.min(start)));
        } else if current.is_none_or(|s| start > s) {
            previous = current;
            current = Some(start);
        } else if Some(start) != current && previous.is_none_or(|s| start > s) {
            previous = Some(start);
        }
    }
    if next {
        following
    } else {
        current.and_then(|start| {
            if position - start > 3.0 {
                Some(start)
            } else {
                previous
            }
        })
    }
}

pub(crate) fn player_key_allowed(key: egui::Key, modifiers: egui::Modifiers) -> bool {
    if modifiers.alt || modifiers.ctrl || modifiers.command || modifiers.mac_cmd {
        return false;
    }
    !modifiers.shift
        || matches!(
            key,
            egui::Key::ArrowLeft | egui::Key::ArrowRight | egui::Key::Questionmark
        )
}

impl App {
    /// Handles the keyboard shortcuts of the player.
    pub(crate) fn keys(&mut self, ctx: &egui::Context) {
        if ctx.egui_wants_keyboard_input() || self.watching.is_none() {
            return;
        }
        use egui::Key::*;
        let pressed = |k| {
            ctx.input(|i| {
                i.events.iter().any(|event| {
                    matches!(event, egui::Event::Key { key, pressed: true, modifiers, .. }
                if *key == k && player_key_allowed(*key, *modifiers))
                })
            })
        };
        let chapter_pressed = |k| {
            ctx.input(|i| {
                i.events.iter().any(|event| {
                    matches!(event, egui::Event::Key { key, pressed: true, modifiers, .. }
                if *key == k && modifiers.shift && player_key_allowed(*key, *modifiers))
                })
            })
        };
        let live = self.player.is_live();
        if pressed(Space) || pressed(K) {
            self.player.toggle_pause();
        }
        if !live {
            let p = self.player.position();
            if chapter_pressed(ArrowLeft) || chapter_pressed(ArrowRight) {
                if let Some(target) = self.player.streams.as_ref().and_then(|streams| {
                    chapter_target(
                        streams.chapters.iter().map(|c| c.start),
                        p,
                        chapter_pressed(ArrowRight),
                    )
                }) {
                    self.player.seek(target);
                }
            } else {
                if pressed(ArrowLeft) {
                    self.player.seek(p - 5.0);
                }
                if pressed(ArrowRight) {
                    self.player.seek(p + 5.0);
                }
            }
            if pressed(J) {
                self.player.seek(p - 10.0);
            }
            if pressed(L) {
                self.player.seek(p + 10.0);
            }
        }
        for (digit, key) in [Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9]
            .into_iter()
            .enumerate()
        {
            if pressed(key) {
                if let Some(target) = percentage_target(self.player.duration(), digit as u8, live) {
                    self.player.seek(target);
                }
            }
        }
        if pressed(R) {
            self.settings.repeat = next_repeat(self.settings.repeat);
            self.save_settings();
            self.toast(match self.settings.repeat {
                Repeat::Off => "Repeat off",
                Repeat::One => "Repeat this video",
                Repeat::All => "Repeat the queue",
            });
        }
        if pressed(S) {
            self.settings.shuffle = !self.settings.shuffle;
            self.save_settings();
            self.toast(if self.settings.shuffle {
                "Shuffle on"
            } else {
                "Shuffle off"
            });
        }
        if pressed(ArrowUp) {
            self.player.set_volume(self.player.volume + 0.05);
        }
        if pressed(ArrowDown) {
            self.player.set_volume(self.player.volume - 0.05);
        }
        if pressed(M) {
            self.player.toggle_mute();
        }
        if pressed(F) {
            self.toggle_fullscreen();
        }
        if pressed(Questionmark) {
            self.show_help = !self.show_help;
        }
        if pressed(N) {
            self.queue_next();
        }
        if pressed(P) {
            self.queue_prev();
        }
        if pressed(Q) {
            if self.comments_open && self.side_tab == SideTab::Queue {
                self.comments_open = false;
            } else {
                self.side_tab = SideTab::Queue;
                self.comments_open = true;
            }
        }
        if pressed(B) {
            self.settings.ambient = !self.settings.ambient;
            self.save_settings();
            self.toast(if self.settings.ambient {
                "Ambient mode on"
            } else {
                "Ambient mode off"
            });
        }
        if pressed(W) {
            if let Some(w) = self
                .watching
                .clone()
                .filter(|w| !w.id.starts_with("local:"))
            {
                self.toggle_in_list("watchlater", &w);
                let on = self
                    .lists
                    .iter()
                    .any(|l| l.id == "watchlater" && l.videos.iter().any(|x| x.id == w.id));
                self.toast(if on {
                    "Added to Watch later"
                } else {
                    "Removed from Watch later"
                });
            }
        }
        if pressed(Y) && self.watching.as_ref().is_some_and(|w| w.music) {
            if self.comments_open && self.side_tab == SideTab::Lyrics {
                self.comments_open = false;
            } else {
                self.side_tab = SideTab::Lyrics;
                self.comments_open = true;
            }
        }
        if pressed(C) {
            self.toggle_comments();
        }
        if pressed(OpenBracket) {
            let v = (self.player.speed - 0.25).max(0.5);
            self.player.set_speed(v);
            self.settings.speed = v;
            self.save_settings();
            self.toast(format!("Speed {v}×"));
        }
        if pressed(CloseBracket) {
            let v = (self.player.speed + 0.25).min(2.0);
            self.player.set_speed(v);
            self.settings.speed = v;
            self.save_settings();
            self.toast(format!("Speed {v}×"));
        }
        if pressed(I) {
            let on = !self.mini;
            self.set_mini(on);
        }
        if pressed(A) {
            let on = !self.player.audio_mode;
            self.player.set_audio_mode(on);
        }
        if pressed(Escape) && !self.pip && !self.show_help {
            if self.fullscreen {
                self.toggle_fullscreen();
            } else {
                self.back_from_player();
            }
        }
        // a key press shows the controls for a moment, like moving the mouse does
        if ctx.input(|i| {
            i.events
                .iter()
                .any(|e| matches!(e, egui::Event::Key { pressed: true, .. }))
        }) {
            self.last_move = Instant::now();
        }
    }
}
