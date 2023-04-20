
macro_rules! spawn_button {

    ($parent:expr,$ui_assets:expr,$buttonComponent:expr,$buttonLabel:expr) => {
        // The macro will expand into the contents of this block.
        $parent.spawn((ButtonBundle
            {
                style: Style{
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(20.0),Val::Percent(100.0)),
                    margin: UiRect::all(Val::Auto),
                    
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },$buttonComponent))
            .with_children(|parent|
                {
                parent.spawn(ImageBundle{
                    style: Style{
                        size: Size::new(Val::Percent(100.0),Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: $ui_assets.button.clone().into(),
                    ..default()
                }).insert(FocusPolicy::Pass).with_children(|parent|{
                    parent.spawn(TextBundle{
                        text: Text::from_section(
                            $buttonLabel,
                            TextStyle{
                                font: $ui_assets.font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9,0.9,0.9),
                            }),
                            focus_policy: FocusPolicy::Pass,
                            ..Default::default()
                        });
                        
                    });
                });
    };
}
macro_rules! spawn_editable_label {

    ($parent:expr,$ui_assets:expr,$labelComponent:expr,$buttonLabel:expr) => {
        $parent.spawn((
        TextBundle::from_section(
            $buttonLabel,
            TextStyle {
                font: $ui_assets.font.clone(),
                font_size: 20.0,
                color: Color::rgb(0.9,0.9,0.9),
            },
        )
        .with_style(Style {
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(20.0),Val::Percent(100.0)),
            margin: UiRect::all(Val::Px(50.0)),
            ..default()
        }),
        $labelComponent));
    };
}
macro_rules! spawn_label {

    ($parent:expr,$ui_assets:expr,$font_size:expr,$text:expr) => {
        $parent.spawn(
            TextBundle::from_section(
                $text,
                TextStyle {
                    font: $ui_assets.font.clone(),
                    font_size: $font_size,
                    color: Color::rgb(0.9,0.9,0.9),
                },
            )
        .with_style(Style {
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(20.0),Val::Percent(100.0)),
            ..default()
    }),);};
    ($parent:expr,$ui_assets:expr,$font_size:expr,$text:expr,$height:expr) => {
        $parent.spawn(
            TextBundle::from_section(
                $text,
                TextStyle {
                    font: $ui_assets.font.clone(),
                    font_size: $font_size,
                    color: Color::rgb(0.9,0.9,0.9),
                },
            )
        .with_style(Style {
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(20.0),Val::Percent($height)),
            ..default()
    }),);
};

($parent:expr,$ui_assets:expr,$font_size:expr,$text:expr,$colour:expr) => {
    $parent.spawn(
        TextBundle::from_section(
            $text,
            TextStyle {
                font: $ui_assets.font.clone(),
                font_size: $font_size,
                color: $colour,
            },
        )
    .with_style(Style {
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(20.0),Val::Percent(100.0)),
            margin: UiRect::all(Val::Px(50.0)),
            ..default()
        }),
    );
};

}

pub(crate) use spawn_button;
pub(crate) use spawn_editable_label;
pub(crate) use spawn_label;