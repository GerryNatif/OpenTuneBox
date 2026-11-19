# Plan de travail

**Le seul plan du projet.** Ce qu'il reste à faire est en haut ; l'historique des lots déjà faits est un
tableau en bas, une ligne par lot. Il n'y a **pas d'autre document de plan** : l'état d'un travail reporté
se lit dans [travaux-reportes.md](travaux-reportes.md), jamais ici (L-092).

**État au 2026-11-10, vérifié** : la **0.9.0 est publiée** (page « dernière version », AppImage et
installateur Windows), les huit outils de PIDtoolbox Pro V0.94 sont tous couverts
([spec-ecrans.md](spec-ecrans.md), `rust/otb-gui/src/tools/mod.rs:all_pages`), `main` est propre, la CI est
verte, et `cargo run -p otb-dev -- verify --strict` avec le corpus, les références figées et la comparaison
au bit près : **0 saut, tout passe**.

Le programme est donc complet et publié, et depuis le 2026-11-10 **il ne reste que du confort** (lot 15 ;
le lot 11 est fait le même jour). Les deux autres familles ont disparu le même jour : vérifier ce que seule une vraie machine vérifie
(lot 13) n'avait plus d'objet, **personne n'ayant jamais utilisé la 0.8.0** ; et **plus aucun seuil de
l'assistant n'est sans origine ni condition** (lot 14), sans qu'une seule valeur ait eu à changer. Le 2026-11-12, le
lot 21 a relu les licences livrées et les sources, en vue de la **0.10.0** (la 1.0 est gardée pour plus tard) ;
la version du dépôt est 0.10.0.

**Chantier ouvert le 2026-11-18 — scripts firmware.** Une étude du 2026-11-17 (document de travail de Cédric,
hors dépôt) propose de faire d'OpenTuneBox un outil **multi-firmware** — Betaflight, INAV, ArduCopter — par un
**script par firmware et par plage de versions** : le script *décrit* la chaîne de filtres et de PID sous forme de
données et *fournit* les briques de calcul avec l'arithmétique exacte de sa famille, le cœur assemble et exécute.
Elle est reprise ici en lots **22 à 25**, dont le **22 est fait** (D-057, 2026-11-18) : il a consigné les quatre
arbitrages de Cédric, le contrat des sept fonctions et les plages de versions
([spec-betaflight.md](spec-betaflight.md) §8). La chaîne y est décrite en **données** avec des briques compilées en Rust (pas de moteur de script) ; le **banc de référence vit
dans le dépôt** et ne copie aucun fichier de firmware ; la nouvelle chaîne de calcul **remplace l'ancienne d'un
coup** au lot 24 ; et le périmètre reste **Betaflight seul**, ce qui a fait sortir du plan INAV (lot 26, W-001 reste
fermé) et ArduCopter (lot 27, désormais W-057). Trois des sept questions ouvertes de l'étude avaient déjà leur
réponse dans les registres — la licence au premier chef (D-045 : **aucune dépendance à copyleft**, et les trois
firmwares sont sous GPL 3). Ce chantier **ne corrige aucun défaut connu** : il ajoute de la justesse.

**Second chantier, du même moment — bibliothèque, refonte et USB.** Deux autres études de Cédric, également
hors dépôt, ouvrent un chantier distinct et bien plus proche de l'utilisateur : un **canevas de neuf maquettes**
du 2026-11-16 (tableau de bord par drone, navigation à deux niveaux, espace « Bruit et filtres » fusionné, écran
d'extraction d'un **fragment** de vol, assistant en cartes) et une **spécification** du 2026-11-14 (récupération
automatique des logs quand on branche un drone en USB, **stockage** des logs et des fragments, petite base
**SQLite**, suivi des mesures dans le temps contre une fiche cible). Repris en lots **28 à 32**, dont le **28 est fait**
(2026-11-18, crate `otb-store`) : la bibliothèque de données d'abord, parce que les fragments, la refonte et l'USB
s'appuient tous dessus. Celui-ci
**change la vie d'un pilote** — moins de gestes entre l'atterrissage et l'analyse, et des logs qui ne se perdent
plus. Deux arbitrages sont déjà rendus (2026-11-18) : le stockage se fait en **fichiers JSON** et non en base
(lot 28), et **D-015 est amendée** pour laisser une entrée de navigation regrouper plusieurs vues (lot 30). Il ne
reste qu'une seule dépendance nouvelle à introduire, pour l'USB (lot 32).

## Comment s'en servir

- **Une session = un lot = une PR.** Faire `/clear`, puis demander « Lot N de `docs/plan.md` ». La branche
  porte le nom indiqué dans le lot ; la PR a pour titre une phrase en français qui dit l'effet du changement et
  une seule étiquette `RN: …` (`CONTRIBUTING.md`, D-056).
- Chaque lot suit les phases **explorer → planifier → implémenter → vérifier → livrer → consigner**.
  Explorer = lire les fichiers listés dans « À lire ». Planifier = présenter le découpage et attendre
  l'accord de Cédric. Consigner = cocher les cases du lot, mettre à jour les registres ([decisions.md](decisions.md), [lecons.md](lecons.md),
  [travaux-reportes.md](travaux-reportes.md)) selon la compétence `docs-sync`.
- **Un nouveau test se prouve par contre-preuve** : réintroduire le défaut (ou le casser à la main), voir le
  test rougir, le noter dans la PR (L-071).
- **Vérification de tout lot** : `cargo run -p otb-dev -- verify` **sans tube** (un `| tail` masque son
  échec, L-022), vert avant de livrer.
- **Critère d'arrêt** : si une hypothèse d'un lot tombe, s'arrêter, consigner ce qui a été vu, et revenir
  vers Cédric ; ne pas improviser un autre découpage.
- Modèle conseillé par lot : **Sonnet** pour le mécanique, la documentation, les tests ; **Opus** pour le
  décodeur, la mémoire et le débogage difficile.
- La numérotation des lots est une **série unique**, jamais réutilisée : les lots 0 à 14 et 16 à 21
  sont au tableau en bas (faits, non retenus ou supprimés), les lots 23 à 25 et 28 à 32 sont ouverts
  ci-dessous, les lots 26 et 27 sont sortis du plan le 2026-11-18, et le prochain numéro libre est **33**.

## Ordre des étapes

| # | Lot | Sujet | Dépend de |
|---|---|---|---|
| 1 | **15** | Dette de l'interface (W-052, pages spectrales) — **conditionnel** | le volet « cache » est soldé (L-094, D-053) ; le reste attend un déclencheur mesuré |
| 2 | **23** | Chaîne Betaflight décrite en données, à résultat identique | D-057 |
| 3 | **24** | Briques à l'arithmétique du firmware, vecteurs de référence **et bascule** | lot 23 |
| 4 | **25** | Plages de versions et veille des changements du firmware | lot 24 |
| 5 | **29** | Fragments : écran d'extraction, index, liste d'analyse | `otb-store` (lot 28, fait) |
| 6 | **30** | Refonte de la navigation et de la mise en page (les neuf maquettes) | lot 29, et le lot 23 pour l'espace « Bruit et filtres » |
| 7 | **31** | Mesures suivies dans le temps et fiche cible par drone | lot 29 |
| 8 | **32** | Récupération des logs par USB (MSP / MSC) — le plus risqué | lot 30 |

Il ne reste que du **confort**, mesuré comme tel : le budget de 60 images par seconde est tenu avec une marge
de 3 sur la plus longue session du corpus. Les lots qui touchaient à un utilisateur réel ont été réglés le
2026-11-10 : le **13** s'est avéré sans objet (personne n'a eu la 0.8.0), le **14** est fait sans qu'aucune
valeur de seuil ait eu à changer, et le **11** a livré l'ouverture de plusieurs logs à la fois (× 2,26). Il ne
reste que le **15**, dont le volet « cache des outils » est **soldé le 2026-11-10** : mesuré (L-094), allégé, puis borné
(D-053) — **W-035 est clos**. Ses autres volets (tracé du viewer, pages spectrales) attendent toujours leur déclencheur
mesuré : ils peuvent attendre que les retours des pilotes disent ce qui gêne vraiment.

Ce constat vaut pour le programme **tel qu'il est**. Les lots **22 à 27** ne le changent pas : ils n'accélèrent
rien et ne corrigent rien pour un pilote Betaflight d'aujourd'hui — ils ouvrent un périmètre neuf, et deux
d'entre eux attendent des logs que le corpus ne contient pas. Les lots **28 à 32**, eux, changent ce que vit
l'utilisateur : ils gardent ses logs, nomment ses morceaux de vol, refont la navigation et suppriment l'étape
manuelle de récupération. Si un seul chantier doit passer d'abord, c'est celui-là.

---

## Lot 15 : dette de l'interface (Opus) — **conditionnel**

**But** : reprendre les trois dettes connues de l'interface. **Ce lot ne se fait pas « pour faire bien »** :
le budget de 60 images par seconde est tenu avec une marge de 3 sur la plus longue session du corpus
(5,6 ms contre 16,7). Il attend un **déclencheur mesuré**.

**Déclencheurs** (l'un suffit) : le viewer accroche sur un log plus long que le corpus ; on veut afficher
beaucoup plus de courbes à la fois ; un pilote signale une lenteur. (La mémoire du cache des outils était le
quatrième : elle a été mesurée le 2026-11-10, et le cache est borné depuis — D-053.)

**Branche** : `perf/viewer-draw` (W-052) ou `refactor/spectral-pages` selon le déclencheur.

**À lire** : [travaux-reportes.md](travaux-reportes.md) W-052 (W-035 est clos : son point 9 a été mesuré puis borné, D-053),
`rust/otb-gui/src/viewer_page.rs:ui`, `rust/otb-gui/src/tools/freq_time.rs` (la texture en cache, le modèle
à suivre), [decisions.md](decisions.md) D-037 et D-048, [lecons.md](lecons.md) L-085, L-086.

- [ ] **W-052** — le cadre est désormais dominé par le **tracé** des ≈ 3 840 points par courbe (4,7 ms en
      vue entière contre 2,1 ms zoomé sur un huitième du même log, à nombre de points égal). Deux pistes, à
      mesurer avant de choisir : un seul segment vertical par colonne de pixels au lieu de deux points, ou la
      courbe décimée dessinée dans une texture en cache comme `freq_time`. Preuve exigée, comme au lot 10 :
      **tracé identique** (captures au bit près, les deux thèmes) et chiffres de `banc_cadre` avant / après.
- [x] **Cache des outils : mesuré** (2026-11-10, `examples/banc_cache.rs`, tout le corpus — 535 logs, 642 sessions ;
      [validation.md](validation.md), « Mémoire du cache des outils »). Une entrée pèse 7,5 Mo (p50) à **72,6 Mo** pour
      l'assistant et 7,4 à **71,3 Mo** pour la réponse indicielle ; le simulateur de filtres, seul suspect de départ
      parce que son nombre d'entrées n'est pas borné, ne pèse que 90 ko. **69 % du rapport de l'assistant** était les
      fenêtres individuelles de la réponse indicielle, que personne n'y lit : libérées, l'entrée tombe d'un facteur 24
      (médiane) à 165 (L-094, `rust/otb-advisor/tests/rapport_memoire.rs`, contre-preuve vue en échec).
- [x] **Cache des outils borné** (2026-11-10, D-053) : plafond **par outil**, en nombre d'entrées — 3 réponses
      indicielles, 20 simulations de filtres, 6 rapports, 2 vérifications du Guide — la moins récemment utilisée jetée la
      première. Le pire cas d'un log ouvert passe de **5,1 Go à ≈ 223 Mo**. Quatre tests dans
      `rust/otb-gui/tests/state.rs` (module `plafond`), contre-preuves vues en échec ; 28 captures identiques au bit
      près. **W-035 est clos.**
- [ ] **Factorisation des pages spectrales** : les pages `spectral`, `freq_throttle` et `freq_time` répètent
      la même mécanique, comme la découpe de période (×6), `breaks` (×5), la découpe bornée (×9), les listes
      déroulantes (×5) et `reset_view` (×5). À ne faire que si une page de plus est prévue, sinon c'est du
      remaniement sans effet.
- [ ] **Lints pédantiques ciblés** : activer quelques lints `clippy::pedantic` utiles, **jamais
      `suboptimal_flops`** (L-051 : il change le résultat des calculs).
- [ ] **Critère d'arrêt** : un tracé qui diffère d'un pixel entre avant et après, ou un gain inférieur
      à ×1,5.

---

---

## Lot 23 : la chaîne Betaflight décrite en données (Opus)

**But** : **n'avoir plus qu'une seule description** de la chaîne de filtres, au lieu de deux, **sans changer un
seul résultat**. C'est ce qui rend un script de firmware possible : D-057 veut une chaîne décrite une fois, pas
deux fois en dur.

**Branche** : `refactor/chain-as-data`. **Suit D-057** (lot 22, fait le 2026-11-18).

**À lire** : `rust/otb-analysis/src/bf_filters.rs:gyro_chain`,
`rust/otb-analysis/src/bf_filters.rs:simulate_gyro`, `rust/otb-analysis/src/bf_filters.rs:whatif_spectrum`,
[spec-betaflight.md](spec-betaflight.md) §3 et §8, [lecons.md](lecons.md) L-048 et L-071.

**Constats du dépôt, vérifiés le 2026-11-18** (une première lecture avait conclu trop vite à deux défauts
visibles ; la vérification du chemin réellement emprunté par l'application dit autre chose) :

- **L'ordre de la chaîne est écrit deux fois.** `gyro_chain` le porte avec ses **deux cadences** (LPF2 à la cadence
  gyro, le reste à la cadence PID) et c'est **ce que l'application utilise**, par `whatif_spectrum` : spectre
  enregistré × rapport des réponses, fenêtre par fenêtre. `simulate_gyro` le porte une **seconde fois**, en dur et
  à la seule cadence du log ; il n'est appelé que par les tests et par `otb-parity`. Même chose pour le terme D.
- **Les deux écarts du chemin temporel ne sont donc pas visibles par un pilote** : la cadence unique et l'ordre
  « dériver puis filtrer » de `simulate_dterm` ne touchent que les tests et la parité, et dans un rapport de
  réponses l'ordre des filtres ne change rien (W-058 porte cette limite et sa condition de reprise).
- Reste le vrai travail : **une seule description**, dont la réponse en fréquence et la simulation temporelle se
  déduisent toutes les deux. Un script de firmware ne décrira sa chaîne qu'une fois.

- [ ] **Description de la chaîne en données** : étapes ordonnées (brique, cadence, paramètres, en service ou non),
      **prises** (où le log a enregistré chaque signal — c'est ce qui décide du badge « simulé » de
      `rust/otb-gui/src/tools/filter_sim.rs`) et **position de la dérivation**. `simulate_gyro` et `simulate_dterm`
      la lisent au lieu de porter l'ordre en dur.
- [ ] **Mesure des deux écarts** ci-dessus sur le corpus, dont un log aux gaz variables : chiffres dans
      [validation.md](validation.md). Si l'écart du terme D est visible, il est **corrigé dans ce lot** et dit dans
      la PR ; sinon il est consigné comme non mesurable et la commutation est écrite noir sur blanc.
- [ ] **Preuve d'iso-résultat** (hors correction mesurée ci-dessus) : `cargo run --release -p otb-parity -- check`
      0 écart, `verify --strict` 0 saut, et les captures de la page Simulateur **identiques au bit près** sur les
      deux thèmes.
- [ ] **Contre-preuve** (L-071) : une étape déplacée ou une cadence changée dans la description fait **rougir** un
      test, vu en échec.
- [ ] **Critère d'arrêt** : un seul résultat qui change sans explication chiffrée → s'arrêter et revenir vers Cédric.

---

## Lot 24 : briques à l'arithmétique du firmware (Opus)

**But** : atteindre le bit à bit contre le firmware compilé sur PC, **et basculer l'application sur cette nouvelle
chaîne dans le même lot** (D-057). C'est le plus gros lot du plan : il déplace un
chiffre dans presque tous les écrans.

**Branche** : `feat/firmware-exact-bricks`. **Dépend du lot 23.**

**À lire** : `rust/otb-dsp/src/lib.rs` (les noyaux actuels, D-033), [decisions.md](decisions.md) D-029 (float32),
D-047 et D-011, [travaux-reportes.md](travaux-reportes.md) W-028, [lecons.md](lecons.md) L-048 et L-051
(⚠️ jamais `suboptimal_flops` : il change le résultat des calculs).

⚠️ **Pourquoi l'ordre des étapes n'est pas négociable.** La chaîne actuelle est la référence de tout : les
références figées, les seuils de l'assistant et les captures au bit près. La bascule d'un coup a été choisie en
connaissance de cause ; elle ne tient que si chaque étape ci-dessous est **livrée et vérifiée avant la suivante**,
et qu'aucune ne saute.

- [ ] **24a — les briques, sans rien brancher.** Float32 à côté des actuelles, rangées par famille de firmware même si **seule celle de Betaflight est écrite** : tout en
      float32 sauf là où le firmware calcule lui-même en double ; approximations du firmware recopiées ; **ordre
      exact des opérations** (`état + k × (entrée − état)` n'a pas les mêmes bits que `(1 − k) × état + k × entrée`) ;
      initialisations et moment du recalcul des coefficients recopiés ; **deux domaines de cadence** réellement
      exécutés (cadence gyro, décimation, cadence PID). Rien de visible dans l'interface à ce stade.
- [ ] **24b — les vecteurs de référence.** Balayage de fréquence, bruit, échelons, fréquences de notch variables.
      La CI les rejoue et exige l'**égalité bit à bit** ; elle n'a pas le corpus, donc tout se prouve sur vecteurs
      et sur `tests/data/jeno5_5s.bbl` (L-023).
- [ ] **24c — l'écart chiffré sur tout le corpus, avant de brancher quoi que ce soit.** Différence entre l'ancienne
      et la nouvelle chaîne sur les 535 logs, consignée dans [validation.md](validation.md). **C'est le point de
      non-retour** : un écart plus grand que prévu s'explique **ici**, pas après la bascule.
- [ ] **24d — la bascule.** L'interface passe à la nouvelle chaîne, les références figées sont réenregistrées
      (W-028 en profite), et les captures sont refaites sur les deux thèmes.
- [ ] **24e — les seuils de l'assistant revérifiés** (`otb-dev corpus calibrate`, compétence `calculs-corpus`).
      ⚠️ Si un seuil change l'**avis** de l'assistant sur un vrai log, **s'arrêter** et revenir vers Cédric avec le
      log et le chiffre : c'est exactement le risque accepté en choisissant la bascule d'un coup.
- [ ] **Écart au vol réel mesuré et affiché**, jamais promis : écart entre gyro simulé et gyro enregistré, montré
      dans l'interface avec sa cause quand le log ne contient pas l'entrée des filtres à pleine cadence.
- [ ] **Critère d'arrêt** : un vecteur qui ne tombe pas au bit près sans cause identifiée (approximation, ordre des
      opérations, initialisation), ou un avis de l'assistant qui change sans explication → s'arrêter, consigner,
      revenir vers Cédric.

---

## Lot 25 : plages de versions et veille du firmware (Sonnet)

**But** : qu'un script dise de quelles versions il répond, et qu'un changement d'arithmétique en amont soit vu
**avant** qu'un pilote ne voie une simulation fausse.

**Branche** : `feat/version-ranges`. **Dépend du lot 24.**

- [ ] **Plage déclarée** par script : première version couverte, dernière version **vérifiée**, « ouverte » tant
      qu'aucun changement n'est connu. Au-delà de la dernière vérifiée, la simulation reste possible mais
      **marquée « non vérifiée »** dans l'interface.
- [ ] **Ce qui coupe une plage** (écrit dans la spécification) : arithmétique d'une brique, ordre de la chaîne,
      cadence, façon de calculer le terme D, noms ou plages des réglages. **Ce qui ne la coupe pas** : un nouveau
      réglage par défaut, un champ de log sans effet sur le calcul.
- [ ] **Veille** : un travail de CI compare, d'une version du firmware à la suivante, les fichiers qui portent le
      calcul, puis rejoue les vecteurs. Il **télécharge et compare, ne garde rien** (D-045), et prend `--locked`
      comme le reste de la CI.
- [ ] **Critère d'arrêt** : une veille qui crie à chaque version sans rien apprendre → la retirer plutôt que la
      laisser grincer.

---

## Lot 29 : fragments (Opus)

**But** : nommer et garder un morceau de vol, au lieu de redécouper à la main à chaque ouverture.

**Branche** : `feat/fragments`. **S'appuie sur `otb-store`** (lot 28, fait le 2026-11-18).

**Ce qui existe déjà** : `rust/otb-analysis/src/period.rs:auto_period` calcule la période de vol (décollage →
atterrissage) et la période fait partie de la clé du cache des outils (D-037) ; le **rôle** de chaque log (filtres,
série PID) est déjà retenu par la mémoire du Guide (D-039). Le fragment en est la généralisation : plusieurs par
session, nommés, datés, avec leurs bornes.

**Fait le 2026-11-18 — 29a, le cœur** (`rust/otb-store/src/link.rs`, `rust/otb-store/tests/fragments.rs`) :

- [x] **Index des fragments** : log d'origine, session, bornes en numéros d'échantillon, nom, rôle, date. Le rôle
      est la chaîne des constantes du Guide, pas un nouveau vocabulaire.
- [x] **Période d'un fragment dans un log ouvert** (`range_of`) et son **message clair** quand le fichier a changé
      depuis : c'est le critère d'arrêt du lot, tenu par un test et sa contre-preuve.
- [x] **Liste d'analyse** : ordre, **indice** de couleur attribué à l'entrée et jamais changé ensuite (un indice
      libéré est repris), actif ou non, fragment de travail qui doit appartenir à la liste.
- [x] **Preuve sur un vrai log** : un fragment de `tests/data/jeno5_5s.bbl` relu après redémarrage rend les mêmes
      bornes **et les mêmes échantillons** ; la contre-preuve d'un décalage d'un seul échantillon est dans le test.
      26 tests pour `otb-store`, 8 contre-preuves vues en échec au total.

**Reste — 29b, l'écran** :

- [ ] **Écran d'extraction** (maquette A, hors rail) : ouvert depuis la timeline ou à l'ouverture d'un log,
      poignées de début et de fin, et saisie à l'échantillon près dans le panneau.
- [ ] ⚠️ **À faire avec le lot 30, pas avant** : cet écran est un écran du prototype, et le lot 30 refait toute la
      navigation. Le construire deux fois serait exactement ce que le plan reproche au couple 23 / 30.
- [ ] Couleurs par `rust/otb-gui/src/theme.rs` seulement (L-016), les deux thèmes vérifiés, captures hors écran.

---

## Lot 30 : refonte de la navigation et de la mise en page (Opus)

**But** : appliquer le **prototype complet de l'application** du canevas du 2026-11-16. Cédric l'a désigné le
2026-11-18 comme **la maquette qui fait foi** : les huit autres sont des essais, et en cas de désaccord c'est le
prototype qui gagne.

**Branche** : `feat/navigation-redesign`. **Dépend du lot 29**, et du **lot 23** pour l'espace « Bruit et filtres ».

**Ce que montre le prototype** (relevé dans son code le 2026-11-18) : un **rail de navigation à gauche**, de sept
entrées — Tableau de bord, Guide, Assistant, Bruit et filtres, Réponse indicielle, Curseurs PID, Configuration —
et **deux écrans hors rail**, ouverts par un geste puis refermés : l'extraction d'un fragment et la fiche d'un
drone (Identité, Machine, Configurations Betaflight, Fiche cible). L'espace « Bruit et filtres » porte les onglets
Spectre, Fréq. × gaz, Fréq. × temps, un choix d'axe, et le panneau « Et si » qui remplace le Simulateur. Donc :
**huit outils à plat deviennent sept entrées de rail plus un tableau de bord**, le seul regroupement étant
Spectre + × gaz + × temps + Simulateur → « Bruit et filtres ».

**Décision à écrire dans ce lot**, tranchée le 2026-11-18 (Cédric a demandé de décider) : **D-015 est amendée, pas
remplacée.** Ce qui en reste tel quel : **aucune fenêtre séparée**, et **une vue cachée ne calcule pas** — cette
seconde partie s'étend aux vues internes d'un espace, où elle compte davantage qu'avant, puisqu'un espace
n'affiche qu'une vue sur quatre. Ce qui change : une entrée de navigation n'est plus forcément un outil, c'est
**soit un outil, soit un espace qui regroupe plusieurs vues d'un même sujet** ; les boutons deviennent un rail à
gauche ; et **deux écrans hors rail** sont permis. Ce qui ne change pas : le trait d'outil
(`rust/otb-gui/src/tool.rs`) et la liste d'enregistrement (`rust/otb-gui/src/tools/mod.rs`) restent le point
d'entrée unique — un espace s'y enregistre comme un outil, et chaque vue interne garde son calcul et son cache
(D-037, D-053). Le texte de D-015 cite encore des fichiers Python : l'amendement le remet en termes Rust. D-040
(écarts voulus avec l'ancienne interface Qt) se relit en même temps.

**Fait le 2026-11-19 — 30a, le rail** (`rust/otb-gui/src/app.rs:rail`, `rust/otb-gui/tests/app.rs`) :

- [x] **Amendement de D-015 écrit avant la première ligne de code** : c'est **D-058**, avec les sept entrées du
      rail et les deux écrans hors rail comme contrat. Il a trouvé que le regroupement **existait déjà** dans le
      code (les vues `HIDDEN` au `TAB` de leur espace) : la décision met le texte en accord avec le programme.
- [x] **Le rail** : les boutons de navigation quittent la barre du haut pour une colonne à gauche de 212 points,
      entrées sans cadre sauf celle qui est affichée. La barre du haut ne garde que ce qui agit sur les logs et
      sur la fenêtre, et la page exportée commence désormais au bord droit du rail.
- [x] **Preuve** : deux tests (`le_rail_porte_les_entrees_de_navigation`, `cliquer_une_entree_du_rail_change_decran`)
      avec contre-preuve vue en échec ; 27 tests de la fenêtre verts ; captures hors écran des deux thèmes
      regardées, et les quatre images de la documentation refaites (elles montraient une barre qui n'existe plus).
- [x] **Un défaut révélé, corrigé dans la foulée** : le `ui.horizontal` imbriqué de `phase_button` formait un bloc
      insécable, et la dernière phase du Guide se faisait **rogner** au lieu de passer à la ligne dès que la
      fenêtre se resserrait. Mis à plat, elle passe à la ligne (capture à l'appui).

**Reste** :

- [ ] **Tableau de bord** : cartes de drones, timeline des fragments à droite (le plus récent en haut), « + » pour
      ajouter un fragment à la liste d'analyse.
- [ ] **Espace « Bruit et filtres »** : Gyro / Terme D, les trois vues en onglets, panneau de filtres modifiable
      qui remplace le Simulateur. **Faire le lot 23 d'abord** : sinon le même code est déplacé deux fois.
- [ ] **Assistant en cartes** triées par gravité, mesure contre seuil en jauge, constats « OK » repliés. ⚠️ Un
      constat se reconnaît par sa clé, jamais par son titre (D-028).
- [ ] **Preuve** : captures hors écran avant / après sur les deux thèmes, et le temps de cadre mesuré
      (le budget de 60 images par seconde est tenu avec une marge de 3 aujourd'hui : il ne doit pas tomber sous 1).
- [ ] **Critère d'arrêt** : une page qui perd son test, ou le budget d'images par seconde dépassé.

---

## Lot 31 : mesures suivies dans le temps (Opus)

**But** : répondre à « est-ce que ce drone s'améliore ? », ce qu'aucun écran ne sait faire aujourd'hui.

**Branche** : `feat/metrics-history`. **Dépend du lot 29** ; la table des mesures existe déjà (`otb-store`).

- [ ] **Une ligne par fragment, axe et mesure** : latence, pic, temps de montée, marges, bruit du gyro et du
      terme D, retard des filtres — avec les réglages de calcul qui l'ont produite et la version de calcul du
      lot 28. Une mesure se jette quand les bornes du fragment ou cette version changent.
- [ ] ⚠️ **La table des mesures ne se charge pas au lancement, mais à la première question posée.** Mesuré au lot 28
      ([validation.md](validation.md)) : au volume plein (535 logs, 77 040 mesures) elle pèse **8 Mo** et met **959 ms**
      à se lire — à elle seule le déclencheur du plan. Les cinq autres tables réunies pèsent 1 Mo.
- [ ] **Fiche cible par drone** (réactivité des PID, filtrage) et un graphique par mesure sur tous ses fragments,
      la cible en bande.
- [ ] ⚠️ **Dire lequel prime** : la cible du drone n'est pas un seuil de l'assistant (D-011, calibré sur le
      corpus). Deux chiffres qui se contredisent à l'écran, c'est un utilisateur perdu.
- [ ] **Critère d'arrêt** : une mesure stockée qui ne se recalcule pas à l'identique à la demande.

---

## Lot 32 : récupération des logs par USB (Opus) — **le plus risqué**

**But** : brancher le drone, récupérer, vérifier, puis seulement effacer. Un log perdu est un vol à refaire ; un
effacement reporté ne coûte rien.

**Branche** : `feat/usb-retrieve`. **Dépend du lot 30** ; la bibliothèque et le refus des doublons existent déjà.

- [ ] **Identité de la machine** par MSP : nom, identifiant de la carte, version du firmware, état armé,
      occupation de la mémoire. L'identifiant de la carte est la clé ; le nom seul ne suffit pas (deux drones
      peuvent le porter, et il se change en une commande).
- [ ] **Deux voies de copie** : mode stockage de masse quand la carte le permet (rapide, seule voie pour une carte
      SD), repli sur la lecture bloc par bloc en MSP (lent : plusieurs minutes pour 16 Mo). Barre de progression,
      et une copie interrompue reprend au début.
- [ ] ⚠️ **Jamais d'effacement sans vérification**, dans cet ordre : taille copiée = place annoncée, le décodeur
      ouvre chaque log et en extrait au moins un fragment, les fichiers sont écrits sur le disque, **puis**
      l'utilisateur confirme. Un échec de lecture interdit l'effacement, quelle que soit l'option choisie.
- [ ] ⚠️ **Sécurité** : drone armé → aucune opération ; conseil de retirer les hélices ; un seul accès au port à la
      fois (le configurateur de Betaflight peut l'occuper) ; droits sur le port série sous Linux et pilote du port
      virtuel sous Windows, détectés et expliqués au lieu d'échouer.
- [ ] **Sauvegarde de la configuration** au même branchement (`diff all`), gardée seulement si elle diffère de la
      précédente : l'écran Configuration peut alors comparer deux versions des réglages, et plus seulement les
      en-têtes de deux logs.
- [ ] **Les huit points à vérifier** de l'étude, dans le code et la documentation de Betaflight, **avant** d'écrire
      (elle a été rédigée de mémoire) : versions où le mode stockage de masse lit la mémoire interne, débit réel de
      la lecture MSP, durée d'un effacement de 16 Mo et comment savoir qu'il est fini, format et stabilité de
      l'identifiant de carte, effacement sur carte SD, bibliothèque série côté Rust, autres firmwares qui parlent
      MSP, et faut-il appliquer les réglages « et si » directement sur la carte.
- [ ] ⚠️ **Nouvelle dépendance et nouvelle surface** : licence vérifiée par `cargo deny` (`rust/deny.toml`), et
      **aucun matériel en CI** — donc un port simulé pour les tests automatiques et une procédure d'essai à la main
      écrite dans la documentation, comme pour les installateurs.
- [ ] **Critère d'arrêt** : un seul chemin de code où l'effacement pourrait partir sans vérification complète →
      s'arrêter net.

## Hors plan, volontairement

Ces travaux reportés sont **vus et laissés** : aucun n'empêche un pilote d'utiliser OpenTuneBox aujourd'hui.
Leur détail et leur condition de reprise sont dans [travaux-reportes.md](travaux-reportes.md) ; ne pas les
rouvrir ici sans cette condition. Deux d'entre eux (W-043 et W-044) viennent du lot 13 supprimé : ils n'ont
jamais dépendu de la 0.8.0, et gardent donc leur condition de reprise. **W-041 a quitté ce tableau le
2026-11-10** : sa condition de reprise — un échec au hasard de la CI sur ce test — a été atteinte, et il est
clos (L-095). C'est à cela que sert une condition de reprise écrite d'avance. **W-043 l'a quitté le 2026-11-12** : il
a été traité par le lot 21.

| ID | Pourquoi il n'est pas dans un lot |
|---|---|
| W-001 | Logs KISS (`.TXT`) et INAV non décodés : **périmètre tranché le 2026-11-18, Betaflight seul** — reste fermé, à rouvrir si Cédric change de firmware ou si de vrais logs INAV arrivent (lot 26 sorti du plan) |
| W-057 | ArduCopter (logs DataFlash `.bin`) non pris en charge : même arbitrage du 2026-11-18 ; reprise = des logs ArduCopter réels, dont un avec `INS_RAW_LOG_OPT` (lot 27 sorti du plan) |
| W-002 | Profils whoop, 2–3,5" et 8"+ appuyés sur trop peu de drones : il faut **plus de logs**, pas plus de calcul |
| W-004 | La boucle ouverte ne se mesure que jusqu'à 20–40 Hz : limite physique de l'énergie du manche |
| W-022 | Tolérance par fonction et non par champ dans `otb-parity` : reprise si un écart est soupçonné sur `freqs` / `bins` / `resolution_hz` |
| W-028 | Écarts des références figées : se règle au **prochain réenregistrement**, pas avant |
| W-029 (4) | Branche « crash » du guide : aucun crash réel dans le corpus, seuls des cas fabriqués |
| W-042 | Avis RustSec hérités d'eframe : ⏸ attend une version d'eframe qui change ses dépendances |
| W-044 (2) | `construire_windows.ps1` jamais essayé sur un vrai Windows : la CI fabrique l'installateur livré, ce script ne sert qu'à construire chez soi — à l'occasion. Son point 1 (`_internal`) est **clos** |
| W-051 | Pistes de performance écartées **par la mesure** le 2026-11-04 : ne pas les rouvrir sans mesure nouvelle |
| W-053 | 45 messages d'erreur imitant les exceptions de Python : atteignabilité non établie |
| W-054 | Huit des dix comparateurs d'`otb-parity` sans test : fabriquer une référence Python pour ces couches est le vrai coût |

**Écarté par arbitrage de Cédric** (« rien sans bénéfice démontrable ») : découper `rules.rs` (3 631 lignes),
`log_check.rs` (1 356) et `guide_page.rs` (1 053) — testés et validés au bit près, les remanier est un risque
sans effet visible ; les ~300 `pub` inutilement publiques (exposition trop large, pas du code mort) ; un cache
Cargo en CI (une mécanique à entretenir pour une CI qui dure 7 min).

---

## Historique des lots

Une ligne par lot fait — ou par lot abandonné, pour que son numéro ne serve jamais à autre chose. Le détail est dans les demandes de fusion et les pages Releases (jusqu'au lot 20 : l'ancien journal,
`git show 2cd2416:CHANGELOG.md`), les mesures dans [validation.md](validation.md),
les pièges dans [lecons.md](lecons.md) et les choix dans [decisions.md](decisions.md) — ce tableau ne fait
que dire quand et par quoi chaque lot est prouvé.

Les lots 0 à 6 viennent de l'**audit du dépôt du 2026-11-04**, qui avait relevé cinq chantiers : décodeur
robuste face à un en-tête hostile (C1), résultat périmé affiché par le socle des outils (C2), commande unique
de vérification (C3), mémoire du décodeur (C4), consignes et CI périmées (C5). **Les cinq sont traités et
prouvés.**

| Lot | Fait le | Sujet et preuve | Ce qu'il en reste |
|---|---|---|---|
| 0 | 2026-11-04 | Sonde du corpus et des références (`chore/corpus-probe`) : `otb-parity` 0 écart, `corpus validate` 697/712, départ mémoire 491 Mo. La ligne de base n'était **pas** « 0 écart » comme le plan le supposait | rien |
| 1 | 2026-11-04 | **C3** — `otb-dev verify [--quick] [--strict]` (`feat/dev-verify`, L-087), marqueur `[OTB-SAUT]`, crochet sans `jq`. 4 contre-preuves ; 74 exécutables de test avec et sans la liste des `-p` (elle était inutile) | rien |
| 2 | 2026-11-04 | **C5** — consignes et CI remises à jour (PR #103) : `AGENTS.md` de 12,6 à 8,4 Ko | rien |
| 3 | 2026-11-04 | **C2** — résultat périmé du socle des outils (PR #104, L-088). Contre-preuve : l'ancien `update` rougit les deux tests | W-036 (8) : garde-fou propre au Simulateur gardé |
| 4 | 2026-11-04 | **C1** — décodeur robuste (`fix/decoder-hostile-header`, D-044, L-080) : bornes 512 champs / intervalles 1 à 65 535, `try_reserve`. 10 cas dont **6 rouges avant** ; 20 000 champs : 3,7 s puis abandon → refus instantané | limite écrite : sous Linux une réservation énorme peut réussir (surengagement) |
| 5 | 2026-11-04 | **C4** — mémoire du décodeur (`perf/decoder-memory`, L-081) : `CompactTable` en `i32` dès le décodeur, **481 → 274 Mo (÷ 1,75)** et 0,63 → 0,54 s ; `FlightLog` identiques sur 528 fichiers | un décodage par blocs vers `f32` irait plus loin (non fait, non chiffré) ; W-014 et W-016 clos |
| 6 | 2026-11-06 | Bilan des cinq chantiers (branche « docs/plan-suite ») : L-087 et L-088 écrites, W-014 et W-016 clos. Aucun code livré | rien |
| 7 | 2026-11-08 | **Première publication Rust, 0.9.0.** 7a `otb-dev licenses --check` (`feat/licenses-check`) ; 7b CHANGELOG et captures (`chore/release-0.9.0`) ; 7c **quatre tours** d'essai sur de vraies machines et **sept défauts** corrigés (PR #118 à #126), **aucun vu par les tests** ; 7d–7e publication, page « dernière version », brouillon d'essai supprimé | W-043 (licences non relues, échéance repoussée à la 1.0) et W-044 (mise à jour depuis la 0.8.0 jamais essayée) → passés au lot 13, puis « Hors plan » quand il a été supprimé (ligne 13) |
| 8 | 2026-11-05 | Banc de temps de cadre (`chore/viewer-frame-bench`) : `examples/banc_cadre.rs`. Étalon retrouvé sur `FINAL.bbl` (7,4 à 8,3 ms) ; `LOG00008.BFL` 12,6 à 13,0 ms à 620 079 échantillons, sous les 16,7 ms | les 0,34 ms de carte graphique de W-049 ne se reproduisent pas (L-084) |
| 9 | — | **Non retenu** : W-048 ne devait pas se régler en affaiblissant un test. Repris hors lot le 2026-11-05 (`test/golden-tolerance`, D-047, L-083) | W-048 clos |
| 10 | 2026-11-05 | Tracé du viewer sans parcourir toute la série (`perf/viewer-pyramid`, D-048, L-085, L-086) : index multi-résolution min/max dans `otb-dsp`, cadre **÷ 2,2 à ÷ 6,5**, tracé **identique au bit près**, pic de mémoire inchangé | W-049 clos ; **W-052 ouvert** (le tracé domine maintenant) → lot 15 |
| 11 | 2026-11-10 | **Ouverture de plusieurs logs en parallèle** (branche `perf/open-parallel`, D-051, L-093) : `otb_log::parallel`, paquets d'au plus 3 fichiers décodés en même temps dans un pool `rayon` propre à l'appel. 6 logs : **1,835 s → 0,811 s (× 2,26)**, un seul fichier non dégradé, `FlightLog` identiques (empreintes) ; 4 contre-preuves | **W-050 clos** ; le chiffre de mémoire de D-051 était trop bas (663 Mo et non ≈ 540), plafond gardé par Cédric |
| 12 | 2026-11-08 | Ménage des registres (PR #129, `chore/registers-cleanup`) : 10 travaux reportés clos ou requalifiés, une **panique** corrigée (`log_rate`, W-029 point 3), un paramètre mort retiré (W-024), D-051 écrite. `verify --strict` 0 saut (125 s) | W-053 ouvert au passage |
| 13 | — | **Supprimé le 2026-11-10** : « nettoyage d'après publication ». Cédric a indiqué que **personne n'a jamais utilisé la 0.8.0** — l'essai de mise à jour 0.8.0 → 0.9.0 et son filet dans l'installateur devenaient sans objet. Le filet (`[InstallDelete]` de `opentunebox.iss`) est retiré dans la foulée (branche `chore/drop-python-upgrade-net`), W-044 point 1 clos | W-043 (licences, échéance 1.0) et W-044 point 2 (script Windows) ne dépendaient pas de la 0.8.0 → « Hors plan », avec leur condition |
| 14 | 2026-11-10 | **Derniers seuils sans origine** (branche docs/source-last-thresholds) : **aucune valeur changée**, parce qu'aucun des deux seuils restants ne peut être calibré — le corpus n'emprunte **jamais** la détection de taille par le nom (0 drone sur 96 sans fréquence moteur) et le profil 8"+ n'a que 6 drones (W-002). Chacun porte désormais sa **condition de levée**. W-005 clos : formule de la bande lue à la page sur une reproduction ([S37]), avec ses trois limites écrites. `verify --strict` 0 saut (126 s) | W-055 ouvert (cinq sources nomment une personne : arbitrage D-050) ; W-003 reste ouvert, mais borné à deux lignes assumées |
| 16 | 2026-11-08 | **Revue intégrale** du code, des tests, de la documentation et de l'outillage (PR #129) : 4 relevés (~41 000 lignes de code, ~20 500 de tests, 30 `.md`, 4 workflows), 53 cases, **2 défauts réels** corrigés (`compare_header`, `slice_time`), **6 constats de relevé faux** corrigés après vérification, chaîne Rust figée (`rust-toolchain.toml`), clippy sous Windows, D-052 écrite, D-050 précisée, récidive de L-079 bridée | **W-054 ouvert** : seuls 2 des 10 comparateurs d'`otb-parity` ont reçu un test |
| 17 | 2026-11-08 | **Rassemblement des plans** (branche « docs/rassembler-les-plans ») : les quatre documents de plan deviennent ce seul `docs/plan.md`, L-092 écrite | rien |
| 18 | 2026-11-11 | **Audit de sécurité** (branche `chore/security-hardening`, D-054) : réglages GitHub relus et complétés par Cédric (versions immuables, CodeQL, actions autorisées, approbation des contributions extérieures) ; `claude.yml` refuse les forks ; `--locked` en CI ; `unsafe_code = "deny"` ; crochet `guard-git.sh` ; essai de mutations (100 000 cas pour le décodeur, 20 000 de bout en bout, 0 plantage). Contre-épreuves notées dans D-054 | signature de l'installateur Windows : ouverte dans D-054 (l'épinglage SHA est exigé depuis le 2026-11-11) |
| 19 | 2026-11-11 | **Relecture des workflows en CI** (branche `chore/lint-workflows`, D-055) : job `actionlint` 1.7.12 dans `tests.yml`, vérifié par empreinte ; rien à installer sur le poste. Avant l'envoi : 19 scripts bash passés à `shellcheck`, rien à signaler | rien : `actionlint` est un contrôle exigé de `protection-main` depuis le 2026-11-11 |
| 20 | 2026-11-11 | **Notes de version à la manière de Betaflight** (PR #147, D-056) : titre de PR en phrase et une étiquette `RN: …` (8 étiquettes créées, dont `dependencies`, absente jusque-là : L-096) ; `.github/release.yml` ; contrôle `etiquette-pr.yml` (rouge sans étiquette, vert avec, vu sur #147) ; `otb-dev release notes` traduit la liste de GitHub ; `CHANGELOG.md` supprimé. 86 PR étiquetées et retitrées, Dependabot excepté (#60 à #147). Page de la 0.9.0 refaite (50 Ko → 6 Ko) depuis une liste fabriquée en local, GitHub ne lisant `release.yml` que sur l'étiquette (L-097) | `etiquette` exigé par `protection-main` depuis le 2026-11-12 ; l'appel de `generate-notes` par le jeton de la CI sera éprouvé à la première étiquette `essai-v…` ou `v…` |
| 21 | 2026-11-12 | **Licences livrées et sources sans nom** (branche « docs/licenses-and-sources-review », D-050 précisée) : textes des 22 composants sans fichier de licence copiés de leur dépôt d'origine (`rust/otb-dev/upstream-licenses/`, table `UPSTREAM`), dont la licence BSD du code d'AccessKit issu de Chromium, **absente du fichier livré avec la 0.9.0** ; polices relues (Noto Emoji, seule police OFL, non compilée) ; partie 3 du fichier vide et refusée par un test, contre-preuve vue en échec. Noms de personnes retirés de 13 sources (5 relevées, 8 trouvées en relisant), pseudonymes compris | W-043 et W-055 clos ; reste, avant la 0.10.0 : rien de ce lot |

| 28 | 2026-11-18 | **Bibliothèque de données** (branche `feat/data-library`) : crate `otb-store`, six tables en **fichiers JSON** dans `bibliotheque/` — drones, logs connus, fragments, `diff all` datés, mesures avec leur version de calcul, liste d'analyse. Un seul module d'accès ; un fichier illisible ou plus récent est mis de côté, jamais écrasé. 15 tests, **5 contre-preuves vues en échec** (mise de côté, version plus récente, péremption des mesures, doublon de log, réglages identiques). Volumes mesurés dans [validation.md](validation.md) | la table des mesures pèse 8 Mo et met 959 ms à se lire au volume plein : **à charger à la demande au lot 31** (noté dans ce lot) |
| 22 | 2026-11-18 | **Cadre des scripts firmware** (branche « docs/firmware-scripts-frame », **D-057**) : les quatre arbitrages de Cédric écrits en décision, le contrat des sept fonctions, la description de chaîne et les plages de versions ajoutés à [spec-betaflight.md](spec-betaflight.md) §8, résumé dans `AGENTS.md`. **Aucun code livré** | **W-058 ouvert** : la simulation temporelle garde une seule cadence et dérive avant de filtrer — limite assumée, invisible dans l'application (vérifié le 2026-11-18, après une première lecture trop rapide) |
| 26 | — | **Sorti du plan le 2026-11-18** : INAV. Périmètre tranché par Cédric — Betaflight seul tant qu'il n'y a pas de vrais logs INAV. Rien n'était commencé | W-001 reste fermé, avec sa condition de reprise |
| 27 | — | **Sorti du plan le 2026-11-18** : ArduCopter, même arbitrage. Son coût propre (lecteur DataFlash, alignement temporel) et son intérêt unique (seul firmware qui logge l'entrée des filtres à pleine cadence) sont consignés | **W-057 ouvert**, condition de reprise écrite |

La migration Python → Rust elle-même, terminée le 2026-11-04, garde son propre document :
[migration-rust.md](migration-rust.md) (phases, preuves d'équivalence, verdict du prototype d'interface).
