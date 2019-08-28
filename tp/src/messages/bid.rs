// This file is generated by rust-protobuf 2.8.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `bid.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct Bid {
    // message fields
    pub record_id: ::std::string::String,
    pub timestamp: u64,
    pub expires: u64,
    pub price: u64,
    pub valid: bool,
    pub owner: ::protobuf::SingularPtrField<Bid_Owner>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Bid {
    fn default() -> &'a Bid {
        <Bid as ::protobuf::Message>::default_instance()
    }
}

impl Bid {
    pub fn new() -> Bid {
        ::std::default::Default::default()
    }

    // string record_id = 1;


    pub fn get_record_id(&self) -> &str {
        &self.record_id
    }
    pub fn clear_record_id(&mut self) {
        self.record_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_record_id(&mut self, v: ::std::string::String) {
        self.record_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_record_id(&mut self) -> &mut ::std::string::String {
        &mut self.record_id
    }

    // Take field
    pub fn take_record_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.record_id, ::std::string::String::new())
    }

    // uint64 timestamp = 2;


    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }

    // uint64 expires = 3;


    pub fn get_expires(&self) -> u64 {
        self.expires
    }
    pub fn clear_expires(&mut self) {
        self.expires = 0;
    }

    // Param is passed by value, moved
    pub fn set_expires(&mut self, v: u64) {
        self.expires = v;
    }

    // uint64 price = 4;


    pub fn get_price(&self) -> u64 {
        self.price
    }
    pub fn clear_price(&mut self) {
        self.price = 0;
    }

    // Param is passed by value, moved
    pub fn set_price(&mut self, v: u64) {
        self.price = v;
    }

    // bool valid = 5;


    pub fn get_valid(&self) -> bool {
        self.valid
    }
    pub fn clear_valid(&mut self) {
        self.valid = false;
    }

    // Param is passed by value, moved
    pub fn set_valid(&mut self, v: bool) {
        self.valid = v;
    }

    // .Bid.Owner owner = 6;


    pub fn get_owner(&self) -> &Bid_Owner {
        self.owner.as_ref().unwrap_or_else(|| Bid_Owner::default_instance())
    }
    pub fn clear_owner(&mut self) {
        self.owner.clear();
    }

    pub fn has_owner(&self) -> bool {
        self.owner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner(&mut self, v: Bid_Owner) {
        self.owner = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner(&mut self) -> &mut Bid_Owner {
        if self.owner.is_none() {
            self.owner.set_default();
        }
        self.owner.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner(&mut self) -> Bid_Owner {
        self.owner.take().unwrap_or_else(|| Bid_Owner::new())
    }
}

impl ::protobuf::Message for Bid {
    fn is_initialized(&self) -> bool {
        for v in &self.owner {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.record_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.expires = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.price = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.valid = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.owner)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.record_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.record_id);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(2, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.expires != 0 {
            my_size += ::protobuf::rt::value_size(3, self.expires, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.price != 0 {
            my_size += ::protobuf::rt::value_size(4, self.price, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.valid != false {
            my_size += 2;
        }
        if let Some(ref v) = self.owner.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.record_id.is_empty() {
            os.write_string(1, &self.record_id)?;
        }
        if self.timestamp != 0 {
            os.write_uint64(2, self.timestamp)?;
        }
        if self.expires != 0 {
            os.write_uint64(3, self.expires)?;
        }
        if self.price != 0 {
            os.write_uint64(4, self.price)?;
        }
        if self.valid != false {
            os.write_bool(5, self.valid)?;
        }
        if let Some(ref v) = self.owner.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Bid {
        Bid::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "record_id",
                    |m: &Bid| { &m.record_id },
                    |m: &mut Bid| { &mut m.record_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    |m: &Bid| { &m.timestamp },
                    |m: &mut Bid| { &mut m.timestamp },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "expires",
                    |m: &Bid| { &m.expires },
                    |m: &mut Bid| { &mut m.expires },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "price",
                    |m: &Bid| { &m.price },
                    |m: &mut Bid| { &mut m.price },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "valid",
                    |m: &Bid| { &m.valid },
                    |m: &mut Bid| { &mut m.valid },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Bid_Owner>>(
                    "owner",
                    |m: &Bid| { &m.owner },
                    |m: &mut Bid| { &mut m.owner },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Bid>(
                    "Bid",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Bid {
        static mut instance: ::protobuf::lazy::Lazy<Bid> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Bid,
        };
        unsafe {
            instance.get(Bid::new)
        }
    }
}

impl ::protobuf::Clear for Bid {
    fn clear(&mut self) {
        self.record_id.clear();
        self.timestamp = 0;
        self.expires = 0;
        self.price = 0;
        self.valid = false;
        self.owner.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Bid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Bid {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Bid_Owner {
    // message fields
    pub agent_id: ::std::string::String,
    pub timestamp: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Bid_Owner {
    fn default() -> &'a Bid_Owner {
        <Bid_Owner as ::protobuf::Message>::default_instance()
    }
}

impl Bid_Owner {
    pub fn new() -> Bid_Owner {
        ::std::default::Default::default()
    }

    // string agent_id = 1;


    pub fn get_agent_id(&self) -> &str {
        &self.agent_id
    }
    pub fn clear_agent_id(&mut self) {
        self.agent_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_agent_id(&mut self, v: ::std::string::String) {
        self.agent_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agent_id(&mut self) -> &mut ::std::string::String {
        &mut self.agent_id
    }

    // Take field
    pub fn take_agent_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.agent_id, ::std::string::String::new())
    }

    // uint64 timestamp = 2;


    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }
}

impl ::protobuf::Message for Bid_Owner {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.agent_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.agent_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.agent_id);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(2, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.agent_id.is_empty() {
            os.write_string(1, &self.agent_id)?;
        }
        if self.timestamp != 0 {
            os.write_uint64(2, self.timestamp)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Bid_Owner {
        Bid_Owner::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "agent_id",
                    |m: &Bid_Owner| { &m.agent_id },
                    |m: &mut Bid_Owner| { &mut m.agent_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    |m: &Bid_Owner| { &m.timestamp },
                    |m: &mut Bid_Owner| { &mut m.timestamp },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Bid_Owner>(
                    "Bid_Owner",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Bid_Owner {
        static mut instance: ::protobuf::lazy::Lazy<Bid_Owner> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Bid_Owner,
        };
        unsafe {
            instance.get(Bid_Owner::new)
        }
    }
}

impl ::protobuf::Clear for Bid_Owner {
    fn clear(&mut self) {
        self.agent_id.clear();
        self.timestamp = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Bid_Owner {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Bid_Owner {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tbid.proto\"\xea\x01\n\x03Bid\x12\x1b\n\trecord_id\x18\x01\x20\x01(\t\
    R\x08recordId\x12\x1c\n\ttimestamp\x18\x02\x20\x01(\x04R\ttimestamp\x12\
    \x18\n\x07expires\x18\x03\x20\x01(\x04R\x07expires\x12\x14\n\x05price\
    \x18\x04\x20\x01(\x04R\x05price\x12\x14\n\x05valid\x18\x05\x20\x01(\x08R\
    \x05valid\x12\x20\n\x05owner\x18\x06\x20\x01(\x0b2\n.Bid.OwnerR\x05owner\
    \x1a@\n\x05Owner\x12\x19\n\x08agent_id\x18\x01\x20\x01(\tR\x07agentId\
    \x12\x1c\n\ttimestamp\x18\x02\x20\x01(\x04R\ttimestampb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}