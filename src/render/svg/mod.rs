mod path_convert;
mod stroke_convert;

use lyon::tessellation::geometry_builder::{BuffersBuilder, VertexBuffers};
use lyon::tessellation::{FillOptions, FillTessellator, StrokeTessellator};
use resvg::tree::Transform;
use resvg;

use super::*;
pub use self::path_convert::convert_path;
pub use self::stroke_convert::convert_stroke;

pub fn tessellate(
    dom: &super::Dom,
    fill_tess: &mut FillTessellator,
    stroke_tess: &mut StrokeTessellator,
    mut mesh: &mut VertexBuffers<GpuFillVertex>,
    transform: &mut Option<Transform>,
) {
    for node in dom.rtree.root().descendants() {
        if let resvg::tree::NodeKind::Path(ref p) = **node.borrow() {
            // use the first transform component
            if *transform == None {
                *transform = Some(node.borrow().transform());
            }

            // get paint or create default one
            let (paint, opacity) = match p.fill {
                Some(ref f) => (f.paint.clone(), f.opacity.value()),
                None => (resvg::tree::Paint::Color(FALLBACK_COLOR), 1.0),
            };

            // fall back to always use color fill
            let color = match paint {
                resvg::tree::Paint::Color(c) => c,
                _ => FALLBACK_COLOR,
            };

            let _ = fill_tess
                .tessellate_path(
                    convert_path(p).path_iter(),
                    &FillOptions::tolerance(0.01),
                    &mut BuffersBuilder::new(&mut mesh, VertexCtor::new(color, opacity)),
                )
                .expect("Error during tesselation!");

            if let Some(ref stroke) = p.stroke {
                let (stroke_color, stroke_opts) = convert_stroke(stroke);
                let opacity = stroke.opacity.value();
                let _ = stroke_tess.tessellate_path(
                    convert_path(p).path_iter(),
                    &stroke_opts.with_tolerance(0.01),
                    &mut BuffersBuilder::new(&mut mesh, VertexCtor::new(stroke_color, opacity)),
                );
            }
        }
    }
}
