#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_abi;

use rustc_driver::Compilation;
use rustc_hir::def::DefKind;
use rustc_interface::interface::Compiler;
use rustc_middle::ty::{TyCtxt, TypingEnv};
use rustc_abi::FieldsShape;
use rustc_abi::FieldIdx;

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
                println!("{:#?}", type_layout);

                let ty = tcx.type_of(def_id);
                let adt_def = ty.skip_binder().ty_adt_def().unwrap();
                let variants = adt_def.variants();
                let variant = variants.iter().next().unwrap();

                if let FieldsShape::Arbitrary{ offsets, memory_index } = type_layout.layout.fields() {
                for (layout_idx, src_idx) in memory_index.iter().enumerate() {
                    let idx = FieldIdx::from_u32(*src_idx);
                    let field_def = &variant.fields.get(idx).unwrap();
                    let name = field_def.ident(tcx).name;
                    let field_ty = tcx.type_of(field_def.did).skip_binder();
                    let offset = offsets.get(idx).unwrap();
                    let typing_env = TypingEnv::post_analysis(tcx, field_def.did);
                    let type_layout = tcx.layout_of(typing_env.as_query_input(field_ty)).unwrap();

                    println!(
                        "\n[{}] {}: {:?}, offset = {}, size = {}",
                        layout_idx,
                        name,
                        field_ty,
                        offset.bytes(),
                        type_layout.size.bytes()
                    );
                }
                }
            }
        }

        Compilation::Continue
    }
}

fn main() {
    let at_args = std::env::args_os()
        .into_iter()
        .skip(1)
        .map(|arg| arg.into_string().unwrap())
        .collect::<Vec<_>>();
    rustc_driver::run_compiler(&at_args, &mut CompilerCallbacks);
}
