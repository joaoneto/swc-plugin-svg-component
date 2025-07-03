use swc_common::DUMMY_SP;
use swc_core::{
    ecma::ast::*,
    ecma::visit::{VisitMut, VisitMutWith},
};

use crate::util::to_camel_case;

fn concat_namespace(ns: &str, name: &str) -> String {
    format!("{}{}", ns, name[..1].to_ascii_uppercase() + &name[1..])
}

fn style_to_obj(style: &str) -> ObjectLit {
    let props = style.split(';').filter_map(|decl| {
        let mut parts = decl.splitn(2, ':');
        let key = parts.next()?.trim();
        let value = parts.next()?.trim();

        let lit = if let Ok(num) = value.parse::<f64>() {
            Lit::Num(Number { span: DUMMY_SP, value: num, raw: None })
        } else {
            Lit::Str(Str { span: DUMMY_SP, value: value.into(), raw: None })
        };

        Some(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(
                IdentName::new(
                    to_camel_case(key, Default::default()).into(),
                    DUMMY_SP
                )
            ),
            value: Box::new(Expr::Lit(lit)),
        }))))
    }).collect();

    ObjectLit { span: DUMMY_SP, props }
}

pub struct TransformSvgComponentVisitor {
    pub jsx_svg_element: Option<JSXElement>,
}

impl TransformSvgComponentVisitor {
    pub fn new() -> Self {
        TransformSvgComponentVisitor {
            jsx_svg_element: None,
        }
    }

    pub fn create_component_body(&self) -> Vec<ModuleItem> {
        if self.jsx_svg_element.is_none() {
            return vec![];
        }

        let mut jsx = self.jsx_svg_element.as_ref().unwrap().clone();

        if let JSXElementName::Ident(_) = &jsx.opening.name {
            let spread = JSXAttrOrSpread::SpreadElement(SpreadElement {
                expr: Box::new(Expr::Ident(Ident::new(
                    "props".into(),
                    DUMMY_SP,
                    Default::default(),
                ))),
                ..Default::default()
            });
            let mut new_attrs = vec![spread];
            new_attrs.extend(jsx.opening.attrs.clone());
            jsx.opening.attrs = new_attrs;
        }

        let component_props = vec![Param {
            span: DUMMY_SP,
            decorators: vec![],
            pat: Pat::Ident(BindingIdent {
                id: Ident::new("props".into(), DUMMY_SP, Default::default()),
                type_ann: None,
            }),
        }];

        let component_decl = FnDecl {
            ident: Ident::new("SvgComponent".into(), DUMMY_SP, Default::default()),
            declare: false,
            function: Box::new(Function {
                span: DUMMY_SP,
                is_generator: false,
                is_async: false,
                params: component_props,
                decorators: vec![],
                ctxt: Default::default(),
                body: Some(BlockStmt {
                    span: DUMMY_SP,
                    ctxt: Default::default(),
                    stmts: vec![Stmt::Return(ReturnStmt {
                        span: DUMMY_SP,
                        arg: Some(Box::new(Expr::JSXElement(Box::new(jsx.clone())))),
                    })],
                }),
                type_params: None,
                return_type: None,
            }),
        };

        let mut body_stmts: Vec<ModuleItem> = Vec::new();

        body_stmts.push(ModuleItem::Stmt(Stmt::Decl(Decl::Fn(
            component_decl,
        ))));

        body_stmts
    }

    pub fn create_exports(&self) -> Vec<ModuleItem> {
        vec![ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(
            ExportDefaultExpr {
                span: DUMMY_SP,
                expr: Box::new(Expr::Ident(Ident::new(
                    "SvgComponent".into(),
                    DUMMY_SP,
                    Default::default()
                ))),
            },
        ))]
    }
}

impl VisitMut for TransformSvgComponentVisitor {
    fn visit_mut_jsx_attr(&mut self, jsx_attr: &mut JSXAttr) {
        match jsx_attr.name.clone() {
            JSXAttrName::JSXNamespacedName(JSXNamespacedName { ns, name, .. }) => {
                jsx_attr.name = JSXAttrName::Ident(IdentName::new(
                    concat_namespace(&ns.sym, &name.sym).into(),
                    DUMMY_SP
                ));
            }
            JSXAttrName::Ident(ident) => {
                let sym = ident.sym;
                if sym == *"class" {
                    jsx_attr.name = JSXAttrName::Ident(IdentName::new("className".into(), DUMMY_SP));
                    return;
                }

                if sym == *"style" {
                    if let Some(JSXAttrValue::Lit(Lit::Str(Str { value, .. }))) =
                        jsx_attr.value.clone()
                    {
                        jsx_attr.value = Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                            span: DUMMY_SP,
                            expr: JSXExpr::Expr(Box::new(Expr::Object(style_to_obj(&value)))),
                        }));
                    }
                    return;
                }

                if sym.contains('-') {
                    let camel = to_camel_case(&sym, Default::default());
                    jsx_attr.name = JSXAttrName::Ident(IdentName::new(camel.into(), DUMMY_SP));
                }
            }
        };
    }

    fn visit_mut_jsx_element(&mut self, jsx_element: &mut JSXElement) {
        jsx_element.visit_mut_children_with(self);

        if let JSXElementName::Ident(tag_name) = &jsx_element.opening.name {
            if *tag_name.sym == *"svg" || *tag_name.sym == *"Svg" {
                self.jsx_svg_element = Some(jsx_element.clone());
            }
        }
    }

    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);
    
        if self.jsx_svg_element.is_some() {
            let mut body = vec![];

            body.extend(self.create_component_body());
            body.extend(self.create_exports());

            module.body = body;
        }
    }
}
