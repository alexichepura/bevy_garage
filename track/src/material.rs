use crate::{AsphaltExtension, GroundExtension};

use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::texture::ImageAddressMode;
use bevy::render::{
    render_resource::{Extent3d, TextureDimension, TextureFormat},
    texture::{ImageSampler, ImageSamplerDescriptor},
};

pub type ExtendedMaterialAsphalt = ExtendedMaterial<StandardMaterial, AsphaltExtension>;
pub type ExtendedMaterialGround = ExtendedMaterial<StandardMaterial, GroundExtension>;

pub type HandleStandard = Handle<StandardMaterial>;
pub type HandleAsphalt = Handle<ExtendedMaterialAsphalt>;
pub type HandleGround = Handle<ExtendedMaterialGround>;

#[derive(Resource)]
pub struct MaterialHandle {
    pub asphalt: Handle<ExtendedMaterialAsphalt>,
    pub ground: Handle<ExtendedMaterialGround>,
    pub asphalt_color: Handle<StandardMaterial>,
    pub ground_color: Handle<StandardMaterial>,
    pub wall: Handle<StandardMaterial>,
    pub kerb: Handle<StandardMaterial>,
}

pub type AsphaltPbr = MaterialMeshBundle<ExtendedMaterialAsphalt>;
pub type GroundPbr = MaterialMeshBundle<ExtendedMaterialGround>;

impl FromWorld for MaterialHandle {
    fn from_world(world: &mut World) -> Self {
        #[cfg(any(target_os = "ios", target_os = "android"))]
        let quality = 2;
        #[cfg(any(target_arch = "wasm32"))]
        let quality = 5;
        #[cfg(not(any(target_arch = "wasm32", target_os = "ios", target_os = "android")))]
        let quality = 10;

        let ground_color: Color = Srgba::hex("6aa84f").unwrap().into();
        let mut ground_materials = world.resource_mut::<Assets<ExtendedMaterialGround>>();
        let ground_handle = ground_materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: ground_color,
                depth_bias: 0.,
                ..Default::default()
            },
            extension: GroundExtension { quality },
        });

        #[cfg(target_arch = "wasm32")]
        let asphalt_depth_bias = 1.;
        #[cfg(not(target_arch = "wasm32"))]
        let asphalt_depth_bias = 100.;

        let asphalt_color: Color = Srgba::hex("333355").unwrap().into();
        let mut asphalt_materials = world.resource_mut::<Assets<ExtendedMaterialAsphalt>>();
        let asphalt_handle = asphalt_materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: asphalt_color,
                depth_bias: asphalt_depth_bias,
                ..Default::default()
            },
            extension: AsphaltExtension { quality },
        });

        let mut images = world.resource_mut::<Assets<Image>>();
        let wall_image_handle = images.add(wall_texture());
        let kerb_image_handle = images.add(kerb_texture());

        let mut standard_materials = world.resource_mut::<Assets<StandardMaterial>>();
        let asphalt_color_handle = standard_materials.add(StandardMaterial {
            base_color: asphalt_color,
            depth_bias: asphalt_depth_bias,
            reflectance: 0.5,
            perceptual_roughness: 0.7,
            ..default()
        });
        let ground_color_handle = standard_materials.add(StandardMaterial {
            base_color: ground_color,
            depth_bias: 0.,
            reflectance: 0.5,
            perceptual_roughness: 0.75,
            ..default()
        });
        let wall_handle = standard_materials.add(StandardMaterial {
            base_color_texture: Some(wall_image_handle),
            perceptual_roughness: 0.7,
            depth_bias: 0.,
            ..default()
        });
        let kerb_handle = standard_materials.add(StandardMaterial {
            base_color_texture: Some(kerb_image_handle),
            perceptual_roughness: 0.7,
            depth_bias: 1.,
            ..default()
        });

        Self {
            asphalt: asphalt_handle,
            asphalt_color: asphalt_color_handle,
            ground: ground_handle,
            ground_color: ground_color_handle,
            kerb: kerb_handle,
            wall: wall_handle,
        }
    }
}

fn wall_texture() -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width: 2,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[
            8, 8, 8, 255, // darker
            128, 128, 128, 255, // dark
        ],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        ..Default::default()
    });

    image
}
fn kerb_texture() -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width: 2,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[
            210, 20, 20, 255, // red
            210, 210, 210, 255, // white
        ],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        ..Default::default()
    });

    image
}

// fn uv_debug_texture() -> Image {
//     const TEXTURE_SIZE: usize = 8;
//     let mut palette: [u8; 32] = [
//         255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
//         198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
//     ];
//     let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
//     for y in 0..TEXTURE_SIZE {
//         let offset = TEXTURE_SIZE * y * 4;
//         texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
//         palette.rotate_right(4);
//     }
//     let mut image = Image::new_fill(
//         Extent3d {
//             width: TEXTURE_SIZE as u32,
//             height: TEXTURE_SIZE as u32,
//             depth_or_array_layers: 1,
//         },
//         TextureDimension::D2,
//         &texture_data,
//         TextureFormat::Rgba8UnormSrgb,
//     );
//     image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
//         address_mode_u: AddressMode::Repeat,
//         address_mode_v: AddressMode::Repeat,
//         address_mode_w: AddressMode::Repeat,
//         ..Default::default()
//     });
//     image
// }
