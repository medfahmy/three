use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::{mem, ops};

// #[cfg(feature = "audio")]
// use crate::audio::{AudioData, Operation as AudioOperation};
use crate::{
    camera::Projection,
    color::{self, Color},
    light::{LightOperation, ShadowMap, ShadowProjection},
    material::Material,
    mesh::DynamicMesh,
    node::{NodeInternal, NodePointer, TransformInternal},
    object::Base,
    render::{GpuData},
    skeleton::{Bone, Skeleton},
    text::{Operation as TextOperation, TextData},
};

use cgmath::Transform;

#[derive(Clone, Debug)]
pub(crate) enum SubLight {
    Ambient,
    Directional,
    Hemisphere { ground: Color },
    Point,
}

#[derive(Clone, Debug)]
pub(crate) struct LightData {
    pub color: Color,
    pub intensity: f32,
    pub sub_light: SubLight,
    pub shadow: Option<(ShadowMap, ShadowProjection)>,
}

#[derive(Clone, Debug)]
pub(crate) struct SkeletonData {
    pub bones: Vec<Bone>,
    pub gpu_buffer_view: BufferView,
    pub gpu_buffer: Buffer,
}

#[derive(Clone, Debug)]
pub(crate) struct VisualData {
    pub material: Material,
    pub gpu: GpuData,
    pub skeleton: Option<Skeleton>,
}

#[derive(Debug)]
pub(crate) enum SubNode {
    /// Camera for rendering a scene.
    Camera(Projection),
    /// Group can be a parent to other objects.
    Group { first_child: Option<NodePointer> },
    #[cfg(feature = "audio")]
    /// Audio data.
    // Audio(AudioData),
    /// Renderable text for 2D user interface.
    UiText(TextData),
    /// Renderable 3D content, such as a mesh.
    Visual(Material, GpuData, Option<Skeleton>),
    /// Lighting information for illumination and shadow casting.
    Light(LightData),
    /// A single bone.
    Bone { index: usize, inverse_bind_matrix: mint::ColumnMatrix4<f32> },
    /// Skeleton root.
    Skeleton(()),
    // Skeleton(SkeletonData),
}

pub(crate) type Message = (froggy::WeakPointer<NodeInternal>, Operation);

#[derive(Debug)]
pub(crate) enum Operation {
    AddChild(NodePointer),
    RemoveChild(NodePointer),
    #[cfg(feature = "audio")]
    // SetAudio(AudioOperation),
    SetVisible(bool),
    SetLight(LightOperation),
    SetText(TextOperation),
    SetTransform(Option<mint::Point3<f32>>, Option<mint::Quaternion<f32>>, Option<f32>),
    SetMaterial(Material),
    SetSkeleton(Skeleton),
    SetShadow(ShadowMap, ShadowProjection),
    SetTexelRange(mint::Point2<i16>, mint::Vector2<u16>),
    SetWeights(Vec<f32>),
    SetName(String),
    SetProjection(Projection),
}

pub(crate) type HubPtr = Arc<Mutex<Hub>>;

pub(crate) struct Hub {
    pub(crate) nodes: froggy::Storage<NodeInternal>,
    pub(crate) message_tx: mpsc::Sender<Message>,
    message_rx: mpsc::Receiver<Message>,
}

impl<T: AsRef<Base>> ops::Index<T> for Hub {
    type Output = NodeInternal;
    fn index(&self, i: T) -> &Self::Output {
        let base: &Base = i.as_ref();
        &self.nodes[&base.node]
    }
}

impl<T: AsRef<Base>> ops::IndexMut<T> for Hub {
    fn index_mut(&mut self, i: T) -> &mut Self::Output {
        let base: &Base = i.as_ref();
        &mut self.nodes[&base.node]
    }
}

impl Hub {
    pub(crate) fn new() -> HubPtr {
        let (tx, rx) = mpsc::channel();
        let hub = Hub { nodes: froggy::Storage::new(), message_tx: tx, message_rx: rx };
        Arc::new(Mutex::new(hub))
    }

    pub(crate) fn spawn(&mut self, sub: SubNode) -> Base {
        Base { node: self.nodes.create(sub.into()), tx: self.message_tx.clone() }
    }

    pub(crate) fn spawn_visual(&mut self, mat: Material, gpu_data: GpuData, skeleton: Option<Skeleton>) -> Base {
        self.spawn(SubNode::Visual(mat, gpu_data, skeleton))
    }

    pub(crate) fn spawn_light(&mut self, data: LightData) -> Base {
        self.spawn(SubNode::Light(data))
    }

    pub(crate) fn spawn_skeleton(&mut self, data: SkeletonData) -> Base {
        self.spawn(SubNode::Skeleton(data))
    }

    /// Upgrades a `NodePointer` to a `Base`.
    pub(crate) fn upgrade_ptr(&self, ptr: NodePointer) -> Base {
        Base { node: ptr, tx: self.message_tx.clone() }
    }

    pub(crate) fn process_messages(&mut self) {
        // while let Ok((weak_ptr, operation)) = self.message_rx.try_recv() {
        //     let ptr = match weak_ptr.upgrade() {
        //         Ok(ptr) => ptr,
        //         Err(_) => continue,
        //     };
        //     match operation {
        //         #[cfg(feature = "audio")]
        //         Operation::SetAudio(operation) => {
        //             if let SubNode::Audio(ref mut data) = self.nodes[&ptr].sub_node {
        //                 Hub::process_audio(operation, data);
        //             }
        //         }
        //         Operation::SetVisible(visible) => {
        //             self.nodes[&ptr].visible = visible;
        //         }
        //         Operation::SetTransform(pos, rot, scale) => {
        //             let transform = &mut self.nodes[&ptr].transform;
        //             if let Some(pos) = pos {
        //                 transform.disp = mint::Vector3::from(pos).into();
        //             }
        //             if let Some(rot) = rot {
        //                 transform.rot = rot.into();
        //             }
        //             if let Some(scale) = scale {
        //                 transform.scale = scale;
        //             }
        //         }
        //         Operation::AddChild(child_ptr) => {
        //             let sibling = match self.nodes[&ptr].sub_node {
        //                 SubNode::Group { ref mut first_child } => mem::replace(first_child, Some(child_ptr.clone())),
        //                 _ => unreachable!(),
        //             };
        //             let child = &mut self.nodes[&child_ptr];
        //             if child.next_sibling.is_some() {
        //                 error!("Element {:?} is added to a group while still having old parent - {}", child.sub_node, "discarding siblings");
        //             }
        //             child.next_sibling = sibling;
        //         }
        //         Operation::RemoveChild(child_ptr) => {
        //             let next_sibling = self.nodes[&child_ptr].next_sibling.clone();
        //             let target_maybe = Some(child_ptr);
        //             let mut cur_ptr = match self.nodes[&ptr].sub_node {
        //                 SubNode::Group { ref mut first_child } => {
        //                     if *first_child == target_maybe {
        //                         *first_child = next_sibling;
        //                         continue;
        //                     }
        //                     first_child.clone()
        //                 }
        //                 _ => unreachable!(),
        //             };
        //
        //             //TODO: consolidate the code with `Scene::remove()`
        //             loop {
        //                 let node = match cur_ptr.take() {
        //                     Some(next_ptr) => &mut self.nodes[&next_ptr],
        //                     None => {
        //                         error!("Unable to find child for removal");
        //                         break;
        //                     }
        //                 };
        //                 if node.next_sibling == target_maybe {
        //                     node.next_sibling = next_sibling;
        //                     break;
        //                 }
        //                 cur_ptr = node.next_sibling.clone(); //TODO: avoid clone
        //             }
        //         }
        //         Operation::SetLight(operation) => match self.nodes[&ptr].sub_node {
        //             SubNode::Light(ref mut data) => {
        //                 Hub::process_light(operation, data);
        //             }
        //             _ => unreachable!(),
        //         },
        //         Operation::SetText(operation) => match self.nodes[&ptr].sub_node {
        //             SubNode::UiText(ref mut data) => {
        //                 Hub::process_text(operation, data);
        //             }
        //             _ => unreachable!(),
        //         },
        //         Operation::SetMaterial(material) => match self.nodes[&ptr].sub_node {
        //             SubNode::Visual(ref mut mat, _, _) => {
        //                 *mat = material;
        //             }
        //             _ => unreachable!(),
        //         },
        //         Operation::SetSkeleton(sleketon) => match self.nodes[&ptr].sub_node {
        //             SubNode::Visual(_, _, ref mut skel) => {
        //                 *skel = Some(sleketon);
        //             }
        //             _ => unreachable!(),
        //         },
        //         Operation::SetShadow(map, proj) => match self.nodes[&ptr].sub_node {
        //             SubNode::Light(ref mut data) => {
        //                 data.shadow = Some((map, proj));
        //             }
        //             _ => unreachable!(),
        //         },
        //         Operation::SetTexelRange(base, size) => match self.nodes[&ptr].sub_node {
        //             SubNode::Visual(Material::Sprite(ref mut params), _, _) => {
        //                 params.map.set_texel_range(base, size);
        //             }
        //             _ => unreachable!(),
        //         },
        //         Operation::SetWeights(weights) => {
        //             fn set_weights(gpu_data: &mut GpuData, weights: &[f32]) {
        //                 use std::iter::repeat;
        //                 for (out, input) in gpu_data.displacement_contributions.iter_mut().zip(weights.iter().chain(repeat(&0.0))) {
        //                     out.weight = *input;
        //                 }
        //             }
        //
        //             let mut x = match self.nodes[&ptr].sub_node {
        //                 SubNode::Visual(_, ref mut gpu_data, _) => {
        //                     set_weights(gpu_data, &weights);
        //                     continue;
        //                 }
        //                 SubNode::Group { ref first_child } => first_child.clone(),
        //                 _ => continue,
        //             };
        //
        //             while let Some(ptr) = x {
        //                 if let SubNode::Visual(_, ref mut gpu_data, _) = self.nodes[&ptr].sub_node {
        //                     set_weights(gpu_data, &weights);
        //                 }
        //                 x = self.nodes[&ptr].next_sibling.clone();
        //             }
        //         }
        //         Operation::SetName(name) => {
        //             self.nodes[&ptr].name = Some(name);
        //         }
        //         Operation::SetProjection(projection) => match self.nodes[&ptr].sub_node {
        //             SubNode::Camera(ref mut internal_projection) => {
        //                 *internal_projection = projection;
        //             }
        //             _ => unreachable!(),
        //         },
        //     }
        // }

        self.nodes.sync_pending();
    }

    #[cfg(feature = "audio")]
    fn process_audio(operation: AudioOperation, data: &mut AudioData) {
        match operation {
            AudioOperation::Append(clip) => data.source.append(clip),
            AudioOperation::Pause => data.source.pause(),
            AudioOperation::Resume => data.source.resume(),
            AudioOperation::Stop => data.source.stop(),
            AudioOperation::SetVolume(volume) => data.source.set_volume(volume),
        }
    }

    fn process_light(operation: LightOperation, data: &mut LightData) {
        match operation {
            LightOperation::Color(color) => data.color = color,
            LightOperation::Intensity(intensity) => data.intensity = intensity,
        }
    }

    fn process_text(operation: TextOperation, data: &mut TextData) {
        // use gfx_glyph::Scale;
        match operation {
            TextOperation::Color(color) => {
                let rgb = color::to_linear_rgb(color);
                data.section.text[0].color = [rgb[0], rgb[1], rgb[2], 1.0];
            }
            TextOperation::Font(font) => data.font = font,
            TextOperation::Layout(layout) => data.section.layout = layout.into(),
            TextOperation::Opacity(opacity) => data.section.text[0].color[3] = opacity,
            TextOperation::Pos(point) => data.section.screen_position = (point.x, point.y),
            // TODO: somehow grab window::hdpi_factor and multiply size
            // TextOperation::Scale(scale) => data.section.text[0].scale = Scale::uniform(scale),
            TextOperation::Size(size) => data.section.bounds = (size.x, size.y),
            TextOperation::Text(text) => data.section.text[0].text = text,
            _ => {}
        }
    }

    pub(crate) fn update_mesh(&mut self, mesh: &DynamicMesh) {
        match self[mesh].sub_node {
            SubNode::Visual(_, ref mut gpu_data, _) => gpu_data.pending = Some(mesh.dynamic.clone()),
            _ => unreachable!(),
        }
    }

    fn walk_impl(&self, base: &Option<NodePointer>, only_visible: bool) -> TreeWalker {
        let default_stack_size = 10;
        let mut walker = TreeWalker { hub: self, only_visible, stack: Vec::with_capacity(default_stack_size) };
        walker.descend(base);
        walker
    }

    pub(crate) fn walk(&self, base: &Option<NodePointer>) -> TreeWalker {
        self.walk_impl(base, true)
    }

    pub(crate) fn walk_all(&self, base: &Option<NodePointer>) -> TreeWalker {
        self.walk_impl(base, false)
    }
}

#[derive(Debug)]
pub(crate) struct WalkedNode<'a> {
    pub(crate) node_ptr: NodePointer,
    pub(crate) node: &'a NodeInternal,
    pub(crate) world_visible: bool,
    pub(crate) world_transform: TransformInternal,
}

pub(crate) struct TreeWalker<'a> {
    hub: &'a Hub,
    only_visible: bool,
    stack: Vec<WalkedNode<'a>>,
}

impl<'a> TreeWalker<'a> {
    fn descend(&mut self, base: &Option<NodePointer>) -> Option<&NodeInternal> {
        // Unwrap the base pointer, returning `None` if `base` is `None`.
        let mut ptr = base.as_ref()?;

        // Note: this is a CPU hotspot, presumably for copying stuff around
        // TODO: profile carefully and optimize
        let mut node = &self.hub.nodes[ptr];

        loop {
            let wn = match self.stack.last() {
                Some(parent) => WalkedNode { node_ptr: ptr.clone(), node, world_visible: parent.world_visible && node.visible, world_transform: parent.world_transform.concat(&node.transform) },
                None => WalkedNode { node_ptr: ptr.clone(), node, world_visible: node.visible, world_transform: node.transform },
            };
            self.stack.push(wn);

            if self.only_visible && !node.visible {
                break;
            }

            match node.sub_node {
                SubNode::Group { first_child: Some(ref child_ptr) } => {
                    ptr = child_ptr;
                    node = &self.hub.nodes[&ptr];
                }
                _ => break,
            }
        }

        Some(node)
    }
}

impl<'a> Iterator for TreeWalker<'a> {
    type Item = WalkedNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(top) = self.stack.pop() {
            self.descend(&top.node.next_sibling);
            if !self.only_visible || top.world_visible {
                return Some(top);
            }
        }
        None
    }
}
