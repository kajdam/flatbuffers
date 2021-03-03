// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, TagUnionValueOffset};

#[allow(unused_imports, dead_code)]
pub mod namespace_a {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, TagUnionValueOffset};
#[allow(unused_imports, dead_code)]
pub mod namespace_b {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, TagUnionValueOffset};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_UNION_IN_NESTED_NS: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_UNION_IN_NESTED_NS: u8 = 1;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_UNION_IN_NESTED_NS: [UnionInNestedNS; 2] = [
  UnionInNestedNS::NONE,
  UnionInNestedNS::TableInNestedNS,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct UnionInNestedNS(pub u8);
#[allow(non_upper_case_globals)]
impl UnionInNestedNS {
  pub const NONE: Self = Self(0);
  pub const TableInNestedNS: Self = Self(1);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 1;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::TableInNestedNS,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::TableInNestedNS => Some("TableInNestedNS"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for UnionInNestedNS {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for UnionInNestedNS {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for UnionInNestedNS {
    type Output = UnionInNestedNS;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for UnionInNestedNS {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = u8::to_le(self.0);
    Self(b)
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let b = u8::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for UnionInNestedNS {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for UnionInNestedNS {}

pub struct UnionInNestedNSUnionTableOffset {}

impl flatbuffers::TaggedUnion for UnionInNestedNSUnionTableOffset {
  type Tag = UnionInNestedNS;
}

impl<'a> flatbuffers::TagUnionValueOffset<TableInNestedNS<'a>> for UnionInNestedNSUnionTableOffset {
  fn from_value_offset(
    o: flatbuffers::WIPOffset<TableInNestedNS<'a>>,
  ) -> flatbuffers::TaggedWIPOffset<Self> {
    flatbuffers::TaggedWIPOffset{ tag: UnionInNestedNS::TableInNestedNS, value: flatbuffers::WIPOffset::new(o.value()) }
  }
}

impl<'a> flatbuffers::UnionVerifiable<'a> for UnionInNestedNSUnionTableOffset {
  fn run_union_verifier(
    v: &mut flatbuffers::Verifier,
    tag: <<Self as flatbuffers::TaggedUnion>::Tag as flatbuffers::Follow<'a>>::Inner,
    pos: usize,
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    match tag {
      UnionInNestedNS::TableInNestedNS => v
        .verify_union_variant::<flatbuffers::ForwardsUOffset<TableInNestedNS>>(
          "UnionInNestedNS::TableInNestedNS",
          pos,
        ),
      _ => Ok(()),
    }
  }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum UnionInNestedNST {
  NONE,
  TableInNestedNS(Box<TableInNestedNST>),
}
impl Default for UnionInNestedNST {
  fn default() -> Self {
    Self::NONE
  }
}
impl UnionInNestedNST {
  pub fn union_in_nested_ns_type(&self) -> UnionInNestedNS {
    match self {
      Self::NONE => UnionInNestedNS::NONE,
      Self::TableInNestedNS(_) => UnionInNestedNS::TableInNestedNS,
    }
  }
  pub fn pack(&self, fbb: &mut flatbuffers::FlatBufferBuilder) -> Option<flatbuffers::WIPOffset<UnionInNestedNSUnionTableOffset>> {
    match self {
      Self::NONE => None,
      Self::TableInNestedNS(v) => Some(UnionInNestedNSUnionTableOffset::from_value_offset(v.pack(fbb)).value),
    }
  }
  /// If the union variant matches, return the owned TableInNestedNST, setting the union to NONE.
  pub fn take_table_in_nested_ns(&mut self) -> Option<Box<TableInNestedNST>> {
    if let Self::TableInNestedNS(_) = self {
      let v = std::mem::replace(self, Self::NONE);
      if let Self::TableInNestedNS(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the TableInNestedNST.
  pub fn as_table_in_nested_ns(&self) -> Option<&TableInNestedNST> {
    if let Self::TableInNestedNS(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the TableInNestedNST.
  pub fn as_table_in_nested_ns_mut(&mut self) -> Option<&mut TableInNestedNST> {
    if let Self::TableInNestedNS(v) = self { Some(v.as_mut()) } else { None }
  }
}
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_ENUM_IN_NESTED_NS: i8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_ENUM_IN_NESTED_NS: i8 = 2;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_ENUM_IN_NESTED_NS: [EnumInNestedNS; 3] = [
  EnumInNestedNS::A,
  EnumInNestedNS::B,
  EnumInNestedNS::C,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct EnumInNestedNS(pub i8);
#[allow(non_upper_case_globals)]
impl EnumInNestedNS {
  pub const A: Self = Self(0);
  pub const B: Self = Self(1);
  pub const C: Self = Self(2);

  pub const ENUM_MIN: i8 = 0;
  pub const ENUM_MAX: i8 = 2;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::A,
    Self::B,
    Self::C,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::A => Some("A"),
      Self::B => Some("B"),
      Self::C => Some("C"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for EnumInNestedNS {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for EnumInNestedNS {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for EnumInNestedNS {
    type Output = EnumInNestedNS;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<i8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for EnumInNestedNS {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = i8::to_le(self.0);
    Self(b)
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let b = i8::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for EnumInNestedNS {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for EnumInNestedNS {}

// struct StructInNestedNS, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Default)]
pub struct StructInNestedNS(pub [u8; 8]);
impl std::fmt::Debug for StructInNestedNS {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("StructInNestedNS")
      .field("a", &self.a())
      .field("b", &self.b())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for StructInNestedNS {}
impl flatbuffers::SafeSliceAccess for StructInNestedNS {}
impl<'a> flatbuffers::Follow<'a> for StructInNestedNS {
  type Inner = &'a StructInNestedNS;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a StructInNestedNS>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a StructInNestedNS {
  type Inner = &'a StructInNestedNS;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<StructInNestedNS>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for StructInNestedNS {
    type Output = StructInNestedNS;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const StructInNestedNS as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b StructInNestedNS {
    type Output = StructInNestedNS;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const StructInNestedNS as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for StructInNestedNS {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl StructInNestedNS {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    a: i32,
    b: i32,
  ) -> Self {
    let mut s = Self([0; 8]);
    s.set_a(a);
    s.set_b(b);
    s
  }

    pub const fn get_fully_qualified_name() -> &'static str {
        "NamespaceA.NamespaceB.StructInNestedNS"
    }

  pub fn a(&self) -> i32 {
    let mut mem = core::mem::MaybeUninit::<i32>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i32>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_a(&mut self, x: i32) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i32 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<i32>(),
      );
    }
  }

  pub fn b(&self) -> i32 {
    let mut mem = core::mem::MaybeUninit::<i32>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i32>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_b(&mut self, x: i32) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i32 as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<i32>(),
      );
    }
  }

  pub fn unpack(&self) -> StructInNestedNST {
    StructInNestedNST {
      a: self.a(),
      b: self.b(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StructInNestedNST {
  pub a: i32,
  pub b: i32,
}
impl StructInNestedNST {
  pub fn pack(&self) -> StructInNestedNS {
    StructInNestedNS::new(
      self.a,
      self.b,
    )
  }
}

pub enum TableInNestedNSOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TableInNestedNS<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TableInNestedNS<'a> {
    type Inner = TableInNestedNS<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> TableInNestedNS<'a> {
    pub const fn get_fully_qualified_name() -> &'static str {
        "NamespaceA.NamespaceB.TableInNestedNS"
    }

    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TableInNestedNS { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TableInNestedNSArgs) -> flatbuffers::WIPOffset<TableInNestedNS<'bldr>> {
      let mut builder = TableInNestedNSBuilder::new(_fbb);
      builder.add_foo(args.foo);
      builder.finish()
    }

    pub fn unpack(&self) -> TableInNestedNST {
      let foo = self.foo();
      TableInNestedNST {
        foo,
      }
    }
    pub const VT_FOO: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn foo(&self) -> i32 {
    self._tab.get::<i32>(TableInNestedNS::VT_FOO, Some(0)).unwrap()
  }
}

impl flatbuffers::Verifiable for TableInNestedNS<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>(&"foo", Self::VT_FOO, false)?
     .finish();
    Ok(())
  }
}
pub struct TableInNestedNSArgs {
    pub foo: i32,
}
impl<'a> Default for TableInNestedNSArgs {
    #[inline]
    fn default() -> Self {
        TableInNestedNSArgs {
            foo: 0,
        }
    }
}
pub struct TableInNestedNSBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TableInNestedNSBuilder<'a, 'b> {
  #[inline]
  pub fn add_foo(&mut self, foo: i32) {
    self.fbb_.push_slot::<i32>(TableInNestedNS::VT_FOO, foo, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TableInNestedNSBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TableInNestedNSBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TableInNestedNS<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for TableInNestedNS<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("TableInNestedNS");
      ds.field("foo", &self.foo());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct TableInNestedNST {
  pub foo: i32,
}
impl Default for TableInNestedNST {
  fn default() -> Self {
    Self {
      foo: 0,
    }
  }
}
impl TableInNestedNST {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<TableInNestedNS<'b>> {
    let foo = self.foo;
    TableInNestedNS::create(_fbb, &TableInNestedNSArgs{
      foo,
    })
  }
}
}  // pub mod NamespaceB
}  // pub mod NamespaceA

