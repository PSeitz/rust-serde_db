use serde;

use de::DeserializableRow;
use de::deserialization_error::{DeserError, DeserResult, prog_err};
use de::db_value::{DbValue, DbValueInto};

#[derive(Debug)]
enum MCD {
    Must,
    Can,
    Done,
}

/// Deserialize a single Row into a normal rust type.
#[derive(Debug)]
pub struct RowDeserializer<ROW> {
    row: ROW,
    cols_treat: MCD,
    next_key: Option<usize>,
}

impl<ROW> RowDeserializer<ROW>
    where ROW: DeserializableRow,
          <ROW as DeserializableRow>::V: DbValue
{
    pub fn new(row: ROW) -> RowDeserializer<ROW> {
        trace!("RowDeserializer::new()");
        let cols_treat = match row.len() {
            1 => MCD::Can,
            _ => MCD::Must,
        };
        RowDeserializer {
            next_key: None,
            cols_treat: cols_treat,
            row: row,
        }
    }

    fn set_next_key(&mut self, next_key: Option<usize>) {
        trace!("RowDeserializer::set_next_key({:?})", next_key);
        self.next_key = next_key;
    }

    fn get_fieldname(&self, idx: usize) -> Option<&String> {
        self.row.get_fieldname(idx)
    }


    fn current_value_pop(&mut self) -> DeserResult<ROW::V> {
        trace!("RowDeserializer::current_value_pop()");
        self.value_deserialization_allowed()?;
        match self.row.pop() {
            Some(tv) => {
                // FIXME trace!("RowDeserializer::current_value_pop(): {:?}", tv);
                Ok(tv)
            }
            None => Err(prog_err("current_value_pop(): no more value found in row")),
        }
    }

    fn current_value_ref(&self) -> DeserResult<&ROW::V> {
        self.value_deserialization_allowed()?;
        match self.row.last() {
            Some(tv) => Ok(tv),
            None => Err(prog_err("current_value_ref(): no more value found in row")),
        }
    }

    fn value_deserialization_allowed(&self) -> DeserResult<()> {
        match self.cols_treat {
            MCD::Must => Err(DeserError::TrailingCols),
            _ => Ok(()),
        }
    }
}

impl<'x, 'a, ROW: DeserializableRow> serde::Deserializer<'x> for &'a mut RowDeserializer<ROW>
    where <ROW as DeserializableRow>::V: DbValue
{
    type Error = DeserError;

    /// This method walks a visitor through a value as it is being deserialized.
    fn deserialize_any<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        panic!("RowDeserializer::deserialize() called");
    }

    /// This method hints that the `Deserialize` type is expecting a `bool` value.
    fn deserialize_bool<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_bool() called");
        visitor.visit_bool(self.current_value_pop()?.try_into()?)
    }

    /// This method hints that the `Deserialize` type is expecting an `u8` value.
    fn deserialize_u8<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_u8() called");
        visitor.visit_u8(self.current_value_pop()?.try_into()?)
    }

    /// This method hints that the `Deserialize` type is expecting an `u16` value.
    fn deserialize_u16<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_u16() called");
        visitor.visit_u16(self.current_value_pop()?.try_into()?)
    }

    /// This method hints that the `Deserialize` type is expecting an `u32` value.
    fn deserialize_u32<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_u32() called");
        visitor.visit_u32(self.current_value_pop()?.try_into()?)
    }

    /// This method hints that the `Deserialize` type is expecting an `u64` value.
    fn deserialize_u64<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_u64() called");
        visitor.visit_u64(self.current_value_pop()?.try_into()?)
    }

    /// This method hints that the `Deserialize` type is expecting an `i8` value.
    fn deserialize_i8<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_i8() called");
        visitor.visit_i8(self.current_value_pop()?.try_into()?)
    }

    /// This method hints that the `Deserialize` type is expecting an `i16` value.
    fn deserialize_i16<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_i16() called");
        visitor.visit_i16(self.current_value_pop()?.try_into()?)
    }

    /// This method hints that the `Deserialize` type is expecting an `i32` value.
    fn deserialize_i32<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_i32() called");
        visitor.visit_i32(self.current_value_pop()?.try_into()?)
    }

    /// This method hints that the `Deserialize` type is expecting an `i64` value.
    fn deserialize_i64<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_i64() called");
        visitor.visit_i64(self.current_value_pop()?.try_into()?)
    }

    /// This method hints that the `Deserialize` type is expecting a `f32` value.
    fn deserialize_f32<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_f32() called");
        visitor.visit_f32(self.current_value_pop()?.try_into()?)
    }

    /// This method hints that the `Deserialize` type is expecting a `f64` value.
    fn deserialize_f64<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_f64() called");
        visitor.visit_f64(self.current_value_pop()?.try_into()?)
    }

    /// This method hints that the `Deserialize` type is expecting a `char` value.
    fn deserialize_char<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        panic!("RowDeserializer::deserialize_char() not implemented!");
    }

    /// This method hints that the `Deserialize` type is expecting a `&str` value.
    fn deserialize_str<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_str() called, delegates to deserialize_string()");
        self.deserialize_string(visitor)
    }

    /// This method hints that the `Deserialize` type is expecting a `String` value.
    fn deserialize_string<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_string() called");
        visitor.visit_string(self.current_value_pop()?.try_into()?)
    }

    /// This method hints that the `Deserialize` type is expecting an `unit` value.
    fn deserialize_unit<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        panic!("RowDeserializer::deserialize_unit(): not implemented!");
    }

    /// This method hints that the `Deserialize` type is expecting an `Option` value. This allows
    /// deserializers that encode an optional value as a nullable value to convert the null value
    /// into a `None`, and a regular value as `Some(value)`.
    fn deserialize_option<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_option() called");
        match self.current_value_ref()?.is_null() {
            false => visitor.visit_some(self),
            true => {
                self.current_value_pop().unwrap();
                visitor.visit_none()
            }
        }
    }

    /// This method hints that the `Deserialize` type is expecting a sequence value. This allows
    /// deserializers to parse sequences that aren't tagged as sequences.
    #[inline]
    fn deserialize_seq<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        panic!("RowDeserializer::deserialize_seq() not implemented");
    }

    /// This method hints that the `Deserialize` type is expecting a map of values. This allows
    /// deserializers to parse sequences that aren't tagged as maps.
    fn deserialize_map<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        panic!("RowDeserializer::deserialize_map(): not implemented!");
    }

    /// This method hints that the `Deserialize` type is expecting a unit struct. This allows
    /// deserializers to a unit struct that aren't tagged as a unit struct.
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        panic!("RowDeserializer::deserialize_unit_struct(): not implemented!");
    }

    /// This method hints that the `Deserialize` type is expecting a newtype struct. This allows
    /// deserializers to a newtype struct that aren't tagged as a newtype struct.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_newtype_struct() called with _name = {}",
               _name);
        visitor.visit_newtype_struct(self)
    }

    /// This method hints that the `Deserialize` type is expecting a tuple struct. This allows
    /// deserializers to parse sequences that aren't tagged as sequences.
    fn deserialize_tuple_struct<V>(self,
                                   _name: &'static str,
                                   _len: usize,
                                   _visitor: V)
                                   -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        panic!("RowDeserializer::deserialize_tuple_struct(): not implemented!");
    }

    /// This method hints that the `Deserialize` type is expecting a struct. This allows
    /// deserializers to parse sequences that aren't tagged as maps.
    fn deserialize_struct<V>(mut self,
                             _name: &'static str,
                             _fields: &'static [&'static str],
                             visitor: V)
                             -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_struct() called");
        match self.cols_treat {
            MCD::Done => Err(prog_err("double-nesting (struct in struct) not possible")),
            _ => {
                self.cols_treat = MCD::Done;
                visitor.visit_map(FieldsMapVisitor::new(&mut self))
            }
        }
    }

    /// Hint that the `Deserialize` type is expecting a byte array and does not
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor<'x> would benefit from taking ownership of `Vec<u8>` data,
    /// indicate this to the `Deserializer` by using `deserialize_byte_buf`
    /// instead.
    fn deserialize_bytes<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_bytes() called");
        visitor.visit_bytes(&DbValueInto::<Vec<u8>>::try_into(self.current_value_pop()?)?)
    }

    /// Hint that the `Deserialize` type is expecting a byte array and would
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor<'x>` would not benefit from taking ownership of `Vec<u8>`
    /// data, indicate that to the `Deserializer` by using `deserialize_bytes`
    /// instead.
    fn deserialize_byte_buf<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_bytes() called");
        visitor.visit_bytes(&DbValueInto::<Vec<u8>>::try_into(self.current_value_pop()?)?)
    }

    /// This method hints that the `Deserialize` type is expecting a tuple value.
    /// This allows deserializers that provide a custom tuple serialization
    /// to properly deserialize the type.
    fn deserialize_tuple<V>(mut self, _len: usize, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_tuple() called");
        match self.cols_treat {
            MCD::Done => {
                Err(prog_err("double-nesting (struct/tuple in struct/tuple) not possible"))
            }
            _ => {
                self.cols_treat = MCD::Done;
                visitor.visit_seq(FieldsSeqVisitor::new(&mut self))
            }
        }
    }

    /// Hint that the `Deserialize` type is expecting an enum value with a
    /// particular name and possible variants.
    fn deserialize_enum<V>(self,
                           _name: &'static str,
                           _variants: &'static [&'static str],
                           _visitor: V)
                           -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        panic!("RowDeserializer::deserialize_enum() not implemented")
    }


    /// This method hints that the Deserialize type is expecting some sort of struct field name.
    /// This allows deserializers to choose between &str, usize, or &[u8] to properly deserialize
    /// a struct field.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        match self.next_key {
            Some(i) => {
                self.next_key = None;
                let fieldname = self.get_fieldname(i).unwrap();
                trace!("RowDeserializer::deserialize_identifier(): column {:?} ({})",
                       i,
                       fieldname);
                visitor.visit_str(fieldname)
            }
            None => {
                trace!("RowDeserializer::deserialize_identifier(): no next_key");
                Err(prog_err("no next_key in RsDeserializer::deserialize_identifier()"))
            }
        }
    }

    /// This method hints that the Deserialize type needs to deserialize a value
    /// whose type doesn't matter because it is ignored.
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        panic!("RowDeserializer::deserialize_ignored_any() not implemented")
    }
}



struct FieldsMapVisitor<'a, R: 'a + DeserializableRow>
    where <R as DeserializableRow>::V: DbValue
{
    de: &'a mut RowDeserializer<R>,
}

impl<'a, R: DeserializableRow> FieldsMapVisitor<'a, R>
    where <R as DeserializableRow>::V: DbValue
{
    pub fn new(de: &'a mut RowDeserializer<R>) -> Self {
        trace!("FieldsMapVisitor::new()");
        FieldsMapVisitor { de: de }
    }
}

impl<'x, 'a, R: DeserializableRow> serde::de::MapAccess<'x> for FieldsMapVisitor<'a, R>
    where <R as DeserializableRow>::V: DbValue
{
    type Error = DeserError;

    /// This returns `Ok(Some(key))` for the next key in the map, or `Ok(None)`
    /// if there are no more remaining entries.
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where K: serde::de::DeserializeSeed<'x>
    {
        match self.de.row.len() {
            0 => {
                trace!("FieldsMapVisitor::next_key_seed() called on empty row");
                Ok(None)
            }
            len => {
                let idx = len - 1;
                trace!("FieldsMapVisitor::next_key_seed() for col {}", idx);
                self.de.set_next_key(Some(idx));
                let value = seed.deserialize(&mut *self.de);
                match value {
                    Ok(res) => Ok(Some(res)),
                    Err(_) => {
                        let fname = self.de.get_fieldname(idx).unwrap();
                        trace!("FieldsMapVisitor::next_key_seed(): Error at {}", fname);
                        Err(DeserError::UnknownField(fname.clone()))
                    }
                }
            }
        }
    }

    /// This returns a `Ok(value)` for the next value in the map.
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where V: serde::de::DeserializeSeed<'x>
    {
        match self.de.row.len() {
            0 => Err(prog_err("FieldsMapVisitor::next_value_seed(): no more value")),
            len => {
                trace!("FieldsMapVisitor::next_value_seed() for col {}", len - 1);
                seed.deserialize(&mut *self.de)
            }
        }
    }
}

struct FieldsSeqVisitor<'a, R: 'a + DeserializableRow>
    where <R as DeserializableRow>::V: DbValue
{
    de: &'a mut RowDeserializer<R>,
}
impl<'a, R: DeserializableRow> FieldsSeqVisitor<'a, R>
    where <R as DeserializableRow>::V: DbValue
{
    pub fn new(de: &'a mut RowDeserializer<R>) -> Self {
        trace!("FieldsSeqVisitor::new()");
        de.row.reverse_values();
        FieldsSeqVisitor { de: de }
    }
}

impl<'x, 'a, R: DeserializableRow> serde::de::SeqAccess<'x> for FieldsSeqVisitor<'a, R>
    where <R as DeserializableRow>::V: DbValue
{
    type Error = DeserError;

    //
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where T: serde::de::DeserializeSeed<'x>
    {
        match self.de.row.len() {
            0 => {
                trace!("FieldsSeqVisitor::next_element_seed() called on empty row");
                Ok(None)
            }
            len => {
                let idx = len - 1;
                trace!("FieldsSeqVisitor::next_element_seed() for column {}", idx);
                self.de.set_next_key(Some(idx));
                let value = seed.deserialize(&mut *self.de);
                match value {
                    Ok(res) => Ok(Some(res)),
                    Err(_) => {
                        let fname = self.de.get_fieldname(idx).unwrap();
                        trace!("FieldsMapVisitor::next_element_seed(): Error at {}", fname);
                        Err(DeserError::UnknownField(fname.clone()))
                    }
                }
            }
        }
    }
}
