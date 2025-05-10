#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;

use rustc_driver::Compilation;
use rustc_hir::def::DefKind;
use rustc_interface::interface::Compiler;
use rustc_middle::ty::{TyCtxt, TypingEnv};

struct CompilerCallbacks;

impl rustc_driver::Callbacks for CompilerCallbacks {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        let items = tcx.hir_crate_items(());

        for item in items.free_items() {
            let hir_id = item.owner_id;
            let def_id = hir_id.to_def_id();
            let def_kind = tcx.def_kind(def_id);
            if def_kind == DefKind::Struct {
                let ty = tcx.type_of(def_id).instantiate_identity();
                let typing_env = TypingEnv::post_analysis(tcx, def_id);
                let type_layout = tcx.layout_of(typing_env.as_query_input(ty)).unwrap();
                dbg!(type_layout);
            }
        }

        Compilation::Continue
    }
}

fn main() {
    print!("Hello from your RUSTC wrapper");

    let at_args = std::env::args_os()
        .into_iter()
        .skip(1)
        .map(|arg| arg.into_string().unwrap())
        .collect::<Vec<_>>();
    rustc_driver::run_compiler(&at_args, &mut CompilerCallbacks);
}
