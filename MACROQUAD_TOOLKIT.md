# Macroquad Toolkit

A collection of common utilities for Macroquad game development, extracted from multiple games to reduce duplication and provide consistent patterns.

## Features

- **Input utilities**: Mouse hovering, clicking, rectangle collision detection
- **UI rendering**: Buttons (with press/release variants), panels, progress bars
- **Asset management**: Texture loading and caching
- **Camera2D**: Pan and zoom for 2D games
- **Audio**: `SoundManager` keyed SFX playback with volume scaling and asset packs
- **Event bus**: Generic event system for decoupled game logic
- **Color palettes**: Consistent dark theme colors
- **Sprite system**: Builder pattern for texture rendering with transformations
- **Screenshot capture**: Env-var-driven headless capture harness for visual verification
- **Color helpers**: `with_alpha`, `lighten`/`darken`, `mix`, HSV conversion, hue shift
- **Math**: lerp/smoothstep/approach, easing curves, time-based pulse, `Tween`
- **Timing**: `Cooldown`, `Timer`, `IntervalTimer`, `Timeline` phase sequencer
- **FX**: trauma screen shake, screen fades, pooled particles, floating text
- **Typewriter / streaming text**: per-char reveal, shared-budget `BlockReveal`
- **Number formatting**: money, compact/idle-magnitude amounts and rates, clocks/timers
- **Form widgets**: toggle, checkbox, slider, stepper, segmented bar, keycap
- **Scroll & tabs**: `ScrollArea` with drawn scrollbar, tab bars / nav columns
- **Settings**: shared `GameSettings` (volume groups, fullscreen, UI scale) with persistence
- **Achievements**: unlock registry that serializes into saves
- **Debug overlay**: toggleable smoothed FPS/frame-time panel
- **Raster**: CPU pixel drawing onto `Image` (primitives, Bresenham, seeded noise)
- **Sprite variation**: seeded HSV-region recolor with per-(id, seed) texture cache
- **Projectiles**: `ProjectileLayer` travel-lerp visuals delivering payloads on arrival
- **Hover tooltip**: delayed, fading tooltip state (`HoverTooltip`)
- **Plaques**: ornamented title/menu buttons with corner marks and style hooks
- **Menu cursor**: wrap-around keyboard selection for pause/settings menus

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
macroquad-toolkit = { path = "../macroquad-toolkit" }
```

### Quick Start

```rust
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    let mut assets = AssetManager::new();
    assets.load_texture("player", "assets/player.png").await.ok();

    loop {
        clear_background(dark::BACKGROUND);

        // Draw a button
        if button(10.0, 10.0, 100.0, 40.0, "Click Me") {
            println!("Button clicked!");
        }

        next_frame().await;
    }
}
```

## Modules

### Input (`input` module)

```rust
use macroquad_toolkit::input::*;

// Check if mouse is over a rectangle
if is_hovered(x, y, w, h) {
    // ...
}

// Check if rectangle was clicked (released)
if was_clicked(x, y, w, h) {
    // ...
}

// Check if rectangle was pressed (down)
if was_pressed(x, y, w, h) {
    // ...
}

// Capture input state
let input = InputState::capture();
if input.left_click {
    // ...
}

// Keyboard-driven menus: wrap-around cursor + standard key readers
let mut cursor = MenuCursor::new(options.len());   // keep in state
cursor.navigate(menu_nav_vertical());              // Up/W, Down/S
volume += menu_nav_horizontal() as f32 * 0.1;      // Left/A, Right/D
if input.enter_pressed { activate(options[cursor.index()]); }
```

### UI (`ui` module)

```rust
use macroquad_toolkit::ui::*;

// Simple button (triggers on release)
if button(x, y, w, h, "Click") {
    // Button was clicked
}

// Button with custom style
let style = ButtonStyle::default_dark();
if button_styled(x, y, w, h, "Custom", &style) {
    // ...
}

// Button that triggers on press (instead of release)
if button_on_press(x, y, w, h, "Press", &style) {
    // Triggers immediately when mouse down
}

// Panel with title
panel(x, y, w, h, Some("Title"));

// Progress bar
progress_bar(x, y, w, h, current, max, dark::POSITIVE);
```

### Assets (`assets` module)

```rust
use macroquad_toolkit::assets::AssetManager;

let mut assets = AssetManager::new();

// Load single texture
assets.load_texture("player", "assets/player.png").await.ok();

// Get texture
if let Some(tex) = assets.get_texture("player") {
    draw_texture(tex, x, y, WHITE);
}
```

### Camera (`camera` module)

```rust
use macroquad_toolkit::camera::Camera2D;

let mut camera = Camera2D::new(vec2(0.0, 0.0), 1.0);

// In game loop
camera.update(get_frame_time(), false);

// Convert coordinates
let world_pos = camera.screen_to_world(mouse_position().into());
let screen_pos = camera.world_to_screen(world_pos);
```

### Audio (`audio` module)

`SoundManager<T>` plays sound effects keyed by a game-defined id enum, scaling
each play by a volume multiplier (wire it to `GameSettings::effective_sfx_volume()`).
Sounds are registered directly or pulled from an `AssetPack`.

```rust
use macroquad_toolkit::audio::{SoundManager, SoundId};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Sfx { Hit, Coin }
impl SoundId for Sfx {}

let mut sounds: SoundManager<Sfx> = SoundManager::new();
sounds.add_asset_pack(pack);              // or register sounds individually
sounds.play_sfx(Sfx::Hit, settings.effective_sfx_volume());
```

### Events (`events` module)

```rust
use macroquad_toolkit::events::EventBus;

enum GameEvent {
    PlayerDied,
    EnemySpawned,
}

let mut events = EventBus::new();
events.push(GameEvent::PlayerDied);

// Process events
for event in events.drain() {
    match event {
        GameEvent::PlayerDied => { /* ... */ }
        GameEvent::EnemySpawned => { /* ... */ }
    }
}
```

### Colors (`colors` module)

```rust
use macroquad_toolkit::colors::dark;

clear_background(dark::BACKGROUND);
draw_rectangle(x, y, w, h, dark::PANEL);
draw_text("Hello", x, y, 20.0, dark::TEXT);
```

Available colors:
- `BACKGROUND`, `PANEL`, `PANEL_HEADER`
- `TEXT`, `TEXT_BRIGHT`, `TEXT_DIM`
- `ACCENT`, `POSITIVE`, `WARNING`, `NEGATIVE`
- `HOVERED`

Color manipulation helpers — do **not** hand-roll `Color::new(c.r, c.g, c.b, a)`
or per-channel brighten/darken/mix in game code:

```rust
use macroquad_toolkit::colors::{with_alpha, multiply_alpha, lighten, darken, shade, tint,
                                mix, shift_hue};

let faded = with_alpha(dark::ACCENT, 0.4);      // replace alpha
let ghost = multiply_alpha(translucent, 0.5);   // scale existing alpha
let hover = lighten(base, 0.1);                 // additive per-channel
let dim = darken(base, 0.15);
let shadowed = shade(base, 0.3);                // blend toward black (multiplicative)
let pale = tint(base, 0.3);                     // blend toward white
let blend = mix(a, b, t);                       // component lerp (alias: lerp_color)
let variant = shift_hue(base, 40.0);            // HSV hue rotation
```

### Math (`math` module)

Interpolation and easing primitives — use these instead of private `lerp`/`ease_*` copies:

```rust
use macroquad_toolkit::math::{lerp, inv_lerp, remap, smoothstep, approach, exp_approach,
                              ease_out_cubic, ease_out_back, pulse01, pulse_range, bob,
                              hash_str, Tween};

let x = lerp(a, b, t);
let glow = pulse_range(3.0, 0.55, 0.77);   // replaces (get_time()*k).sin() idioms
let mut slide = Tween::new(0.0, 8.0);      // exponential ease-toward-target
slide.set_target(panel_x);
slide.update(dt);
let seed = hash_str(&entity_id);           // FNV-1a, stable procedural seeds
```

### Timing (`timing` module)

Replaces bare `f32` cooldown fields, `accum` tickers, and hand-stepped phase machines:

```rust
use macroquad_toolkit::timing::{Cooldown, Timer, IntervalTimer, Timeline};

let mut fire = Cooldown::new(0.5);
if wants_fire && fire.try_trigger() { /* shoot */ }
fire.tick(dt);

let mut flash = Timer::new(0.3);            // one-shot with 0..1 progress
let mut spawner = IntervalTimer::new(2.0);  // fires N times per update
for _ in 0..spawner.tick(dt) { /* spawn */ }

let mut swing = Timeline::new(vec![(Phase::WindUp, 0.2), (Phase::Strike, 0.1)]);
swing.advance(dt);
if let Some((phase, progress)) = swing.current() { /* animate */ }
```

### FX (`fx` module)

```rust
use macroquad_toolkit::fx::{ScreenShake, ScreenFade, ParticleSystem, BurstConfig, FloatingTextLayer};

let mut shake = ScreenShake::new(12.0);     // trauma model: offset ~ trauma^2
shake.add_trauma(0.4);
shake.update(dt);
let cam_offset = shake.offset();

let mut fade = ScreenFade::new(0.4);        // scene transitions
fade.fade_out();
if fade.update(dt) { /* swap scene, then fade.fade_in() */ }
fade.draw();

let mut particles = ParticleSystem::new();  // capped pool
particles.spawn_burst(hit_pos, 12, &BurstConfig::default());
particles.update(dt);
particles.draw();

let mut floaters = FloatingTextLayer::new(); // damage numbers / gains
floaters.spawn("+5", world_pos, GOLD);
floaters.update(dt);
floaters.draw();                             // inside camera for world anchor
```

Attack visuals that lerp toward a target and resolve a hit on arrival use
`ProjectileLayer<T>` — the payload is whatever your game needs to apply the
impact (serde round-trips for saves):

```rust
use macroquad_toolkit::fx::{ProjectileLayer, TravelingProjectile};

let mut projectiles: ProjectileLayer<Impact> = ProjectileLayer::new();
projectiles.spawn(attacker_pos, target_pos, 0.3, Impact { defender, damage });
projectiles.push(                                     // melee slash: stays near attacker
    TravelingProjectile::new(a, b, 0.15, impact).with_travel_ratio(0.3),
);
for impact in projectiles.update(dt) { apply_damage(impact); }
for p in projectiles.iter() { draw_at(p.position(), p.progress()); }
```

Typewriter / streaming-text helpers reveal text a character at a time (dialogue,
terminal boot logs, killfeeds). The single-string helpers reveal each string
independently; `BlockReveal` streams a block of lines against one shared
character budget so line `N` only starts once lines `0..N` are fully shown.

```rust
use macroquad_toolkit::fx::{typed_prefix, is_fully_typed, prefix_chars, reveal_block};

// One string, char-by-char:
let shown = typed_prefix(line, elapsed, 30.0);   // valid &str slice
draw_ui_text(shown, x, y, 20.0, dark::TEXT);
if is_fully_typed(line, elapsed, 30.0) { /* advance */ }

// A block of lines as one continuous stream:
let reveal = reveal_block(&lines, elapsed, 40.0);
for (i, line) in lines.iter().enumerate() {
    draw_ui_text(prefix_chars(line, reveal.shown[i]), x, y + i as f32 * 18.0, 16.0, dark::TEXT);
}
// reveal.cursor_line / reveal.complete drive a blinking write-cursor.
```

### Form widgets, scroll, and tabs (`ui` module)

```rust
use macroquad_toolkit::ui::{toggle_row, checkbox, slider_row, stepper_row,
                            segmented_bar, keycap_hint, ScrollArea, tab_bar, nav_column};

toggle_row(row, "Screen shake", &mut settings.screen_shake);
slider_row(row2, "Music", &mut settings.music_volume, 0.0, 1.0);
match stepper_row(row3, "UI scale", &format!("{:.0}%", scale * 100.0)) {
    d if d != 0 => { /* apply step */ }
    _ => {}
}

let mut scroll = ScrollArea::new();          // keep in state
scroll.update(list_rect, content_height);
// draw rows offset by -scroll.offset(), then:
scroll.draw_scrollbar(list_rect, content_height);

if let Some(clicked) = tab_bar(bar_rect, &["Stats", "Gear", "Log"], active_tab) {
    active_tab = clicked;
}
```

### Hover tooltips and menu plaques (`ui` module)

`HoverTooltip` adds a show delay and fade-in/out on top of the stateless
`draw_tooltip`: widgets report hover during drawing, and the owner draws once
at the end of the frame so the tooltip renders above everything.

```rust
use macroquad_toolkit::ui::{HoverTooltip, TooltipStyle};

let mut tooltip = HoverTooltip::new();          // keep in UI state
// While drawing each widget:
tooltip.hover_rect("forge-btn", "Forge a new blade", btn_rect, mouse, get_time());
// End of frame (applies fade alpha to the style):
tooltip.draw(&TooltipStyle::default(), None, get_time());
```

Plaques are the ornamented title/pause-menu buttons (drop shadow, framed
face, corner tick marks). `PlaqueStyle` exposes hooks for the parts games
differ on — bevel band, inner borders, top highlight, mark placement — and
`PlaquePalette` maps interaction state to face colors, one palette per tone:

```rust
use macroquad_toolkit::ui::{plaque_button, plaque_button_ex, draw_plaque,
                            draw_corner_marks, PlaquePalette, PlaqueStyle};

let style = PlaqueStyle::default();             // build once per screen/tone
let palette = PlaquePalette::default();
if plaque_button(rect, "New Game", &style, &palette, enabled, logical_mouse) {
    // activated (released over the button)
}
// Keyboard menus: highlight the MenuCursor's row
plaque_button_ex(rect, "Resume", &style, &palette, true, cursor.index() == i, mouse);
draw_corner_marks(panel_rect, gold);            // standalone decorations
```

### Settings (`settings` module)

```rust
use macroquad_toolkit::settings::GameSettings;

let mut settings = GameSettings::load("my_game");   // defaults when missing
settings.apply_display();                            // fullscreen + UI text scale
sound.play_sfx(Sfx::Hit, settings.effective_sfx_volume());
settings.save("my_game").ok();
```

### Achievements (`achievements` module)

```rust
use macroquad_toolkit::achievements::{Achievement, Achievements};

let mut achievements = Achievements::from_definitions(vec![
    Achievement::new("first_win", "First Win", "Win a run."),
]);
if achievements.unlock("first_win") { notifications.success("Achievement: First Win"); }
let (done, total) = achievements.progress();
// Serialize into the save; call sync_definitions(defs) after load.
```

### Debug overlay (`debug` module)

```rust
use macroquad_toolkit::debug::DebugOverlay;

let mut overlay = DebugOverlay::new();      // keep in Game
overlay.record_frame(get_frame_time());     // every frame
if is_key_pressed(KeyCode::F3) { overlay.toggle(); }
overlay.draw(&[format!("entities: {}", count)]);
```

Number, time, and string formatting live in `ui` — use these instead of
hand-rolled `format!` idioms:

```rust
use macroquad_toolkit::ui::{format_money, format_compact_money, format_amount,
                            format_rate, format_mmss, format_hmmss, format_clock};

format_money(1_234)        // "$1,234"
format_compact_money(2_500) // compact i64 currency (saturates ~9.2e18)
format_amount(1_500.0)     // "1.50K" — idle/incremental magnitudes to 1e30, then sci notation
format_rate(12_500.0)      // "12.50K" per-second rate with the same suffixes
format_mmss(462.0)         // "07:42" (minutes grow past an hour)
format_hmmss(secs)         // "H:MM:SS" once an hour is reached
format_clock(8, 30)        // "08:30" in-game clock, hours wrap at 24
```

### RNG (`rng` module)

Two layers: convenience wrappers around `macroquad::rand` (WebGL-safe, shared
generator) and `SeededRng`, a deterministic xorshift64* generator for
gameplay. Prefer `SeededRng` for anything that affects simulation outcomes —
keep it state-owned so runs are reproducible (`CODE_STANDARDS.md`), and use
the shared-generator helpers only for cosmetic randomness. Do **not** write a
project-local RNG; games that need one re-export this
(e.g. `pub use macroquad_toolkit::rng::SeededRng as Rng;`).

```rust
use macroquad_toolkit::rng::{self, SeededRng};

// Deterministic, state-owned gameplay RNG. Serde-serializable so mid-run
// state can live in saves.
let mut rng = SeededRng::new(world_seed);
let roll = rng.next_u64();          // raw 64 bits
let t = rng.next_f32();             // [0, 1)
let speed = rng.range_f32(0.5, 2.0); // [low, high)
let index = rng.below(items.len()); // [0, n); 0 when n == 0
if rng.chance(0.25) { /* 25% */ }
let picked = rng.choose(&items);    // Option<&T>

// Shared-generator helpers (cosmetic randomness only).
rng::srand(seed);
let v = rng::gen_range(0, 10);
let flip = rng::chance(0.5);
rng::shuffle(&mut deck);
let one = rng::choose(&palette);
```

### Sprite (`sprite` module)

```rust
use macroquad_toolkit::sprite::Sprite;

let sprite = Sprite::new()
    .with_texture(texture)
    .at(100.0, 100.0)
    .scaled(2.0, 2.0)
    .rotated(0.5)
    .colored(RED);

sprite.draw();
```

Per-entity visual diversity from one image: `SpriteVariationCache` recolors
hue/saturation regions of a base sprite deterministically per seed and caches
the resulting textures.

```rust
use macroquad_toolkit::sprite::{ColorRegion, SpriteVariationCache, SpriteVariationConfig};

let mut variations = SpriteVariationCache::new();   // keep in render state
variations.register("goblin", SpriteVariationConfig {
    color_regions: vec![
        ColorRegion::new("skin", 60.0, 150.0, 0.3, 0.8, 0.8),  // green hues
        ColorRegion::new("cloth", 0.0, 360.0, 0.3, 1.0, 1.5),
    ],
    variation_strength: 0.7,
});
let texture = variations.get_or_create("goblin", entity.variation_seed, &base_texture);
```

### Raster (`raster` module)

CPU-side pixel drawing onto an `Image` for procedural art (sprites,
portraits, generated textures). Everything clips against the image bounds;
noise is seeded and deterministic.

```rust
use macroquad_toolkit::raster::{fill_rect, fill_circle, fill_ellipse,
                                draw_line_pixels, add_noise, set_pixel_safe};

let mut image = Image::gen_image_color(64, 64, BLANK);
fill_rect(&mut image, 4, 4, 56, 40, wall_color);
fill_ellipse(&mut image, 32, 50, 20, 8, shadow_color);
draw_line_pixels(&mut image, 0, 63, 63, 0, trim_color);   // Bresenham
add_noise(&mut image, seed, 0.15);                        // grain, stable per seed
let texture = Texture2D::from_image(&image);
```

### Capture (`capture` module)

Headless screenshot harness: when a `PREFIX_CAPTURE_PATH` env var is set, the
game boots into a chosen scene, simulates a fixed number of frames at a fixed
timestep, writes a PNG, and exits. This makes UI changes visually verifiable
from a script (or by an AI agent reading the PNG back) with no interactive
input. Full walkthrough and gotchas: `docs/screenshot_capture_harness_guide.md`.

```rust
use macroquad_toolkit::capture;

fn window_conf() -> Conf {
    // Reads MYGAME_WINDOW_WIDTH/HEIGHT overrides; disables high_dpi while
    // capturing so screenshots are pixel-aligned with the logical layout.
    capture::capture_window_conf("MYGAME", "My Game", 1280, 720)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    if let Some(config) = capture::CaptureConfig::from_env("MYGAME") {
        game.begin_capture_scene(&config.scene); // your scene-seeding method
        capture::run_capture(&config, |dt| {
            game.update(dt);
            game.draw();
        })
        .await;
        return;
    }

    loop { /* normal interactive loop */ }
}
```

The only per-game code is `begin_capture_scene(&str)` — a method that puts the
session into a named scene (e.g. `"gameplay"`, `"map"`, `"loadout"`) so the
capture starts in the state you want to photograph.

Env vars (replace `MYGAME` with your per-game prefix):

- `MYGAME_CAPTURE_PATH` — output PNG path; presence enables capture mode
- `MYGAME_CAPTURE_SCENE` — scene name (default `gameplay`)
- `MYGAME_CAPTURE_FRAMES` — frames to simulate before capture (default 150)
- `MYGAME_WINDOW_WIDTH` / `MYGAME_WINDOW_HEIGHT` — window size override

All env access is stubbed out on `wasm32`, so web builds are unaffected.

A shared wrapper script (`macroquad-toolkit/scripts/capture_ui.ps1`) builds the
game, runs one capture per scene, and sanity-checks each PNG. It derives the
package name, exe path, and env prefix from `cargo metadata`, so from a game
directory it needs no arguments:

```powershell
& ..\macroquad-toolkit\scripts\capture_ui.ps1 -Scenes gameplay,map
& ..\macroquad-toolkit\scripts\capture_ui.ps1 -Scenes gameplay -SkipBuild
```

Pass `-Prefix` if the game's env-var prefix differs from its package name
(e.g. `carriage_run` uses `CARRIAGE`). See `carriage_run` for a reference
integration, including a thin per-game `scripts/capture_ui.ps1` wrapper.

### Notifications (`notifications` module)

Queue-based toast notifications with fade-out and typed styling.

```rust
use macroquad_toolkit::notifications::{NotificationManager, NotificationRenderConfig};

let mut notifications = NotificationManager::new();   // keep in Game
notifications.success("Saved");
notifications.warning("Low health");
notifications.info("New quest");

notifications.update(get_frame_time());
notifications.draw();                                  // or draw_with_config(&cfg)
```

### States (`states` module)

A `GameState<T>` trait standardizing what a screen looks like (`update`/`draw`
plus `on_enter`/`on_exit` hooks) and a `Transition` enum. Most games still drive
their own top-level `GameState` enum with explicit matching (see
`CODE_STANDARDS.md` §5.4) — reach for this only when a trait-object screen stack
fits better.

```rust
use macroquad_toolkit::states::{GameState, Transition};

impl GameState<Game> for MenuScreen {
    fn update(&mut self, game: &mut Game) -> Option<Box<dyn std::any::Any>> {
        if start_clicked { Some(Box::new(Transition::Switch)) } else { None }
    }
    fn draw(&self, game: &Game) { /* render */ }
}
```

### Data loading (`data_loader` module)

Helpers for loading JSON game data — embedded at compile time via `include_str!`
(WASM-safe) or from disk at runtime, plus a keyed `DataRegistry<T>`.

```rust
use macroquad_toolkit::data_loader::{load_embedded_json, load_json_with_fallback_sync};

// Compile-time embed (works on WASM):
let config: Config = load_embedded_json(include_str!("../assets/config.json"))?;

// Native: read assets/ from disk, fall back to the embedded copy:
let config: Config = load_json_with_fallback_sync(
    "assets/config.json",
    include_str!("../assets/config.json"),
)?;
```

### Persistence (`persistence` module)

Save/load with native (JSON files) and WASM (`localStorage`) backends behind one
keyed API. Use the `*_json_key(game_name, key, ..)` helpers or implement
`JsonStorage` for slot-based saves; also provides named save slots (`slots`),
atomic file writes, versioned migration (`version`), and `AutoSaveManager` for
interval autosaves.

```rust
use macroquad_toolkit::persistence::{JsonStorage, save_json_key, load_json_key, AutoSaveManager};

#[derive(Serialize, Deserialize)]
struct SaveData { level: u32 }
impl JsonStorage for SaveData {}

// Keyed helpers (native file or WASM localStorage, chosen at compile time):
save_json_key("my_game", "save", &data).ok();
let data: Result<SaveData, String> = load_json_key("my_game", "save");

// Or via the trait's slot methods:
data.save_to_slot("my_game", "slot1").ok();

// Autosave: call every frame; the closure runs only when the interval elapses.
let mut autosave = AutoSaveManager::new(30.0);   // seconds between autosaves
autosave.update(get_frame_time(), dirty, || save_json_key("my_game", "auto", &data)).ok();
```

### Entities (`entities` module)

A lightweight generic entity store (`EntityManager<T>`) with opt-in
`HasPosition` / `HasHealth` / `HasStatusEffects` traits for spatial queries,
damage, and timed status effects — suited to grid/strategy games that don't need
a full ECS.

```rust
use macroquad_toolkit::entities::{EntityManager, HasPosition};

let mut entities: EntityManager<Unit> = EntityManager::new();
let id = entities.spawn(Unit::new());
if let Some(unit) = entities.get_mut(id) { unit.hp -= 5; }
entities.remove(id);
```

### Grid (`grid` module)

Tile-grid data structures and coordinate math: `Grid<T>` and `FlatGrid`, a
`TilePos` type, isometric ↔ world conversion, fog-of-war state, line-of-sight /
vision-radius queries, and BFS pathfinding / flood fill over tiles.

```rust
use macroquad_toolkit::grid::{Grid, TilePos, bfs_path, has_line_of_sight, world_to_iso};

let mut tiles: Grid<Tile> = Grid::new(width, height, Tile::Floor);
tiles.set(TilePos::new(3, 4), Tile::Wall);

let path = bfs_path(start, goal, false, |p| tiles.get(p) != Some(&Tile::Wall), |_| true);
// predicate returns true when a tile *blocks* vision:
let visible = has_line_of_sight(from, to, |p| tiles.get(p) == Some(&Tile::Wall));
```

### Pathfinding (`pathfinding` module)

A* pathfinding over a weighted `PathfindingGrid` (`Pos` coordinates), with
4/8-way movement, per-tile costs, selectable heuristics, and an optional
`PathCache`. Distinct from `grid::bfs_path`, which is unweighted BFS over
`TilePos`.

```rust
use macroquad_toolkit::pathfinding::{PathfindingGrid, Pos, find_path, Heuristic};

let mut grid = PathfindingGrid::new(64, 64);
grid.set_walkable(Pos { x: 10, y: 5 }, false);

if let Some(path) = find_path(start, goal, &grid, Heuristic::Manhattan, false) {
    let next = path.next_after(current);   // step-follow the waypoints
}
```

### 3D rendering (`render3d` module)

Billboards (camera-facing 2D sprites in 3D space) and camera helpers
(`IsometricCamera`, `OrbitCamera`) for tile/strategy 3D views. See also
`prelude_3d`.

```rust
use macroquad_toolkit::render3d::{draw_billboard, IsometricCamera};

let mut camera = IsometricCamera::new(0.0, 0.0);
camera.update(get_frame_time());
// ...set_camera(&camera.into())..., then:
draw_billboard(world_pos, vec2(1.0, 1.0), &texture, camera_position);
```

### Database (`db` module, optional)

SQLite access via `sqlx`, gated behind the `db` cargo feature (off by default —
enable with `features = ["db"]`). Most games don't need it; it exists for tools
and servers that back onto a database.

```toml
macroquad-toolkit = { path = "../macroquad-toolkit", features = ["db"] }
```

## Button Click Semantics

The toolkit provides two button variants to handle different click behaviors:

- **`button()` and `button_on_release()`**: Fire when mouse button is **released** over the button. This is the safer default as it prevents accidental double-clicks and allows users to move the mouse away to cancel.

- **`button_on_press()`**: Fires when mouse button is **pressed down** over the button. Use this for instant feedback scenarios.

## License

This toolkit is extracted from game projects and shared for reuse across multiple games.
