use crate::utils::cast_enum;
use crate::CommandRunner;

use anyhow::{Context, Ok};
use clap::Parser as ClapParser;
use std::io::Read;
use swc_ecmascript::ast::{EsVersion, FnDecl, ModuleItem, Program, Stmt};

use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use swc::config::{Config, ExperimentalOptions, JscConfig, ModuleConfig, TransformConfig};
use swc::config::{IsModule, Options};
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, SourceMap,
};
use swc_ecma_parser::{Syntax, TsConfig};

use crate::commands::*;

pub struct FindConstFuncs {
    pub module_path: String,
}

impl CommandRunner<Vec<String>> for FindConstFuncs {
    fn execute(&self) -> anyhow::Result<Vec<String>> {
        let cm = Arc::<SourceMap>::default();
        let compiler = swc::Compiler::new(cm.clone());
        let opts = get_compiler_opts();

        let mut source_file = File::open(&self.module_path).expect("File doesnt exist");
        let mut buf = String::new();
        source_file
            .read_to_string(&mut buf)
            .expect("Failed to read file");

        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
        let fm = cm.new_source_file(FileName::Custom("test.js".into()), buf);

        let program = compiler
            .parse_js(
                fm,
                &handler,
                EsVersion::latest(),
                opts.config.jsc.syntax.unwrap(),
                IsModule::Bool(true),
                None,
            )
            .unwrap();

        match process_program(&program) {
            Some(funcs) => Ok(funcs),
            None => Ok(vec![]),
        }

        // let result = compiler.process_js_file(fm, &handler, &opts).unwrap();

        // println!("Executing!");
        //
        // let out_path = Path::new("output.js");
        //
        // std::fs::write(out_path, result.code).unwrap();
    }
}

fn process_program<'a>(program: &'a Program) -> Option<Vec<String>> {
    if let Program::Script(_) = program {
        return None;
    }

    let mut const_funcs = vec![];

    let module = cast_enum!(program, Program::Module);

    let get_fn_decl = |stmt: &'a Stmt| -> anyhow::Result<&FnDecl> {
        let fn_decl = stmt.as_decl().context("")?.as_fn_decl().context("")?;

        Ok(fn_decl)
    };

    for body_item in &module.body {
        if !body_item.is_stmt() {
            continue;
        }

        let fn_decl = get_fn_decl(body_item.as_stmt().unwrap());
        if fn_decl.is_err() {
            continue;
        }
        let fn_decl = fn_decl.unwrap();
        println!("FN DECL!");

        if fn_decl.ident.sym.starts_with("__const__") {
            const_funcs.push(fn_decl.ident.sym.to_string());
        }
    }

    Some(const_funcs)
}

pub fn get_compiler_opts() -> Options {
    Options {
        config: Config {
            env: Some(Default::default()),
            test: None,
            exclude: None,
            jsc: JscConfig {
                syntax: Some(Syntax::Typescript(TsConfig {
                    tsx: false,
                    decorators: true,
                    no_early_errors: true,
                    dts: true,
                })),
                target: Some(EsVersion::latest()),
                transform: Some(TransformConfig {
                    ..Default::default()
                })
                .into(),
                ..Default::default()
            },
            module: Some(ModuleConfig::CommonJs(
                swc_ecma_transforms::modules::common_js::Config {
                    strict: true,
                    ..Default::default()
                },
            )),
            minify: false.into(),
            input_source_map: None,
            source_maps: None,
            inline_sources_content: false.into(),
            emit_source_map_columns: Default::default(),
            error: Default::default(),
            is_module: Default::default(),
            schema: None,
        },
        skip_helper_injection: true.into(),
        disable_hygiene: true.into(),
        disable_fixer: true.into(),
        top_level_mark: None,
        cwd: Default::default(),
        caller: None,
        filename: "".to_string(),
        config_file: None,
        root: None,
        root_mode: Default::default(),
        swcrc: false,
        swcrc_roots: None,
        env_name: "".to_string(),
        source_maps: None,
        source_file_name: None,
        source_root: None,
        output_path: None,
        experimental: ExperimentalOptions { error_format: None },
    }
}
