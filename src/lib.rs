use swc_core::{
    ecma::ast::Program,
    ecma::visit::VisitMutWith,
    plugin::{metadata::TransformPluginProgramMetadata, plugin_transform},
};

use crate::transform::svg_component_visitor::TransformSvgComponentVisitor;

pub(crate) mod util;
pub mod transform;

#[plugin_transform]
pub fn process_transform(mut program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.visit_mut_with(&mut TransformSvgComponentVisitor::new());

    program
}
