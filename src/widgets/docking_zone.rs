use bevy::{
    ecs::system::{Command, CommandQueue},
    prelude::*,
};

use crate::{
    drag_interaction::{DragState, Draggable},
    drop_interaction::{DropPhase, DropZone, DroppableUpdate},
    ui_builder::{UiBuilder, UiBuilderExt},
    ui_style::{
        SetBackgroundColorExt, SetNodeHeightExt, SetNodeLeftExt, SetNodeShowHideExt, SetNodeTopExt,
        SetNodeWidthExt, UiStyleExt,
    },
};

use super::{
    floating_panel::{FloatingPanelPreUpdate, FloatingPanelTitle},
    prelude::{SizedZoneConfig, UiSizedZoneExt, UiTabContainerExt},
    sized_zone::{SizedZone, SizedZoneResizeHandleContainer},
    tab_container::{Tab, TabBar, TabContainer, UiTabContainerSubExt},
};

pub struct DockingZonePlugin;
// TODO: Un-split docking zones once there is only one child
// TODO: Re-write docking zone ui builder
impl Plugin for DockingZonePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, DockingZoneUpdate.after(DroppableUpdate))
            .add_systems(
                PreUpdate,
                remove_empty_docking_zones
                    .run_if(should_process_empty_docking_zones)
                    .after(FloatingPanelPreUpdate),
            )
            .add_systems(
                Update,
                (
                    update_docking_zone_resize_handles.run_if(should_update_resize_handles),
                    handle_docking_zone_drop_zone_change,
                )
                    .in_set(DockingZoneUpdate),
            );
    }
}

#[derive(SystemSet, Clone, Eq, Debug, Hash, PartialEq)]
pub struct DockingZoneUpdate;

fn should_process_empty_docking_zones(q_removed_tabs: RemovedComponents<Tab>) -> bool {
    q_removed_tabs.len() > 0
}

// TODO: Use TabContainer->tab_count instead!
// TODO: Move logic to TabContainer?, remove empty or single-docking-zone-children zones
fn remove_empty_docking_zones(
    q_empty_tab_bars: Query<&TabBar, Without<Children>>,
    q_tab_bars: Query<(&TabBar, &Children)>,
    q_tab: Query<&Tab>,
    q_tab_container_to_remove: Query<&RemoveEmptyDockingZone>,
    mut commands: Commands,
) {
    for tab_bar in &q_empty_tab_bars {
        let Ok(to_remove) = q_tab_container_to_remove.get(tab_bar.container_id()) else {
            continue;
        };

        commands.entity(to_remove.zone).despawn_recursive();
    }

    for (tab_bar, children) in &q_tab_bars {
        let Ok(to_remove) = q_tab_container_to_remove.get(tab_bar.container_id()) else {
            continue;
        };

        if children.iter().any(|child| q_tab.get(*child).is_ok()) {
            continue;
        }

        commands.entity(to_remove.zone).despawn_recursive();
    }
}

fn should_update_resize_handles(
    q_accepted_types: Query<&Draggable, (With<FloatingPanelTitle>, Changed<Draggable>)>,
) -> bool {
    q_accepted_types
        .iter()
        .any(|draggable| draggable.state != DragState::Inactive)
}

fn update_docking_zone_resize_handles(
    q_accepted_types: Query<&Draggable, (With<FloatingPanelTitle>, Changed<Draggable>)>,
    q_handle_containers: Query<Entity, With<SizedZoneResizeHandleContainer>>,
    mut commands: Commands,
) {
    let dragging = q_accepted_types.iter().any(|draggable| {
        draggable.state == DragState::DragStart || draggable.state == DragState::Dragging
    });

    for container in &q_handle_containers {
        commands.style(container).render(!dragging);
    }
}

fn handle_docking_zone_drop_zone_change(
    q_docking_zones: Query<
        (Entity, &DockingZone, &DropZone, &Node, &GlobalTransform),
        Changed<DropZone>,
    >,
    q_accepted_query: Query<&FloatingPanelTitle>,
    q_tab_container: Query<&TabContainer>,
    q_tab_bar: Query<(&Node, &Interaction), With<TabBar>>,
    mut commands: Commands,
) {
    for (entity, docking_zone, drop_zone, node, transform) in &q_docking_zones {
        let Ok(tab_container) = q_tab_container.get(docking_zone.tab_container) else {
            warn!("Docking zone {:?} missing its tab container!", entity);
            continue;
        };

        let Ok((tab_bar_node, bar_interaction)) = q_tab_bar.get(tab_container.bar_id()) else {
            warn!(
                "Tab container {:?} missing its tab bar!",
                docking_zone.tab_container
            );
            continue;
        };

        let center = transform.translation().truncate();
        let tab_bar_height = tab_bar_node.size().y;

        if *bar_interaction == Interaction::Hovered
            || drop_zone.drop_phase() == DropPhase::Inactive
            || drop_zone.drop_phase() == DropPhase::DropCanceled
            || drop_zone.drop_phase() == DropPhase::DroppableLeft
            || drop_zone.incoming_droppable().is_none()
            || q_accepted_query
                .get(drop_zone.incoming_droppable().unwrap())
                .is_err()
        {
            commands
                .style(docking_zone.zone_highlight)
                .background_color(Color::NONE);

            continue;
        }

        // How else would the droppable be over the zone?
        let position = drop_zone.position().unwrap();
        let drop_area = calculate_drop_area(position, center, node.size());

        if drop_zone.drop_phase() == DropPhase::DroppableEntered
            || drop_zone.drop_phase() == DropPhase::DroppableHover
        {
            let full_size = Val::Percent(100.);
            let half_size = Val::Percent(50.);
            let auto_size = Val::Auto;

            let (width, height, top, left) = match drop_area {
                DropArea::Center => (
                    full_size,
                    Val::Px(node.size().y - tab_bar_height),
                    Val::Px(tab_bar_height),
                    auto_size,
                ),
                DropArea::North => (full_size, half_size, auto_size, auto_size),
                DropArea::East => (half_size, full_size, auto_size, half_size),
                DropArea::South => (full_size, half_size, half_size, auto_size),
                DropArea::West => (half_size, full_size, auto_size, auto_size),
                _ => (full_size, full_size, auto_size, auto_size),
            };

            commands
                .style(docking_zone.zone_highlight)
                .width(width)
                .height(height)
                .left(left)
                .top(top)
                .background_color(Color::rgba(0.7, 0.8, 0.9, 0.2));
        } else if drop_zone.drop_phase() == DropPhase::Dropped {
            // Validated above
            let droppable_title = q_accepted_query
                .get(drop_zone.incoming_droppable().unwrap())
                .unwrap();

            if drop_area == DropArea::Center {
                commands
                    .ui_builder(*tab_container)
                    .dock_panel(droppable_title.panel());
            } else {
                let split_direction = match drop_area {
                    DropArea::North => DockingZoneSplitDirection::VerticallyBefore,
                    DropArea::East => DockingZoneSplitDirection::HorizontallyAfter,
                    DropArea::South => DockingZoneSplitDirection::VerticallyAfter,
                    DropArea::West => DockingZoneSplitDirection::HorizontallyBefore,
                    _ => DockingZoneSplitDirection::VerticallyAfter,
                };

                commands.add(DockingZoneSplit {
                    direction: split_direction,
                    docking_zone: entity,
                    panel_to_dock: droppable_title.panel().into(),
                });
            }

            commands
                .style(docking_zone.zone_highlight)
                .background_color(Color::NONE);
        }
    }
}

fn calculate_drop_area(position: Vec2, center: Vec2, size: Vec2) -> DropArea {
    let sixth_width = size.x / 6.;
    let sixth_height = size.y / 6.;

    if position.x < center.x - sixth_width {
        DropArea::West
    } else if position.x > center.x + sixth_width {
        DropArea::East
    } else if position.y < center.y - sixth_height {
        DropArea::North
    } else if position.y > center.y + sixth_height {
        DropArea::South
    } else {
        DropArea::Center
    }
}

#[derive(PartialEq, Eq)]
enum DockingZoneSplitDirection {
    VerticallyBefore,
    VerticallyAfter,
    HorizontallyBefore,
    HorizontallyAfter,
}

struct DockingZoneSplit {
    docking_zone: Entity,
    direction: DockingZoneSplitDirection,
    panel_to_dock: Option<Entity>,
}

impl Command for DockingZoneSplit {
    fn apply(self, world: &mut World) {
        let Ok((docking_zone, parent, sized_zone)) = world
            .query::<(&DockingZone, &Parent, &SizedZone)>()
            .get(world, self.docking_zone)
        else {
            error!(
                "Tried to split entity {:?} when it isn't a valid DockingZone!",
                self.docking_zone
            );
            return;
        };

        let tab_container_id = docking_zone.tab_container;
        let mut parent_id = parent.get();
        let current_direction = sized_zone.direction();
        let current_size = sized_zone.size();
        let current_min_size = sized_zone.min_size();
        let Ok(_) = world.query::<&TabContainer>().get(world, tab_container_id) else {
            error!(
                "Tab container {:?} missing from docking zone {:?}",
                tab_container_id, self.docking_zone
            );
            return;
        };

        // This must exists, since the Parent exists
        let current_index = world
            .query::<&Children>()
            .get(world, parent_id)
            .unwrap()
            .iter()
            .position(|child| *child == self.docking_zone)
            .unwrap();

        let (inject_container, sibling_before) = match current_direction {
            FlexDirection::Row => match self.direction {
                DockingZoneSplitDirection::VerticallyBefore => (false, true),
                DockingZoneSplitDirection::VerticallyAfter => (false, false),
                DockingZoneSplitDirection::HorizontallyBefore => (true, true),
                DockingZoneSplitDirection::HorizontallyAfter => (true, false),
            },
            FlexDirection::Column => match self.direction {
                DockingZoneSplitDirection::VerticallyBefore => (true, true),
                DockingZoneSplitDirection::VerticallyAfter => (true, false),
                DockingZoneSplitDirection::HorizontallyBefore => (false, true),
                DockingZoneSplitDirection::HorizontallyAfter => (false, false),
            },
            FlexDirection::RowReverse => match self.direction {
                DockingZoneSplitDirection::VerticallyBefore => (false, false),
                DockingZoneSplitDirection::VerticallyAfter => (false, true),
                DockingZoneSplitDirection::HorizontallyBefore => (true, false),
                DockingZoneSplitDirection::HorizontallyAfter => (true, true),
            },
            FlexDirection::ColumnReverse => match self.direction {
                DockingZoneSplitDirection::VerticallyBefore => (true, false),
                DockingZoneSplitDirection::VerticallyAfter => (true, true),
                DockingZoneSplitDirection::HorizontallyBefore => (false, false),
                DockingZoneSplitDirection::HorizontallyAfter => (false, true),
            },
        };

        let mut sized_zone = world
            .query::<&mut SizedZone>()
            .get_mut(world, self.docking_zone)
            .unwrap();

        let new_container_size = if inject_container {
            50.
        } else {
            current_size / 2.
        };
        sized_zone.set_size(new_container_size);

        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, world);

        if inject_container {
            let new_parent_id = commands
                .ui_builder(parent_id)
                .sized_zone(
                    SizedZoneConfig {
                        size: current_size,
                        min_size: current_min_size,
                        ..default()
                    },
                    |_| {},
                )
                .id();
            commands
                .entity(parent_id)
                .insert_children(current_index, &[new_parent_id]);

            parent_id = new_parent_id;
        }

        let new_docking_zone_id = commands
            .ui_builder(parent_id)
            .docking_zone(
                SizedZoneConfig {
                    size: new_container_size,
                    min_size: current_min_size,
                    ..default()
                },
                self.panel_to_dock.is_some(),
                |tab_container| {
                    if let Some(floating_panel_id) = self.panel_to_dock {
                        tab_container.dock_panel(floating_panel_id);
                    }
                },
            )
            .id();

        if inject_container {
            if sibling_before {
                commands.entity(parent_id).add_child(self.docking_zone);
            } else {
                commands
                    .entity(parent_id)
                    .insert_children(0, &[self.docking_zone]);
            }
        } else {
            if sibling_before {
                commands
                    .entity(parent_id)
                    .insert_children(current_index, &[new_docking_zone_id]);
            } else {
                commands
                    .entity(parent_id)
                    .insert_children(current_index + 1, &[new_docking_zone_id]);
            }
        }

        queue.apply(world);
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
enum DropArea {
    #[default]
    None,
    Center,
    North,
    East,
    South,
    West,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DockingZone {
    tab_container: Entity,
    zone_highlight: Entity,
}

impl Default for DockingZone {
    fn default() -> Self {
        Self {
            tab_container: Entity::PLACEHOLDER,
            zone_highlight: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DockingZoneHighlight {
    zone: Entity,
}

impl Default for DockingZoneHighlight {
    fn default() -> Self {
        Self {
            zone: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct RemoveEmptyDockingZone {
    zone: Entity,
}

impl Default for RemoveEmptyDockingZone {
    fn default() -> Self {
        Self {
            zone: Entity::PLACEHOLDER,
        }
    }
}

impl DockingZone {
    fn zone_highlight() -> impl Bundle {
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            background_color: Color::NONE.into(),
            z_index: ZIndex::Local(100),
            ..default()
        }
    }
}

pub trait UiDockingZoneExt<'w, 's> {
    fn docking_zone<'a>(
        &'a mut self,
        config: SizedZoneConfig,
        remove_empty: bool,
        spawn_children: impl FnOnce(&mut UiBuilder<TabContainer>),
    ) -> UiBuilder<'w, 's, 'a, Entity>;
}

impl<'w, 's> UiDockingZoneExt<'w, 's> for UiBuilder<'w, 's, '_, Entity> {
    fn docking_zone<'a>(
        &'a mut self,
        config: SizedZoneConfig,
        remove_empty: bool,
        spawn_children: impl FnOnce(&mut UiBuilder<TabContainer>),
    ) -> UiBuilder<'w, 's, 'a, Entity> {
        let mut tab_container = Entity::PLACEHOLDER;
        let mut zone_highlight = Entity::PLACEHOLDER;

        let mut docking_zone = self.sized_zone(config, |zone| {
            let zone_id = zone.id();

            let mut new_tab_container = zone.tab_container(spawn_children);
            if remove_empty {
                new_tab_container.insert(RemoveEmptyDockingZone { zone: zone_id });
            }
            tab_container = new_tab_container.id();

            zone_highlight = zone
                .spawn((
                    DockingZone::zone_highlight(),
                    DockingZoneHighlight { zone: zone_id },
                ))
                .id();
        });

        docking_zone.insert((
            DockingZone {
                tab_container,
                zone_highlight,
            },
            Interaction::default(),
            DropZone::default(),
        ));

        docking_zone
    }
}