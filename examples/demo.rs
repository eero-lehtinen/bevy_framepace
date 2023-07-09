use bevy::{prelude::*, render::pipelined_rendering::PipelinedRenderingPlugin};

fn main() {
    App::new()
        // Disable pipelined rendering to prioritize latency reduction
        .add_plugins((
            DefaultPlugins.build().disable::<PipelinedRenderingPlugin>(),
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
            bevy_framepace::FramepacePlugin,
            bevy_framepace::debug::DiagnosticsPlugin,
            bevy_framepace::debug::CursorPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_plugin, update_ui))
        .run();
}

#[derive(Component)]
struct EnableText;

fn toggle_plugin(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        use bevy_framepace::Limiter;
        settings.limiter = match settings.limiter {
            Limiter::Auto => Limiter::Off,
            Limiter::Off => Limiter::from_framerate(30.0),
            Limiter::Manual(_) => Limiter::Auto,
        }
    }
}

fn update_ui(
    mut text: Query<&mut Text, With<EnableText>>,
    settings: Res<bevy_framepace::FramepaceSettings>,
) {
    text.single_mut().sections[1].value = format!("{}", settings.limiter);
}

/// set up the scene
fn setup(mut commands: Commands, mut windows: Query<&mut Window>, asset_server: Res<AssetServer>) {
    windows.iter_mut().next().unwrap().cursor.icon = CursorIcon::Crosshair;
    commands.spawn((Camera3dBundle::default(),));
    // UI
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    commands.spawn((
        TextBundle::from_sections(vec![
            TextSection {
                value: "Frame pacing: ".to_string(),
                style: style.clone(),
            },
            TextSection {
                value: "".to_string(),
                style: style.clone(),
            },
            TextSection {
                value: "\n[press space]".to_string(),
                style,
            },
        ]),
        EnableText,
    ));
}
