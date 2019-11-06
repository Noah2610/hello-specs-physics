mod build_camera;
mod build_player;
mod build_tiles;

use crate::sprite_sheet_handles::SpriteSheetHandles;
use amethyst::ecs::world::Index;
use amethyst::ecs::{Builder, World, WorldExt};
use amethyst::ecs::{Entities, Join, ReadStorage};
use amethyst::renderer::{Camera, SpriteRender};
use json::JsonValue;

use crate::components::prelude::*;
use crate::helpers::*;

const TILE_SIZE: (f32, f32) = (16.0, 16.0);
const LEVELS_DIR: &str = "levels";
const PROPERTY_Z_KEY: &str = "z";

struct EntityData {
    pub pos:        Vector,
    pub size:       Vector,
    pub properties: JsonValue,
    pub sprite:     Option<SpriteData>,
}

struct SpriteData {
    pub spritesheet_path: String,
    pub sprite_id:        usize,
}

#[derive(Default)]
pub struct LevelLoader {
    finished_loading: bool,
    level_size:       Option<Vector>,
    player_data:      Option<EntityData>,
    tiles_data:       Vec<EntityData>,
}

impl LevelLoader {
    pub fn load<S>(&mut self, filename: S)
    where
        S: ToString,
    {
        use std::fs::File;
        use std::io::Read;

        self.finished_loading = false;

        self.level_size = Default::default();
        self.player_data = Default::default();
        self.tiles_data = Default::default();

        let path = resource(format!("{}/{}", LEVELS_DIR, filename.to_string()));
        let mut file = File::open(&path)
            .expect(&format!("Couldn't open level file {}", &path));
        let mut raw = String::new();
        file.read_to_string(&mut raw)
            .expect("Couldn't read file contents to string");
        let json = json::parse(&raw).expect("Couldn't parse level JSON file");

        self.load_level_data(&json["level"]);
        self.load_objects(&json["objects"]);
        self.load_tiles(&json["tiles"]);
    }

    pub fn build(&mut self, world: &mut World) {
        self.finished_loading = false;

        self.build_player(world);
        self.build_camera(world);
        self.build_tiles(world);

        self.finished_loading = true;
    }

    pub fn is_finished(&self) -> bool {
        self.finished_loading
    }

    fn load_level_data(&mut self, json: &JsonValue) {
        const ERRMSG: &str = "\"level\".\"size\" values should be f32";
        for (key, val) in json.entries() {
            match key {
                "size" => {
                    self.level_size = Some((
                        val["w"].as_f32().expect(ERRMSG),
                        val["h"].as_f32().expect(ERRMSG),
                    ))
                }
                _ => (),
            }
        }
    }

    fn load_objects(&mut self, json: &JsonValue) {
        for object_data in json.members() {
            if let (
                Some(obj_type),
                (Some(x), Some(y)),
                (Some(w), Some(h)),
                properties,
            ) = (
                object_data["type"].as_str(),
                (
                    object_data["pos"]["x"].as_f32(),
                    object_data["pos"]["y"].as_f32(),
                ),
                (
                    object_data["size"]["w"].as_f32(),
                    object_data["size"]["h"].as_f32(),
                ),
                &object_data["properties"],
            ) {
                let size = (w, h);
                let pos = (x + size.0 * 0.5, y - size.1 * 0.5);

                match obj_type {
                    "Player" => {
                        self.player_data = Some(EntityData {
                            pos,
                            size,
                            sprite: None,
                            properties: properties.clone(),
                        });
                    }
                    _ => (),
                }
            }
        }
    }

    fn load_tiles(&mut self, json: &JsonValue) {
        for tile_data in json.members() {
            if let (
                Some(id),
                (Some(x), Some(y)),
                properties,
                Some(tileset_name),
            ) = (
                tile_data["id"].as_usize(),
                (
                    tile_data["pos"]["x"].as_f32(),
                    tile_data["pos"]["y"].as_f32(),
                ),
                &tile_data["properties"],
                tile_data["ts"].as_str(),
            ) {
                let spritesheet_path =
                    resource(format!("spritesheets/{}.png", tileset_name));
                let size = TILE_SIZE;
                let pos = (x + size.0 * 0.5, y - size.1 * 0.5);

                self.tiles_data.push(EntityData {
                    pos,
                    size,
                    properties: properties.clone(),
                    sprite: Some(SpriteData {
                        spritesheet_path,
                        sprite_id: id,
                    }),
                });
            }
        }
    }
}

fn is_solid(properties: &JsonValue) -> bool {
    properties["solid"].as_bool().unwrap_or(false)
}
