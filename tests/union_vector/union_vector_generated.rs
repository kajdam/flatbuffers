// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow, TaggedUnion};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_CHARACTER: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_CHARACTER: u8 = 6;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_CHARACTER: [Character; 7] = [
  Character::NONE,
  Character::MuLan,
  Character::Rapunzel,
  Character::Belle,
  Character::BookFan,
  Character::Other,
  Character::Unused,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Character(pub u8);
#[allow(non_upper_case_globals)]
impl Character {
  pub const NONE: Self = Self(0);
  pub const MuLan: Self = Self(1);
  pub const Rapunzel: Self = Self(2);
  pub const Belle: Self = Self(3);
  pub const BookFan: Self = Self(4);
  pub const Other: Self = Self(5);
  pub const Unused: Self = Self(6);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 6;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::MuLan,
    Self::Rapunzel,
    Self::Belle,
    Self::BookFan,
    Self::Other,
    Self::Unused,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::MuLan => Some("MuLan"),
      Self::Rapunzel => Some("Rapunzel"),
      Self::Belle => Some("Belle"),
      Self::BookFan => Some("BookFan"),
      Self::Other => Some("Other"),
      Self::Unused => Some("Unused"),
      _ => None,
    }
  }

  #[inline]
  pub fn tag_as_mu_lan(
    o: flatbuffers::WIPOffset<Attacker>,
  ) -> flatbuffers::UnionWIPOffset<CharacterUnionValue> {
    flatbuffers::UnionWIPOffset::new(Self::MuLan, flatbuffers::WIPOffset::new(o.value()))
  }

  #[inline]
  pub fn tag_as_rapunzel(
    o: flatbuffers::WIPOffset<Rapunzel>,
  ) -> flatbuffers::UnionWIPOffset<CharacterUnionValue> {
    flatbuffers::UnionWIPOffset::new(Self::Rapunzel, flatbuffers::WIPOffset::new(o.value()))
  }

  #[inline]
  pub fn tag_as_belle(
    o: flatbuffers::WIPOffset<BookReader>,
  ) -> flatbuffers::UnionWIPOffset<CharacterUnionValue> {
    flatbuffers::UnionWIPOffset::new(Self::Belle, flatbuffers::WIPOffset::new(o.value()))
  }

  #[inline]
  pub fn tag_as_book_fan(
    o: flatbuffers::WIPOffset<BookReader>,
  ) -> flatbuffers::UnionWIPOffset<CharacterUnionValue> {
    flatbuffers::UnionWIPOffset::new(Self::BookFan, flatbuffers::WIPOffset::new(o.value()))
  }

  #[inline]
  pub fn tag_as_other(
    o: flatbuffers::WIPOffset<&str>,
  ) -> flatbuffers::UnionWIPOffset<CharacterUnionValue> {
    flatbuffers::UnionWIPOffset::new(Self::Other, flatbuffers::WIPOffset::new(o.value()))
  }

  #[inline]
  pub fn tag_as_unused(
    o: flatbuffers::WIPOffset<&str>,
  ) -> flatbuffers::UnionWIPOffset<CharacterUnionValue> {
    flatbuffers::UnionWIPOffset::new(Self::Unused, flatbuffers::WIPOffset::new(o.value()))
  }

}
impl std::fmt::Debug for Character {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for Character {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for Character {
    type Output = Character;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for Character {
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

impl<'a> flatbuffers::Verifiable for Character {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Character {}

impl From<Character> for u8 {
  #[inline]
  fn from(v: Character) -> u8 {
    v.0
  }
}

impl<'a: 'b, 'b> flatbuffers::BuildVector<'a, 'b> for Character {
  type VectorBuilder = CharacterVectorBuilder<'a, 'b>;
}

pub struct CharacterVectorBuilder<'a: 'b, 'b> {
  fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  num_items: usize,
}

impl<'a: 'b, 'b> CharacterVectorBuilder<'a, 'b> {
  #[inline]
  pub fn new(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, num_items: usize) -> Self {
    fbb.start_union_vector::<CharacterUnionValue>(num_items);
    Self { fbb, num_items }
  }

  #[inline]
  pub fn finish(&mut self) -> flatbuffers::UnionVectorWIPOffsets<'a, CharacterUnionValue> {
    self.fbb.end_union_vector(self.num_items)
  }

  #[inline]
  pub fn push_as_mu_lan(&mut self, o: flatbuffers::WIPOffset<Attacker>) {
    self.fbb.push_union_vector_item(Character::tag_as_mu_lan(o));
  }

  #[inline]
  pub fn push_as_rapunzel(&mut self, o: flatbuffers::WIPOffset<Rapunzel>) {
    self.fbb.push_union_vector_item(Character::tag_as_rapunzel(o));
  }

  #[inline]
  pub fn push_as_belle(&mut self, o: flatbuffers::WIPOffset<BookReader>) {
    self.fbb.push_union_vector_item(Character::tag_as_belle(o));
  }

  #[inline]
  pub fn push_as_book_fan(&mut self, o: flatbuffers::WIPOffset<BookReader>) {
    self.fbb.push_union_vector_item(Character::tag_as_book_fan(o));
  }

  #[inline]
  pub fn push_as_other(&mut self, o: flatbuffers::WIPOffset<&str>) {
    self.fbb.push_union_vector_item(Character::tag_as_other(o));
  }

  #[inline]
  pub fn push_as_unused(&mut self, o: flatbuffers::WIPOffset<&str>) {
    self.fbb.push_union_vector_item(Character::tag_as_unused(o));
  }

}

pub struct CharacterUnionValue {}

impl flatbuffers::TaggedUnion for CharacterUnionValue {
  type Tag = Character;
}

impl<'a> flatbuffers::UnionVerifiable<'a> for CharacterUnionValue {
  fn run_union_verifier(
    v: &mut flatbuffers::Verifier,
    tag: <<Self as flatbuffers::TaggedUnion>::Tag as flatbuffers::Follow<'a>>::Inner,
    pos: usize,
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    match tag {
      Character::MuLan => v
        .verify_union_variant::<flatbuffers::ForwardsUOffset<Attacker>>(
          "Character::MuLan",
          pos,
        ),
      Character::Rapunzel => v
        .verify_union_variant::<flatbuffers::ForwardsUOffset<Rapunzel>>(
          "Character::Rapunzel",
          pos,
        ),
      Character::Belle => v
        .verify_union_variant::<flatbuffers::ForwardsUOffset<BookReader>>(
          "Character::Belle",
          pos,
        ),
      Character::BookFan => v
        .verify_union_variant::<flatbuffers::ForwardsUOffset<BookReader>>(
          "Character::BookFan",
          pos,
        ),
      Character::Other => v
        .verify_union_variant::<flatbuffers::ForwardsUOffset<&str>>(
          "Character::Other",
          pos,
        ),
      Character::Unused => v
        .verify_union_variant::<flatbuffers::ForwardsUOffset<&str>>(
          "Character::Unused",
          pos,
        ),
      _ => Ok(()),
    }
  }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum CharacterT {
  NONE,
  MuLan(Box<AttackerT>),
  Rapunzel(Box<RapunzelT>),
  Belle(Box<BookReaderT>),
  BookFan(Box<BookReaderT>),
  Other(Box<std::string::String>),
  Unused(Box<std::string::String>),
}
impl Default for CharacterT {
  fn default() -> Self {
    Self::NONE
  }
}
impl CharacterT {
  pub fn character_type(&self) -> Character {
    match self {
      Self::NONE => Character::NONE,
      Self::MuLan(_) => Character::MuLan,
      Self::Rapunzel(_) => Character::Rapunzel,
      Self::Belle(_) => Character::Belle,
      Self::BookFan(_) => Character::BookFan,
      Self::Other(_) => Character::Other,
      Self::Unused(_) => Character::Unused,
    }
  }
  pub fn pack(&self, fbb: &mut flatbuffers::FlatBufferBuilder) -> Option<flatbuffers::WIPOffset<CharacterUnionValue>> {
    match self {
      Self::NONE => None,
      Self::MuLan(v) => Some(Character::tag_as_mu_lan(v.pack(fbb)).value_offset()),
      Self::Rapunzel(v) => Some(Character::tag_as_rapunzel(v.pack(fbb)).value_offset()),
      Self::Belle(v) => Some(Character::tag_as_belle(v.pack(fbb)).value_offset()),
      Self::BookFan(v) => Some(Character::tag_as_book_fan(v.pack(fbb)).value_offset()),
      Self::Other(v) => Some(Character::tag_as_other(fbb.create_string(v.as_str())).value_offset()),
      Self::Unused(v) => Some(Character::tag_as_unused(fbb.create_string(v.as_str())).value_offset()),
    }
  }
  /// If the union variant matches, return the owned AttackerT, setting the union to NONE.
  pub fn take_mu_lan(&mut self) -> Option<Box<AttackerT>> {
    if let Self::MuLan(_) = self {
      let v = std::mem::replace(self, Self::NONE);
      if let Self::MuLan(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the AttackerT.
  pub fn as_mu_lan(&self) -> Option<&AttackerT> {
    if let Self::MuLan(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the AttackerT.
  pub fn as_mu_lan_mut(&mut self) -> Option<&mut AttackerT> {
    if let Self::MuLan(v) = self { Some(v.as_mut()) } else { None }
  }
  /// If the union variant matches, return the owned RapunzelT, setting the union to NONE.
  pub fn take_rapunzel(&mut self) -> Option<Box<RapunzelT>> {
    if let Self::Rapunzel(_) = self {
      let v = std::mem::replace(self, Self::NONE);
      if let Self::Rapunzel(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the RapunzelT.
  pub fn as_rapunzel(&self) -> Option<&RapunzelT> {
    if let Self::Rapunzel(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the RapunzelT.
  pub fn as_rapunzel_mut(&mut self) -> Option<&mut RapunzelT> {
    if let Self::Rapunzel(v) = self { Some(v.as_mut()) } else { None }
  }
  /// If the union variant matches, return the owned BookReaderT, setting the union to NONE.
  pub fn take_belle(&mut self) -> Option<Box<BookReaderT>> {
    if let Self::Belle(_) = self {
      let v = std::mem::replace(self, Self::NONE);
      if let Self::Belle(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the BookReaderT.
  pub fn as_belle(&self) -> Option<&BookReaderT> {
    if let Self::Belle(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the BookReaderT.
  pub fn as_belle_mut(&mut self) -> Option<&mut BookReaderT> {
    if let Self::Belle(v) = self { Some(v.as_mut()) } else { None }
  }
  /// If the union variant matches, return the owned BookReaderT, setting the union to NONE.
  pub fn take_book_fan(&mut self) -> Option<Box<BookReaderT>> {
    if let Self::BookFan(_) = self {
      let v = std::mem::replace(self, Self::NONE);
      if let Self::BookFan(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the BookReaderT.
  pub fn as_book_fan(&self) -> Option<&BookReaderT> {
    if let Self::BookFan(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the BookReaderT.
  pub fn as_book_fan_mut(&mut self) -> Option<&mut BookReaderT> {
    if let Self::BookFan(v) = self { Some(v.as_mut()) } else { None }
  }
  /// If the union variant matches, return the owned std::string::String, setting the union to NONE.
  pub fn take_other(&mut self) -> Option<Box<std::string::String>> {
    if let Self::Other(_) = self {
      let v = std::mem::replace(self, Self::NONE);
      if let Self::Other(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the std::string::String.
  pub fn as_other(&self) -> Option<&std::string::String> {
    if let Self::Other(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the std::string::String.
  pub fn as_other_mut(&mut self) -> Option<&mut std::string::String> {
    if let Self::Other(v) = self { Some(v.as_mut()) } else { None }
  }
  /// If the union variant matches, return the owned std::string::String, setting the union to NONE.
  pub fn take_unused(&mut self) -> Option<Box<std::string::String>> {
    if let Self::Unused(_) = self {
      let v = std::mem::replace(self, Self::NONE);
      if let Self::Unused(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the std::string::String.
  pub fn as_unused(&self) -> Option<&std::string::String> {
    if let Self::Unused(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the std::string::String.
  pub fn as_unused_mut(&mut self) -> Option<&mut std::string::String> {
    if let Self::Unused(v) = self { Some(v.as_mut()) } else { None }
  }
}
// struct Rapunzel, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Default)]
pub struct Rapunzel(pub [u8; 4]);
impl std::fmt::Debug for Rapunzel {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("Rapunzel")
      .field("hair_length", &self.hair_length())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Rapunzel {}
impl flatbuffers::SafeSliceAccess for Rapunzel {}
impl<'a> flatbuffers::Follow<'a> for Rapunzel {
  type Inner = &'a Rapunzel;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Rapunzel>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Rapunzel {
  type Inner = &'a Rapunzel;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Rapunzel>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Rapunzel {
    type Output = Rapunzel;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Rapunzel as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Rapunzel {
    type Output = Rapunzel;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Rapunzel as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Rapunzel {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl Rapunzel {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    hair_length: i32,
  ) -> Self {
    let mut s = Self([0; 4]);
    s.set_hair_length(hair_length);
    s
  }

    pub const fn get_fully_qualified_name() -> &'static str {
        "Rapunzel"
    }

  pub fn hair_length(&self) -> i32 {
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

  pub fn set_hair_length(&mut self, x: i32) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i32 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<i32>(),
      );
    }
  }

  pub fn unpack(&self) -> RapunzelT {
    RapunzelT {
      hair_length: self.hair_length(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RapunzelT {
  pub hair_length: i32,
}
impl RapunzelT {
  pub fn pack(&self) -> Rapunzel {
    Rapunzel::new(
      self.hair_length,
    )
  }
}

// struct BookReader, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Default)]
pub struct BookReader(pub [u8; 4]);
impl std::fmt::Debug for BookReader {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("BookReader")
      .field("books_read", &self.books_read())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for BookReader {}
impl flatbuffers::SafeSliceAccess for BookReader {}
impl<'a> flatbuffers::Follow<'a> for BookReader {
  type Inner = &'a BookReader;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a BookReader>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a BookReader {
  type Inner = &'a BookReader;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<BookReader>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for BookReader {
    type Output = BookReader;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const BookReader as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b BookReader {
    type Output = BookReader;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const BookReader as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for BookReader {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl BookReader {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    books_read: i32,
  ) -> Self {
    let mut s = Self([0; 4]);
    s.set_books_read(books_read);
    s
  }

    pub const fn get_fully_qualified_name() -> &'static str {
        "BookReader"
    }

  pub fn books_read(&self) -> i32 {
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

  pub fn set_books_read(&mut self, x: i32) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i32 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<i32>(),
      );
    }
  }

  pub fn unpack(&self) -> BookReaderT {
    BookReaderT {
      books_read: self.books_read(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BookReaderT {
  pub books_read: i32,
}
impl BookReaderT {
  pub fn pack(&self) -> BookReader {
    BookReader::new(
      self.books_read,
    )
  }
}

pub enum AttackerOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Attacker<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Attacker<'a> {
    type Inner = Attacker<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> Attacker<'a> {
    pub const fn get_fully_qualified_name() -> &'static str {
        "Attacker"
    }

    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Attacker { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args AttackerArgs) -> flatbuffers::WIPOffset<Attacker<'bldr>> {
      let mut builder = AttackerBuilder::new(_fbb);
      builder.add_sword_attack_damage(args.sword_attack_damage);
      builder.finish()
    }

    pub fn unpack(&self) -> AttackerT {
      let sword_attack_damage = self.sword_attack_damage();
      AttackerT {
        sword_attack_damage,
      }
    }
    pub const VT_SWORD_ATTACK_DAMAGE: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn sword_attack_damage(&self) -> i32 {
    self._tab.get::<i32>(Attacker::VT_SWORD_ATTACK_DAMAGE, Some(0)).unwrap()
  }
}

impl flatbuffers::Verifiable for Attacker<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>(&"sword_attack_damage", Self::VT_SWORD_ATTACK_DAMAGE, false)?
     .finish();
    Ok(())
  }
}
pub struct AttackerArgs {
    pub sword_attack_damage: i32,
}
impl<'a> Default for AttackerArgs {
    #[inline]
    fn default() -> Self {
        AttackerArgs {
            sword_attack_damage: 0,
        }
    }
}
pub struct AttackerBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> AttackerBuilder<'a, 'b> {
  #[inline]
  pub fn add_sword_attack_damage(&mut self, sword_attack_damage: i32) {
    self.fbb_.push_slot::<i32>(Attacker::VT_SWORD_ATTACK_DAMAGE, sword_attack_damage, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> AttackerBuilder<'a, 'b> {
    let start = _fbb.start_table();
    AttackerBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Attacker<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Attacker<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Attacker");
      ds.field("sword_attack_damage", &self.sword_attack_damage());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct AttackerT {
  pub sword_attack_damage: i32,
}
impl Default for AttackerT {
  fn default() -> Self {
    Self {
      sword_attack_damage: 0,
    }
  }
}
impl AttackerT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<Attacker<'b>> {
    let sword_attack_damage = self.sword_attack_damage;
    Attacker::create(_fbb, &AttackerArgs{
      sword_attack_damage,
    })
  }
}
pub enum MovieOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Movie<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Movie<'a> {
    type Inner = Movie<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> Movie<'a> {
    pub const fn get_fully_qualified_name() -> &'static str {
        "Movie"
    }

    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Movie { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MovieArgs<'args>) -> flatbuffers::WIPOffset<Movie<'bldr>> {
      let mut builder = MovieBuilder::new(_fbb);
      if let Some(x) = args.characters { builder.add_characters(x); }
      if let Some(x) = args.characters_type { builder.add_characters_type(x); }
      if let Some(x) = args.main_character { builder.add_main_character(x); }
      builder.add_main_character_type(args.main_character_type);
      builder.finish()
    }

    pub fn unpack(&self) -> MovieT {
      let main_character = match self.main_character_type() {
        Character::NONE => CharacterT::NONE,
        Character::MuLan => CharacterT::MuLan(Box::new(
          self.main_character_as_mu_lan()
              .expect("Invalid union table, expected `Character::MuLan`.")
              .unpack()
        )),
        Character::Rapunzel => CharacterT::Rapunzel(Box::new(
          self.main_character_as_rapunzel()
              .expect("Invalid union table, expected `Character::Rapunzel`.")
              .unpack()
        )),
        Character::Belle => CharacterT::Belle(Box::new(
          self.main_character_as_belle()
              .expect("Invalid union table, expected `Character::Belle`.")
              .unpack()
        )),
        Character::BookFan => CharacterT::BookFan(Box::new(
          self.main_character_as_book_fan()
              .expect("Invalid union table, expected `Character::BookFan`.")
              .unpack()
        )),
        Character::Other => CharacterT::Other(Box::new(
          self.main_character_as_other()
              .expect("Invalid union table, expected `Character::Other`.")
              .to_string()
        )),
        Character::Unused => CharacterT::Unused(Box::new(
          self.main_character_as_unused()
              .expect("Invalid union table, expected `Character::Unused`.")
              .to_string()
        )),
        _ => CharacterT::NONE,
      };
      let characters_type = self.characters_type().map(|x| {
        x.iter().collect()
      });
      let characters = characters_type.as_ref().zip(self.characters())
          .map(|x: (&Vec<Character>, flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<flatbuffers::Table<'_>>>)| {
            x.0.iter().zip(x.1.iter()).map(|t| {
              let key = t.0;
              let table = t.1;
              match key {
                &Character::NONE => CharacterT::NONE,
                &Character::MuLan => CharacterT::MuLan(Box::new(
                    <Attacker>::init_from_table(table).unpack())),
                &Character::Rapunzel => CharacterT::Rapunzel(Box::new(
                    <Rapunzel>::init_from_table(table).unpack())),
                &Character::Belle => CharacterT::Belle(Box::new(
                    <BookReader>::init_from_table(table).unpack())),
                &Character::BookFan => CharacterT::BookFan(Box::new(
                    <BookReader>::init_from_table(table).unpack())),
                &Character::Other => CharacterT::Other(Box::new(
                    <&str>::follow(table.buf, table.loc).to_string())),
                &Character::Unused => CharacterT::Unused(Box::new(
                    <&str>::follow(table.buf, table.loc).to_string())),
                _ => CharacterT::NONE,
              }
          }).collect()
      });
      MovieT {
        main_character,
        characters,
      }
    }
    pub const VT_MAIN_CHARACTER_TYPE: flatbuffers::VOffsetT = 4;
    pub const VT_MAIN_CHARACTER: flatbuffers::VOffsetT = 6;
    pub const VT_CHARACTERS_TYPE: flatbuffers::VOffsetT = 8;
    pub const VT_CHARACTERS: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn main_character_type(&self) -> Character {
    self._tab.get::<Character>(Movie::VT_MAIN_CHARACTER_TYPE, Some(Character::NONE)).unwrap()
  }
  #[inline]
  pub fn main_character(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Movie::VT_MAIN_CHARACTER, None)
  }
  #[inline]
  pub fn characters_type(&self) -> Option<flatbuffers::Vector<'a, Character>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, Character>>>(Movie::VT_CHARACTERS_TYPE, None)
  }
  #[inline]
  pub fn characters(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>>>(Movie::VT_CHARACTERS, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn main_character_as_mu_lan(&self) -> Option<Attacker<'a>> {
    if self.main_character_type() == Character::MuLan {
      self.main_character().map(<Attacker>::init_from_table)
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn main_character_as_rapunzel(&self) -> Option<Rapunzel<'a>> {
    if self.main_character_type() == Character::Rapunzel {
      self.main_character().map(<Rapunzel>::init_from_table)
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn main_character_as_belle(&self) -> Option<BookReader<'a>> {
    if self.main_character_type() == Character::Belle {
      self.main_character().map(<BookReader>::init_from_table)
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn main_character_as_book_fan(&self) -> Option<BookReader<'a>> {
    if self.main_character_type() == Character::BookFan {
      self.main_character().map(<BookReader>::init_from_table)
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn main_character_as_other(&self) -> Option<&str> {
    if self.main_character_type() == Character::Other {
      self.main_character().map(|t| <&str>::follow(t.buf, t.loc))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn main_character_as_unused(&self) -> Option<&str> {
    if self.main_character_type() == Character::Unused {
      self.main_character().map(|t| <&str>::follow(t.buf, t.loc))
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for Movie<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_union::<CharacterUnionValue>(&"main_character_type", Self::VT_MAIN_CHARACTER_TYPE, &"main_character", Self::VT_MAIN_CHARACTER, false,
     )?
     .visit_union_vector::<CharacterUnionValue>(&"characters_type", Self::VT_CHARACTERS_TYPE, &"characters", Self::VT_CHARACTERS, false,
     )?
     .finish();
    Ok(())
  }
}
pub struct MovieArgs<'a> {
    pub main_character_type: Character,
    pub main_character: Option<flatbuffers::WIPOffset<CharacterUnionValue>>,
    pub characters_type: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, Character>>>,
    pub characters: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CharacterUnionValue>>>>,
}
impl<'a> Default for MovieArgs<'a> {
    #[inline]
    fn default() -> Self {
        MovieArgs {
            main_character_type: Character::NONE,
            main_character: None,
            characters_type: None,
            characters: None,
        }
    }
}
pub struct MovieBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MovieBuilder<'a, 'b> {
  #[inline]
  pub fn add_main_character_type(&mut self, main_character_type: Character) {
    self.fbb_.push_slot::<Character>(Movie::VT_MAIN_CHARACTER_TYPE, main_character_type, Character::NONE);
  }
  #[inline]
  pub fn add_main_character(&mut self, main_character: flatbuffers::WIPOffset<CharacterUnionValue>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Movie::VT_MAIN_CHARACTER, main_character);
  }
  #[inline]
  pub fn add_characters_type(&mut self, characters_type: flatbuffers::WIPOffset<flatbuffers::Vector<'b , Character>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Movie::VT_CHARACTERS_TYPE, characters_type);
  }
  #[inline]
  pub fn add_characters(&mut self, characters: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<CharacterUnionValue>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Movie::VT_CHARACTERS, characters);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MovieBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MovieBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Movie<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Movie<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Movie");
      ds.field("main_character_type", &self.main_character_type());
      match self.main_character_type() {
        Character::MuLan => {
          if let Some(x) = self.main_character_as_mu_lan() {
            ds.field("main_character", &x)
          } else {
            ds.field("main_character", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        Character::Rapunzel => {
          if let Some(x) = self.main_character_as_rapunzel() {
            ds.field("main_character", &x)
          } else {
            ds.field("main_character", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        Character::Belle => {
          if let Some(x) = self.main_character_as_belle() {
            ds.field("main_character", &x)
          } else {
            ds.field("main_character", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        Character::BookFan => {
          if let Some(x) = self.main_character_as_book_fan() {
            ds.field("main_character", &x)
          } else {
            ds.field("main_character", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        Character::Other => {
          if let Some(x) = self.main_character_as_other() {
            ds.field("main_character", &x)
          } else {
            ds.field("main_character", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        Character::Unused => {
          if let Some(x) = self.main_character_as_unused() {
            ds.field("main_character", &x)
          } else {
            ds.field("main_character", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("main_character", &x)
        },
      };
      ds.field("characters_type", &self.characters_type());
      ds.field("characters", &self.characters());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct MovieT {
  pub main_character: CharacterT,
  pub characters: Option<Vec<CharacterT>>,
}
impl Default for MovieT {
  fn default() -> Self {
    Self {
      main_character: CharacterT::NONE,
      characters: None,
    }
  }
}
impl MovieT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<Movie<'b>> {
    let main_character_type = self.main_character.character_type();
    let main_character = self.main_character.pack(_fbb);
    let characters_type = self.characters.as_ref().map(|x|{
      let w: Vec<_> = x.iter().map(|t| t.character_type()).collect();
      _fbb.create_vector(&w)
    });
    let characters = self.characters.as_ref().map(|x|{
      let w: Vec<_> = x.iter().map(|t| match t.pack(_fbb) { Some(o) => o, None => flatbuffers::WIPOffset::new(0u32),}).collect(); _fbb.create_vector(&w)
    });
    Movie::create(_fbb, &MovieArgs{
      main_character_type,
      main_character,
      characters_type,
      characters,
    })
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_movie<'a>(buf: &'a [u8]) -> Movie<'a> {
  unsafe { flatbuffers::root_unchecked::<Movie<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_movie<'a>(buf: &'a [u8]) -> Movie<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<Movie<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `Movie`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_movie_unchecked`.
pub fn root_as_movie(buf: &[u8]) -> Result<Movie, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Movie>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Movie` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_movie_unchecked`.
pub fn size_prefixed_root_as_movie(buf: &[u8]) -> Result<Movie, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Movie>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Movie` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_movie_unchecked`.
pub fn root_as_movie_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Movie<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Movie<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Movie` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_movie_unchecked`.
pub fn size_prefixed_root_as_movie_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Movie<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Movie<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Movie and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Movie`.
pub unsafe fn root_as_movie_unchecked(buf: &[u8]) -> Movie {
  flatbuffers::root_unchecked::<Movie>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Movie and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Movie`.
pub unsafe fn size_prefixed_root_as_movie_unchecked(buf: &[u8]) -> Movie {
  flatbuffers::size_prefixed_root_unchecked::<Movie>(buf)
}
pub const MOVIE_IDENTIFIER: &str = "MOVI";

#[inline]
pub fn movie_buffer_has_identifier(buf: &[u8]) -> bool {
  flatbuffers::buffer_has_identifier(buf, MOVIE_IDENTIFIER, false)
}

#[inline]
pub fn movie_size_prefixed_buffer_has_identifier(buf: &[u8]) -> bool {
  flatbuffers::buffer_has_identifier(buf, MOVIE_IDENTIFIER, true)
}

#[inline]
pub fn finish_movie_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Movie<'a>>) {
  fbb.finish(root, Some(MOVIE_IDENTIFIER));
}

#[inline]
pub fn finish_size_prefixed_movie_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Movie<'a>>) {
  fbb.finish_size_prefixed(root, Some(MOVIE_IDENTIFIER));
}
