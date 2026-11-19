//! Le tableau de bord : une carte par drone connu de la bibliothèque, et les logs de celui qu'on choisit.
//!
//! C'est l'écran qui rend visible `otb-store` (lot 28) : les drones et les logs y sont ceux que la bibliothèque a
//! retenus, pas seulement ceux qui sont ouverts en ce moment. Un drone est reconnu par le **nom de machine** de
//! l'en-tête des logs (`Craft name`) tant que la récupération par USB n'apporte pas l'identifiant de la carte
//! (lot 32) : ce choix est écrit ici parce qu'il décide de ce que l'utilisateur voit.

use egui::{RichText, Ui};
use otb_store::Library;
use std::path::PathBuf;

use crate::state::AppState;
use crate::theme::Theme;

/// Ce que l'utilisateur demande depuis le tableau de bord.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DashAction {
    /// Ouvrir le Guide sur ce drone (nom de la fiche).
    Guide(String),
    /// Ouvrir ce fichier de log (connu de la bibliothèque, pas forcément ouvert).
    Open(PathBuf),
}

/// Date `AAAA-MM-JJ` d'un instant donné en secondes depuis 1970 (UTC).
///
/// La conversion « jours → date civile » est celle de `rust/otb-gui/src/export.rs` (nom des captures) : une seule
/// implémentation dans le programme, et elle est déjà éprouvée par ses tests.
pub fn date_iso(epoch_secs: i64) -> String {
    let (y, m, d) = crate::export::date_from_secs(epoch_secs);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Date de dernière modification d'un fichier, en `AAAA-MM-JJ`, ou `""` si le système ne la donne pas.
///
/// C'est la date du **fichier**, pas celle du vol : l'en-tête des logs porte bien un « Log start datetime », mais
/// il vaut `0000-01-01` dès que la carte de vol n'a ni horloge ni GPS à l'armement — le cas de la plupart des
/// logs du corpus.
pub fn file_date(path: &std::path::Path) -> String {
    let Ok(meta) = std::fs::metadata(path) else {
        return String::new();
    };
    let Ok(modified) = meta.modified() else {
        return String::new();
    };
    match modified.duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => date_iso(d.as_secs() as i64),
        Err(_) => String::new(),
    }
}

/// L'écran du tableau de bord : le drone choisi, dont les logs s'affichent à droite.
#[derive(Debug, Default)]
pub struct DashboardPage {
    /// Identifiant du drone choisi (`None` : aucun panneau ouvert).
    pub selected: Option<String>,
}

impl DashboardPage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut Ui, lib: &Library, state: &AppState, th: &Theme) -> Vec<DashAction> {
        let mut actions = Vec::new();
        // le panneau du drone choisi, à droite, avec ses logs du plus récent au plus ancien
        let chosen = self
            .selected
            .clone()
            .filter(|id| lib.drone(id).is_some() || lib.logs().iter().any(|l| &l.drone_id == id));
        if let Some(id) = chosen {
            egui::Panel::right("tableau-drone")
                .default_size(320.0)
                .size_range(260.0..=520.0)
                .show(ui, |ui| self.drone_panel(ui, lib, state, th, &id, &mut actions));
        }
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Tableau de bord");
            if lib.drones().is_empty() {
                ui.add_space(8.0);
                ui.label(
                    "Aucun drone connu pour l'instant. Ouvrez un log : son nom de machine devient un drone, et \
                     la bibliothèque le retient d'un lancement à l'autre.",
                );
                return;
            }
            ui.add_space(8.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for drone in lib.drones() {
                        self.card(ui, lib, th, &drone.id, &mut actions);
                    }
                });
            });
        });
        actions
    }

    /// Une carte de drone : son nom, son firmware, ses logs connus et la date du plus récent.
    fn card(&mut self, ui: &mut Ui, lib: &Library, th: &Theme, id: &str, actions: &mut Vec<DashAction>) {
        let drone = lib.drone(id);
        let nom = drone.map_or(id, |d| d.name.as_str()).to_string();
        let firmware = drone.map(|d| d.firmware.clone()).unwrap_or_default();
        let logs: Vec<_> = lib.logs().iter().filter(|l| l.drone_id == id).collect();
        let dernier = logs.iter().map(|l| l.added.as_str()).max().unwrap_or("").to_string();
        let choisi = self.selected.as_deref() == Some(id);
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_width(280.0);
            ui.vertical(|ui| {
                ui.label(RichText::new(&nom).strong().size(16.0).color(th.accent("blue")));
                if !firmware.is_empty() {
                    ui.label(RichText::new(&firmware).weak());
                }
                ui.label(format!(
                    "{} log{} connu{}{}",
                    logs.len(),
                    if logs.len() > 1 { "s" } else { "" },
                    if logs.len() > 1 { "s" } else { "" },
                    if dernier.is_empty() {
                        String::new()
                    } else {
                        format!(" · dernier : {dernier}")
                    }
                ));
                ui.horizontal(|ui| {
                    let voir = if choisi { "Fermer" } else { "Voir ses logs" };
                    if ui.button(voir).clicked() {
                        self.selected = if choisi { None } else { Some(id.to_string()) };
                    }
                    if ui.button("Ouvrir le Guide").clicked() {
                        actions.push(DashAction::Guide(nom.clone()));
                    }
                });
            });
        });
    }

    /// Le panneau de droite : les logs du drone choisi, et de quoi les ouvrir.
    fn drone_panel(
        &mut self,
        ui: &mut Ui,
        lib: &Library,
        state: &AppState,
        th: &Theme,
        id: &str,
        actions: &mut Vec<DashAction>,
    ) {
        let nom = lib.drone(id).map_or(id, |d| d.name.as_str()).to_string();
        ui.horizontal(|ui| {
            ui.label(RichText::new(&nom).strong());
            if ui.button("Fermer").clicked() {
                self.selected = None;
            }
        });
        ui.separator();
        let mut logs: Vec<_> = lib.logs().iter().filter(|l| l.drone_id == id).collect();
        // le plus récent en haut, comme la timeline des maquettes
        logs.sort_by(|a, b| b.added.cmp(&a.added).then(a.name.cmp(&b.name)));
        if logs.is_empty() {
            ui.label("Aucun log connu pour ce drone.");
            return;
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            for l in logs {
                let path = PathBuf::from(&l.path);
                // un log déjà ouvert ne se rouvre pas : il est marqué comme tel
                let ouvert = state.logs.iter().any(|o| o.log.path == path);
                ui.horizontal(|ui| {
                    if ouvert {
                        // ⚠️ « ✓ » n'existe dans aucune police embarquée : le banc panique et l'application
                        // dessine un carré. Les signes de ce genre passent par une icône (L-068).
                        ui.add(
                            crate::icons::icon("check", 14.0)
                                .tint(th.accent("green"))
                                .alt_text("Déjà ouvert"),
                        );
                    } else if ui.button("+").on_hover_text("Ouvrir ce log").clicked() {
                        actions.push(DashAction::Open(path.clone()));
                    }
                    ui.label(RichText::new(&l.name).strong());
                });
                ui.label(RichText::new(format!("{} · {} ko", l.added, l.bytes / 1000)).weak());
                ui.separator();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::date_iso;

    /// La date d'un enregistrement de la bibliothèque, au format qui sert aussi à la ranger (l'ordre
    /// alphabétique de `AAAA-MM-JJ` est l'ordre chronologique).
    #[test]
    fn la_date_suit_le_calendrier() {
        assert_eq!(date_iso(0), "1970-01-01");
        assert_eq!(date_iso(86_399), "1970-01-01", "la veille de minuit");
        assert_eq!(date_iso(86_400), "1970-01-02");
        // une année bissextile et son 29 février
        assert_eq!(date_iso(1_709_164_800), "2024-02-29");
        assert_eq!(date_iso(1_763_510_400), "2025-11-19");
        // 2000 est bissextile, 1900 ne l'était pas : la règle des siècles
        assert_eq!(date_iso(951_782_400), "2000-02-29");
        assert_eq!(date_iso(-2_208_988_800), "1900-01-01");
        // 1900 n'est pas bissextile : au 28 février succède le 1er mars
        assert_eq!(date_iso(-2_203_977_600), "1900-02-28");
        assert_eq!(
            date_iso(-2_203_977_600 + 86_400),
            "1900-03-01",
            "pas de 29 février en 1900"
        );
    }
}
