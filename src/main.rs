use {
    ply_generator_lib::{
        V3,
        print_ply,
        merge_meshes,
        negate_z_coord,
        flip_normals
    },
    simple_tiled_wfc::grid_generation::{WfcModule, WfcContext, WfcContextBuilder},
    ron::de::from_reader,
    crate::{
        types::{ControlPoints},
        wang_info::{
            PTS_FILE,
            is_north_neighbour,
            is_south_neighbour,
            is_west_neighbour,
            is_east_neighbour
        }
    },
    bitsetium::Bits128,
    std::sync::mpsc::channel
};

mod wang_info;
mod types;
mod generators;

fn main() {
    let points: ControlPoints = from_reader(PTS_FILE).expect("failed to deserialize RON file");
    wfc_experiment(&points);
}

fn wfc_experiment(points: &ControlPoints) {
    let mut tile_indices = Vec::with_capacity(96);
    let mut modules = Vec::with_capacity(96);
    for i in 0..96 {
        let idx = (i as usize) % 32;
        let height_prev = (i as u16) / 32;

        let mut tile_id = wang_info::TILE_MASKS[idx] & 0b1_1111_00_00_00_00;
        tile_id = tile_id |
            if (wang_info::TILE_MASKS[idx] & wang_info::NW) != 0 {
                height_prev + 1
            } else {
                height_prev
            } << 0u16;
        tile_id = tile_id |
            if (wang_info::TILE_MASKS[idx] & wang_info::NE) != 0 {
                height_prev + 1
            } else {
                height_prev
            } << 2u16;
        tile_id = tile_id |
            if (wang_info::TILE_MASKS[idx] & wang_info::SW) != 0 {
                height_prev + 1
            } else {
                height_prev
            } << 4u16;
        tile_id = tile_id |
            if (wang_info::TILE_MASKS[idx] & wang_info::SE) != 0 {
                height_prev + 1
            } else {
                height_prev
            } << 6u16;

        tile_indices.push(tile_id);
    }
    let mut tile_indices_indices =
        (0..tile_indices.len())
            .map(|it| (it, it / 32))
            .collect::<Vec<_>>();
    for i in (0..3).rev() {
        let idx = 32 * i;
        tile_indices_indices[idx + 30] = tile_indices_indices[idx + 15];
        tile_indices_indices[idx + 29] = tile_indices_indices[idx + 15];
        tile_indices_indices[idx + 28] = tile_indices_indices[idx + 15];

        tile_indices_indices[idx + 26] = tile_indices_indices[idx + 15];
        tile_indices_indices[idx + 25] = tile_indices_indices[idx + 15];
        tile_indices_indices[idx + 24] = tile_indices_indices[idx];

        tile_indices_indices[idx + 22] = tile_indices_indices[idx];
        tile_indices_indices[idx + 21] = tile_indices_indices[idx];
        tile_indices_indices[idx + 20] = tile_indices_indices[idx];

        tile_indices_indices[idx + 18] = tile_indices_indices[idx];
        tile_indices_indices[idx + 17] = tile_indices_indices[idx];
        tile_indices_indices[idx + 16] = tile_indices_indices[idx];
    }

    for i in 0..tile_indices_indices.len() {
        let mut module: WfcModule<Bits128> = WfcModule::new();
        let tile_idx = tile_indices_indices[i].0;
        let tile = tile_indices[tile_idx];
        for j in 0..tile_indices_indices.len() {
            let other_idx = tile_indices_indices[j].0;
            let other_rile = tile_indices[other_idx];
            if is_north_neighbour(tile, other_rile) {
                if check_slide_indices_south_north(tile_idx, other_idx) {
                    module.add_north_neighbour(j);
                }
            }
            if is_south_neighbour(tile, other_rile) {
                if check_slide_indices_south_north(tile_idx, other_idx) {
                    module.add_south_neighbour(j);
                }
            }
            if is_west_neighbour(tile, other_rile) {
                if check_slide_indices_east_west(tile_idx, other_idx) {
                    module.add_west_neighbour(j);
                }
            }
            if is_east_neighbour(tile, other_rile) {
                if check_slide_indices_east_west(tile_idx, other_idx) {
                    module.add_east_neighbour(j);
                }
            }
        }
        modules.push(module);

        fn check_slide_indices_east_west(tile_idx: usize, other_idx: usize) -> bool {
            let is_same_idx = tile_idx == other_idx;
            let is_same_slide = is_same_idx &&
                (
                    tile_idx == 19 ||
                    tile_idx == 31 ||
                    tile_idx == 19 + 32 ||
                    tile_idx == 31 + 32 ||
                    tile_idx == 19 + 64 ||
                    tile_idx == 31 + 64
                );
            let is_opposing_slide =
                tile_idx == 23 && other_idx == 27 ||
                tile_idx == 27 && other_idx == 23 ||
                tile_idx == 23 + 32 && other_idx == 27 + 32 ||
                tile_idx == 27 + 32 && other_idx == 23 + 32 ||
                tile_idx == 23 + 64 && other_idx == 27 + 64 ||
                tile_idx == 27 + 64 && other_idx == 23 + 64 ||
                tile_idx == 23 && (other_idx == 10 || other_idx == 11 || other_idx == 14) ||
                other_idx == 23 && (tile_idx == 10 || tile_idx == 11 || tile_idx == 14) ||
                tile_idx == 27 && (other_idx == 5 || other_idx == 7 || other_idx == 13) ||
                other_idx == 27 && (tile_idx == 5 || tile_idx == 7 || tile_idx == 13) ||
                tile_idx == 32 + 23 && (other_idx == 32 + 10 || other_idx == 32 + 11 || other_idx == 32 + 14) ||
                other_idx == 32 + 23 && (tile_idx == 32 + 10 || tile_idx == 32 + 11 || tile_idx == 32 + 14) ||
                tile_idx == 32 + 27 && (other_idx == 32 + 5 || other_idx == 32 + 7 || other_idx == 32 + 13) ||
                other_idx == 32 + 27 && (tile_idx == 32 + 5 || tile_idx == 32 + 7 || tile_idx == 32 + 13) ||
                tile_idx == 64 + 23 && (other_idx == 64 + 10 || other_idx == 64 + 11 || other_idx == 64 + 14) ||
                other_idx == 64 + 23 && (tile_idx == 64 + 10 || tile_idx == 64 + 11 || tile_idx == 64 + 14) ||
                tile_idx == 64 + 27 && (other_idx == 64 + 5 || other_idx == 64 + 7 || other_idx == 64 + 13) ||
                other_idx == 64 + 27 && (tile_idx == 64 + 5 || tile_idx == 64 + 7 || tile_idx == 64 + 13);

            !is_same_slide && !is_opposing_slide
        }

        fn check_slide_indices_south_north(tile_idx: usize, other_idx: usize) -> bool {
            let is_same_idx = tile_idx == other_idx;
            let is_same_slide = is_same_idx &&
                (
                    tile_idx == 23 ||
                    tile_idx == 27 ||
                    tile_idx == 23 + 32 ||
                    tile_idx == 27 + 32 ||
                    tile_idx == 23 + 64 ||
                    tile_idx == 27 + 64
                );
            let is_opposing_slide =
                tile_idx == 19 && other_idx == 31 ||
                tile_idx == 31 && other_idx == 19 ||
                tile_idx == 19 + 32 && other_idx == 31 + 32 ||
                tile_idx == 31 + 32 && other_idx == 19 + 32 ||
                tile_idx == 19 + 64 && other_idx == 31 + 64 ||
                tile_idx == 31 + 64 && other_idx == 19 + 64 ||
                tile_idx == 19 && (other_idx == 12 || other_idx == 13 || other_idx == 14) ||
                other_idx == 19 && (tile_idx == 12 || tile_idx == 13 || tile_idx == 14) ||
                tile_idx == 31 && (other_idx == 3 || other_idx == 7 || other_idx == 11) ||
                other_idx == 31 && (tile_idx == 3 || tile_idx == 7 || tile_idx == 11) ||
                tile_idx == 32 + 19 && (other_idx == 32 + 12 || other_idx == 32 + 13 || other_idx == 32 + 14) ||
                other_idx == 32 + 19 && (tile_idx == 32 + 12 || tile_idx == 32 + 13 || tile_idx == 32 + 14) ||
                tile_idx == 32 + 31 && (other_idx == 32 + 3 || other_idx == 32 + 7 || other_idx == 32 + 11) ||
                other_idx == 32 + 31 && (tile_idx == 32 + 3 || tile_idx == 32 + 7 || tile_idx == 32 + 11) ||
                tile_idx == 64 + 19 && (other_idx == 64 + 12 || other_idx == 64 + 13 || other_idx == 64 + 14) ||
                other_idx == 64 + 19 && (tile_idx == 64 + 12 || tile_idx == 64 + 13 || tile_idx == 64 + 14) ||
                tile_idx == 64 + 31 && (other_idx == 64 + 3 || other_idx == 64 + 7 || other_idx == 64 + 11) ||
                other_idx == 64 + 31 && (tile_idx == 64 + 3 || tile_idx == 64 + 7 || tile_idx == 64 + 11);

            !is_same_slide && !is_opposing_slide
        }
    }

    let w = 36;
    let h = 36;

    let (tx, rc) = channel();

    let mut wfc_context: WfcContext<Bits128> = WfcContextBuilder::new(&modules, w, h).build();
    wfc_context.set_module(0, 0, 0);
    wfc_context.set_module(0, 35, 0);
    wfc_context.set_module(35, 0, 0);
    wfc_context.set_module(35, 35, 0);
    wfc_context.set_module(17, 17, modules.len()-1);
    wfc_context.collapse(100, tx.clone());

    let wfc_result = rc.recv().unwrap();

    match wfc_result {
        Ok(results) => {
            let mut meshes = Vec::with_capacity(w * h);
            let mut offset = 0;
            for result_id in results {
                let x = ((offset % w) * 2) as f32;
                let z = ((offset / w) as f32) * -2.0;
                let y = tile_indices_indices[result_id].1 as f32;
                let origin = V3 { x, y, z };
                let true_id = tile_indices_indices[result_id].0;
                meshes.push(wang_info::TILE_DEFINITIONS[true_id % 32].gen_mesh(&points, origin, 0.87));
                offset += 1;
            }

            let mut merged = merge_meshes(&meshes);
            negate_z_coord(&mut merged);
            flip_normals(&mut merged);

            let printed = print_ply(&merged, Some("test"));
            print!("{}", &printed);
        }
        Err(_) => {}
    }
}