Dumped error:
============

```
Initializing (look for `progress[done:true]` message)...
> 1: InitializeResult {
    capabilities: ServerCapabilities {
        text_document_sync: Some(
            Kind(
                Incremental
            )
        ),
        hover_provider: Some(
            true
        ),
        completion_provider: Some(
            CompletionOptions {
                resolve_provider: Some(
                    true
                ),
                trigger_characters: Some(
                    [
                        ".",
                        ":"
                    ]
                )
            }
        ),
        signature_help_provider: None,
        definition_provider: Some(
            true
        ),
        type_definition_provider: None,
        implementation_provider: Some(
            Simple(
                true
            )
        ),
        references_provider: Some(
            true
        ),
        document_highlight_provider: Some(
            true
        ),
        document_symbol_provider: Some(
            true
        ),
        workspace_symbol_provider: Some(
            true
        ),
        code_action_provider: Some(
            true
        ),
        code_lens_provider: Some(
            CodeLensOptions {
                resolve_provider: Some(
                    false
                )
            }
        ),
        document_formatting_provider: Some(
            true
        ),
        document_range_formatting_provider: Some(
            false
        ),
        document_on_type_formatting_provider: None,
        rename_provider: Some(
            true
        ),
        color_provider: None,
        execute_command_provider: Some(
            ExecuteCommandOptions {
                commands: [
                    "rls.applySuggestion-5657",
                    "rls.deglobImports-5657"
                ]
            }
        )
    }
}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"version_check","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"nodrop","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"cfg_if","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"memoffset","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"scopeguard","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"libc","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"matches","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"build_script_build","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"gcc","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"build_script_build","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"build_script_build","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"cfg_if","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"yansi","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"unicode_normalization","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"matches","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"percent_encoding","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"byteorder","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"unicode_normalization","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"untrusted","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"safemem","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"safemem","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"lazy_static","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"void","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"traitobject","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"libc","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"untrusted","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"lazy_static","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"typeable","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"percent_encoding","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"safemem","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"language_tags","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"void","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"safemem","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"byteorder","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"state","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"ordermap","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"language_tags","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"typeable","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"pear","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"traitobject","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"state","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"ordermap","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"pear","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"yansi","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"crossbeam_utils","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"log","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"arrayvec","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"unicode_bidi","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"log","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"build_script_build","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"build_script_build","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"build_script_build","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"unicode_bidi","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"build_script_build","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"build_script_build","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"build_script_build","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"num_cpus","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"time","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"isatty","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"base64","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"base64","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"unreachable","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"unreachable","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"base64","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"time","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"num_cpus","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"base64","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"isatty","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"log","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"log","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"serde","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"serde","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"httparse","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"httparse","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"smallvec","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"smallvec","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"unicase","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"unicase","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"lazy_static","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"mime","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"mime","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"memchr","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"memchr","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"pear_codegen","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"crossbeam_epoch","title":"Building"}}
thread 'main' panicked at 'No span found for use glob', libcore/option.rs:1008:5
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at libstd/sys_common/backtrace.rs:59
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:480
   6: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:390
   7: rust_begin_unwind
             at libstd/panicking.rs:325
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:77
   9: core::option::expect_failed
             at libcore/option.rs:1008
  10: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O>>::process_use_tree
  11: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
  12: syntax::visit::walk_expr
  13: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_local
  14: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_expr
  15: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_expr
  16: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
  17: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_mod
  18: <rustc_save_analysis::DumpHandler<'a> as rustc_save_analysis::SaveHandler>::save
  19: rustc::ty::context::tls::with_context
  20: rustc_save_analysis::process_crate
  21: rustc_driver::enable_save_analysis::{{closure}}::{{closure}}
  22: rustc::util::common::time
  23: rustc_driver::enable_save_analysis::{{closure}}
  24: rustc::ty::context::tls::with_context
  25: rustc_driver::driver::compile_input::{{closure}}
  26: rustc::ty::context::tls::enter_context
  27: <std::thread::local::LocalKey<T>>::with
  28: rustc::ty::context::TyCtxt::create_and_enter
  29: rustc_driver::driver::compile_input
  30: rustc_driver::run_compiler_with_pool
  31: rustc_driver::driver::spawn_thread_pool
  32: rustc_driver::run_compiler
  33: <scoped_tls::ScopedKey<T>>::set
  34: syntax::with_globals
  35: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  36: rustc_driver::run
  37: rls_rustc::run
  38: rls::main_inner
  39: rls::main
  40: std::rt::lang_start::{{closure}}
  41: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  42: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  43: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  44: main
  45: __libc_start_main
  46: <unknown>
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.31.0-nightly (d586d5d2f 2018-10-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -C prefer-dynamic -C debuginfo=2 --crate-type dylib

note: some of the compiler flags provided by cargo are hidden
```