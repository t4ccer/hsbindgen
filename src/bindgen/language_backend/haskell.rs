use crate::bindgen::ir::{
    Documentation, Enum, Function, IntKind, ItemContainer, Literal, OpaqueItem, PrimitiveType,
    Static, Struct, Type, Typedef, Union,
};
use crate::bindgen::language_backend::LanguageBackend;
use crate::bindgen::writer::SourceWriter;
use crate::bindgen::{Config, Layout};
use crate::Bindings;

pub struct HaskellLanguageBackend<'a> {
    config: &'a Config,
}

impl<'a> HaskellLanguageBackend<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    fn write_type_with_io<W: std::io::Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        t: &Type,
        in_io: bool,
    ) {
        if in_io {
            write!(out, "IO (");
        }

        match dbg!(t) {
            Type::Ptr {
                ty,
                is_const: _,
                is_nullable: _,
                is_ref: _,
            } => {
                write!(out, "Ptr (");
                self.write_type_with_io(out, ty.as_ref(), false);
                write!(out, ")");
            }
            Type::Path(generic_path) => {
                write!(out, "{}", generic_path.path().name());
            }
            Type::Primitive(primitive_type) => match primitive_type {
                PrimitiveType::Void => write!(out, "()"),
                PrimitiveType::Bool => write!(out, "CBool"),
                PrimitiveType::Char => write!(out, "CChar"),
                PrimitiveType::SChar => write!(out, "CSChar"),
                PrimitiveType::UChar => write!(out, "CUChar"),
                PrimitiveType::Char32 => write!(out, "Word32"),
                PrimitiveType::Float => write!(out, "CFloat"),
                PrimitiveType::Double => write!(out, "CDouble"),
                PrimitiveType::VaList => unimplemented!(),
                PrimitiveType::PtrDiffT => write!(out, "CPtrdiff"),
                PrimitiveType::Integer {
                    zeroable,
                    signed,
                    kind,
                } => {
                    assert!(zeroable);
                    if *signed {
                        match kind {
                            IntKind::Short => write!(out, "CShort"),
                            IntKind::Int => write!(out, "CInt"),
                            IntKind::Long => write!(out, "CLong"),
                            IntKind::LongLong => write!(out, "CLLong"),
                            IntKind::SizeT => write!(out, "CSize"),
                            IntKind::Size => write!(out, "CSize"),
                            IntKind::B8 => write!(out, "Int8"),
                            IntKind::B16 => write!(out, "Int16"),
                            IntKind::B32 => write!(out, "Int32"),
                            IntKind::B64 => write!(out, "Int64"),
                        }
                    } else {
                        match kind {
                            IntKind::Short => write!(out, "CUShort"),
                            IntKind::Int => write!(out, "CUInt"),
                            IntKind::Long => write!(out, "CULong"),
                            IntKind::LongLong => write!(out, "CULLong"),
                            IntKind::SizeT => write!(out, "CUSize"),
                            IntKind::Size => write!(out, "CUSize"),
                            IntKind::B8 => write!(out, "Word8"),
                            IntKind::B16 => write!(out, "Word16"),
                            IntKind::B32 => write!(out, "Word32"),
                            IntKind::B64 => write!(out, "Word64"),
                        }
                    }
                }
            },
            Type::Array(_, const_expr) => todo!(),
            Type::FuncPtr {
                ret,
                args,
                is_nullable: _,
                never_return: _,
            } => {
                write!(out, "FunPtr (");
                for (_, arg) in args.iter() {
                    self.write_type_with_io(out, arg, false);
                    write!(out, " -> ");
                }
                self.write_type_with_io(out, ret.as_ref(), true);
                write!(out, ")");
            }
        }

        if in_io {
            write!(out, ")");
        }
    }
}

impl<'a> LanguageBackend for HaskellLanguageBackend<'a> {
    fn open_namespaces<W: std::io::Write>(&mut self, _out: &mut SourceWriter<W>) {}

    fn close_namespaces<W: std::io::Write>(&mut self, _out: &mut SourceWriter<W>) {}

    fn write_headers<W: std::io::Write>(&self, out: &mut SourceWriter<W>, _package_version: &str) {
        if self.config.include_version {
            out.new_line_if_not_start();
            write!(
                out,
                "-- Generated with cbindgen:{}",
                crate::bindgen::config::VERSION
            );
            out.new_line();
        }

        write!(out, "{{-# OPTIONS_GHC -Wno-missing-import-lists -Wno-unused-imports -Wno-missing-export-lists -Wwarn #-}}");
        out.new_line();

        if let Some(namespace) = self.config.namespace.as_ref() {
            write!(out, "module {} where", namespace);
            out.new_line();
        }

        if !&self.config.no_includes {
            out.write("import Foreign.Ptr");
            out.new_line();
            out.write("import Data.Word");
            out.new_line();
            out.write("import Data.Int");
            out.new_line();
            out.write("import Foreign.C.String");
            out.new_line();
            out.write("import Foreign.C.Types");
            out.new_line();
        }
    }

    fn write_footers<W: std::io::Write>(&mut self, _out: &mut SourceWriter<W>) {}

    fn write_enum<W: std::io::Write>(&mut self, out: &mut SourceWriter<W>, e: &Enum) {
        todo!()
    }

    fn write_struct<W: std::io::Write>(&mut self, out: &mut SourceWriter<W>, s: &Struct) {
        todo!()
    }

    fn write_union<W: std::io::Write>(&mut self, out: &mut SourceWriter<W>, u: &Union) {
        todo!()
    }

    fn write_opaque_item<W: std::io::Write>(&mut self, out: &mut SourceWriter<W>, o: &OpaqueItem) {
        write!(out, "data {}", o.export_name);
        out.new_line();
    }

    fn write_type_def<W: std::io::Write>(&mut self, out: &mut SourceWriter<W>, t: &Typedef) {
        write!(out, "type {} = ", t.export_name);
        self.write_type(out, &t.aliased);
    }

    fn write_static<W: std::io::Write>(&mut self, out: &mut SourceWriter<W>, s: &Static) {
        todo!()
    }

    fn write_function<W: std::io::Write>(
        &mut self,
        config: &Config,
        out: &mut SourceWriter<W>,
        f: &Function,
    ) {
        write!(out, "foreign import ccall \"{}\"", f.path.name());
        out.new_line();
        write!(out, "  {} :: ", f.path.name());
        for arg in &f.args {
            self.write_type(out, &arg.ty);
            write!(out, " -> ");
        }
        self.write_type_with_io(out, &f.ret, true);
        out.new_line();

        out.new_line();

        write!(out, "foreign import ccall \"&{}\"", f.path.name());
        out.new_line();
        write!(out, "  {}_p :: FunPtr (", f.path.name());
        for arg in &f.args {
            self.write_type(out, &arg.ty);
            write!(out, " -> ");
        }
        self.write_type_with_io(out, &f.ret, true);
        write!(out, ")");
        out.new_line();
    }

    fn write_function_with_layout<W: std::io::Write>(
        &mut self,
        config: &Config,
        out: &mut SourceWriter<W>,
        func: &Function,
        _layout: Layout,
    ) {
        self.write_function(config, out, func);
    }

    fn write_type<W: std::io::Write>(&mut self, out: &mut SourceWriter<W>, t: &Type) {
        self.write_type_with_io(out, t, false);
    }

    fn write_documentation<W: std::io::Write>(
        &mut self,
        _out: &mut SourceWriter<W>,
        _d: &Documentation,
    ) {
    }

    fn write_literal<W: std::io::Write>(&mut self, out: &mut SourceWriter<W>, l: &Literal) {
        todo!()
    }

    fn write_bindings<W: std::io::Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        self.write_headers(out, &b.package_version);
        self.open_namespaces(out);
        self.write_primitive_constants(out, b);
        self.write_items(out, b);
        self.write_non_primitive_constants(out, b);
        self.write_globals(out, b);
        self.write_functions(out, b);
        self.close_namespaces(out);
        self.write_footers(out);
        self.write_trailer(out, b);
    }

    fn write_primitive_constants<W: std::io::Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        b: &Bindings,
    ) {
        for constant in &b.constants {
            if constant.uses_only_primitive_types() {
                out.new_line_if_not_start();
                constant.write(&b.config, self, out, None);
                out.new_line();
            }
        }
    }

    fn write_struct_or_typedef<W: std::io::Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        s: &Struct,
        b: &Bindings,
    ) {
        if let Some(typedef) = s.as_typedef() {
            self.write_type_def(out, &typedef);
            for constant in &s.associated_constants {
                out.new_line();
                constant.write(&b.config, self, out, Some(s));
            }
        } else {
            self.write_struct(out, s);
        }
    }

    fn write_items<W: std::io::Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        for item in &b.items {
            if item
                .deref()
                .annotations()
                .bool("no-export")
                .unwrap_or(false)
            {
                continue;
            }

            out.new_line_if_not_start();
            match *item {
                ItemContainer::Constant(..) => std::unreachable!(),
                ItemContainer::Static(..) => std::unreachable!(),
                ItemContainer::Enum(ref x) => self.write_enum(out, x),
                ItemContainer::Struct(ref x) => self.write_struct_or_typedef(out, x, b),
                ItemContainer::Union(ref x) => self.write_union(out, x),
                ItemContainer::OpaqueItem(ref x) => self.write_opaque_item(out, x),
                ItemContainer::Typedef(ref x) => self.write_type_def(out, x),
            }
            out.new_line();
        }
    }

    fn write_non_primitive_constants<W: std::io::Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        b: &crate::Bindings,
    ) {
        for constant in &b.constants {
            if !constant.uses_only_primitive_types() {
                out.new_line_if_not_start();
                constant.write(&b.config, self, out, None);
                out.new_line();
            }
        }
    }

    fn write_globals<W: std::io::Write>(&mut self, out: &mut SourceWriter<W>, b: &crate::Bindings) {
        self.write_globals_default(out, b)
    }

    fn write_globals_default<W: std::io::Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        b: &crate::Bindings,
    ) {
        for global in &b.globals {
            out.new_line_if_not_start();
            self.write_static(out, global);
            out.new_line();
        }
    }

    fn write_functions<W: std::io::Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        b: &crate::Bindings,
    ) {
        self.write_functions_default(out, b)
    }

    fn write_functions_default<W: std::io::Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        b: &crate::Bindings,
    ) {
        for function in &b.functions {
            out.new_line_if_not_start();
            self.write_function(&b.config, out, function);
            out.new_line();
        }
    }

    fn write_trailer<W: std::io::Write>(&mut self, out: &mut SourceWriter<W>, b: &crate::Bindings) {
        if let Some(ref f) = b.config.trailer {
            out.new_line_if_not_start();
            std::write!(out, "{}", f);
            if !f.ends_with('\n') {
                out.new_line();
            }
        }
    }
}
