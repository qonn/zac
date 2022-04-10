use crate::{ast, generator::context::InferedType};

use super::{context, fn_call};

pub fn generate(ctx: &mut context::Context, ast: &ast::MemberAccess) -> String {
    let mut results: Vec<(Vec<String>, ast::Expr, String)> = vec![];

    let mut result = "".to_string();

    scan(ctx, ast, vec![], &mut results);

    let mut results2: Vec<(String, Option<InferedType>)> = vec![];

    resolve_types(ctx, &mut results, &mut results2);

    let results = results
        .iter()
        .map(|(a, b, c)| (a.join("."), b, c))
        .collect::<Vec<_>>();

    for (idx, (path, item, res)) in results.iter().enumerate() {
        let (current_infered_path, current_infered_type) = &results2[idx];

        match item {
            ast::Expr::Id(v) => {
                let string = v.string.to_string();

                if string == "await" {
                    result = format!("(await {result})")
                } else {
                    if let Some(_) = ctx.find_mod(string.clone()) {
                    } else {
                        if idx == 0 {
                            result = string;
                        } else {
                            if let None = current_infered_type {
                            } else {
                                result = format!("{result}_{string}");
                            }
                        }
                    }
                }
            }

            ast::Expr::FnCall(v) => {
                let gen_fn = res;

                let (_, last_infered_type) = &results2[idx - 1];

                let mut found_method = if let Some(last_item) = last_infered_type {
                    if let Some((method_path, method_def)) =
                        ctx.find_method(v.id.string.to_string(), last_item.clone().into())
                    {
                        method_path.to_string()
                    } else {
                        v.id.string.to_string()
                    }
                } else {
                    v.id.string.to_string()
                };

                let to_replace = format!("{}(", v.id.string);
                let gen_fn = gen_fn.replace(&to_replace, "");
                let path = path.replace(".", "_");

                if let Some(v) = ctx.find_fn(format!("{path}_{found_method}")) {
                    found_method = format!("{path}_{found_method}");
                }

                let result_gen_fn = if gen_fn.trim().len() > 0 {
                    if gen_fn.trim() == ")" {
                        format!("{result}{gen_fn}")
                    } else {
                        if let None = last_infered_type {
                            format!("{result}{gen_fn}")
                        } else {
                            format!("{result}, {gen_fn}")
                        }
                    }
                } else {
                    result
                };

                result = format!("{found_method}({result_gen_fn}");
            }
            _ => panic!("Unsupported"),
        }
    }

    return result;
}

fn resolve_types(
    ctx: &mut context::Context,
    results: &mut Vec<(Vec<String>, ast::Expr, String)>,
    results2: &mut Vec<(String, Option<InferedType>)>,
) {
    for (idx, (path, item, res)) in results.iter().enumerate() {
        let path2 = path.join("_");

        let id = match item {
            ast::Expr::Id(v) => {
                if v.string == "await" {
                    v.string.clone().into()
                } else {
                    "".into()
                }
            }
            ast::Expr::FnCall(v) => v.id.string.to_string(),
            _ => panic!("Unsupported"),
        };

        let test = if id.len() > 0 {
            format!("{path2}_{id}")
        } else {
            format!("{path2}{id}")
        };

        let resolved_type: Option<InferedType> = if id == "await" {
            let (prev_path, prev_item) = &results2[idx - 1];

            if let Some(prev_item) = prev_item {
                if prev_item.id == "Promise" {
                    Some(InferedType {
                        id: prev_item.generics[0].to_string(),
                        generics: vec![],
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else if let Some(fn_def) = ctx.fn_defs.get(&test) {
            Some(fn_def.output.clone().into())
        } else if let Some(var_def) = ctx.resolved_type_defs.get(&test) {
            Some(var_def.clone())
        } else if let Some(var_def) = ctx.resolved_type_defs.get(&id) {
            Some(var_def.clone())
        } else if let Some((prev_path, prev_item)) =
            &results2.get(if idx > 0 { idx - 1 } else { idx })
        {
            if let Some(prev_item) = prev_item {
                if let Some((path, method)) = ctx.find_method(id.clone(), prev_item.clone().into())
                {
                    Some(method.output.clone().into())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        results2.push((test, resolved_type));
    }
}

fn scan(
    ctx: &mut context::Context,
    ast: &ast::MemberAccess,
    path: Vec<String>,
    results: &mut Vec<(Vec<String>, ast::Expr, String)>,
) {
    let mut path = path;

    let obj = match &ast.obj {
        ast::Expr::Id(v) => {
            if v.string != "await" {
                path = path
                    .into_iter()
                    .chain(vec![v.string.clone()].into_iter())
                    .collect::<Vec<_>>();
            }
            v.string.to_string()
        }
        ast::Expr::FnCall(v) => format!("{}", fn_call::generate(ctx, v)),
        _ => panic!("Unsupported"),
    };

    let prop = match &ast.prop {
        ast::Expr::Id(v) => v.string.to_string(),
        ast::Expr::FnCall(v) => format!("{}", fn_call::generate(ctx, v)),
        ast::Expr::MemberAccess(v) => {
            scan(ctx, v, path.clone(), results);
            "".into()
        }
        _ => panic!("Unsupported"),
    };

    if prop.len() > 0 {
        results.insert(0, (path.clone(), ast.prop.clone(), prop));
    }

    if obj.len() > 0 {
        results.insert(0, (path.clone(), ast.obj.clone(), obj));
    }
}
