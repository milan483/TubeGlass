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
        if ctx.egui_wants_keyboard_input()
            || self.watching.is_none()
            || self.shortcut_modal_open()
            || egui::Popup::is_any_open(ctx)
            || self.ime_composing
        {
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

pub(crate) const SEARCH_SHORTCUT: (&str, &str) = ("Ctrl/Cmd+K", "Focus search (select query)");

pub(crate) fn search_key(key: egui::Key, m: egui::Modifiers) -> bool {
    key == egui::Key::K && m.command && !m.shift && !m.alt && !(m.ctrl && m.mac_cmd)
}

pub(crate) fn search_allowed(
    modal: bool,
    popup: bool,
    composing: bool,
    editing: bool,
    own_search: bool,
) -> bool {
    !modal && !popup && !composing && (!editing || own_search)
}

pub(crate) fn search_destination(sel: &Sel, full_player: bool) -> Sel {
    if full_player
        || matches!(
            sel,
            Sel::Settings
                | Sel::Downloads
                | Sel::Stats
                | Sel::Clips
                | Sel::Podcasts
                | Sel::Podcast(_)
        )
    {
        Sel::Home
    } else {
        sel.clone()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum EscapeContext {
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
}

pub(crate) fn consume_escape(ctx: &egui::Context) {
    ctx.input_mut(|i| {
        i.events.retain(|e| {
            !matches!(
                e,
                egui::Event::Key {
                    key: egui::Key::Escape,
                    ..
                }
            )
        })
    });
}

pub(crate) fn escape_context(active: &[bool; 11]) -> Option<EscapeContext> {
    use EscapeContext::*;
    [
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
    ]
    .into_iter()
    .zip(active)
    .find_map(|(context, active)| active.then_some(context))
}

impl App {
    pub(crate) fn shortcut_modal_open(&self) -> bool {
        self.picking
            || self.show_help
            || self.new_group.is_some()
            || self.import_list.is_some()
            || self.new_list.is_some()
            || self.update_notes_open
    }

    /// Route before widgets and player handlers; consuming Escape prevents same-frame fallthrough.
    #[allow(deprecated)] // Some backends still emit the legacy IME Disabled event.
    pub(crate) fn global_keys(&mut self, ctx: &egui::Context) {
        let ime_event = ctx.input(|i| i.events.iter().any(|e| matches!(e, egui::Event::Ime(_))));
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Ime(event) = event {
                    match event {
                        egui::ImeEvent::Preedit { text, .. } => {
                            self.ime_composing = !text.is_empty()
                        }
                        egui::ImeEvent::Commit(_) | egui::ImeEvent::Disabled => {
                            self.ime_composing = false
                        }
                        _ => {}
                    }
                }
            }
        });
        if self.picking || self.ime_composing || ime_event {
            return;
        }
        let popup = egui::Popup::is_any_open(ctx);
        let editing = ctx.egui_wants_keyboard_input();
        let own_search = ctx
            .memory(|m| m.focused())
            .is_some_and(|id| Some(id) == self.search_field);
        let search = ctx.input(|i| {
            i.events.iter().any(|e| {
                matches!(e,
            egui::Event::Key { key, modifiers, pressed: true, .. } if search_key(*key, *modifiers))
            })
        });
        if search
            && search_allowed(
                self.shortcut_modal_open(),
                popup,
                self.ime_composing || ime_event,
                editing,
                own_search,
            )
        {
            ctx.input_mut(|i| {
                i.events.retain(|e| {
                    !matches!(e,
                egui::Event::Key { key, modifiers, .. } if search_key(*key, *modifiers))
                })
            });
            let full = self.watching.is_some() && !self.pip;
            let destination = search_destination(&self.sel, full || self.mini);
            if self.mini {
                self.set_mini(false);
            }
            if full {
                self.enter_pip();
            }
            self.sel = destination;
            self.focus_search = true;
            self.release_focus = false;
        }
        if !ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            return;
        }
        let context = escape_context(&[
            popup,
            self.show_help,
            self.new_group.is_some(),
            self.import_list.is_some(),
            self.new_list.is_some(),
            self.update_notes_open,
            self.sugg_rect.is_some() || self.music_sugg_rect.is_some(),
            editing,
            self.watching.is_some() && !self.pip && self.fullscreen,
            self.watching.is_some() && !self.pip,
            self.can_go_back(),
        ]);
        // Leave Escape to a focused text editor when no overlay is present.
        if context == Some(EscapeContext::Editing) {
            return;
        }
        if context.is_some() {
            consume_escape(ctx);
        }
        match context {
            Some(EscapeContext::Popup) => egui::Popup::close_all(ctx),
            Some(EscapeContext::Help) => self.show_help = false,
            Some(EscapeContext::Group) => self.new_group = None,
            Some(EscapeContext::Import) => self.import_list = None,
            Some(EscapeContext::List) => self.new_list = None,
            Some(EscapeContext::Notes) => self.update_notes_open = false,
            Some(EscapeContext::Suggestions) => {
                self.sugg_rect = None;
                self.music_sugg_rect = None;
                if let Some(id) = ctx.memory(|m| m.focused()) {
                    ctx.memory_mut(|m| m.surrender_focus(id));
                }
            }
            Some(EscapeContext::Fullscreen) => self.toggle_fullscreen(),
            Some(EscapeContext::Player) => self.back_from_player(),
            Some(EscapeContext::History) => self.go_back(),
            _ => {}
        }
    }
}
