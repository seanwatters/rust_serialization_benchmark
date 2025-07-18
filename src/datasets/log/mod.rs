#[cfg(feature = "capnp")]
pub mod log_capnp;
#[cfg(feature = "flatbuffers")]
#[path = "log_generated.rs"]
#[allow(unused_imports, clippy::all)]
pub mod log_fb;
#[cfg(feature = "prost")]
#[path = "prost.log.rs"]
pub mod log_prost;
#[cfg(feature = "protobuf")]
pub mod log_protobuf;

#[cfg(feature = "flatbuffers")]
use flatbuffers::{FlatBufferBuilder, WIPOffset};
#[cfg(feature = "capnp")]
pub use log_capnp as cp;
#[cfg(feature = "flatbuffers")]
pub use log_fb::log as fb;
#[cfg(feature = "nanoserde")]
use nanoserde::{DeBin, SerBin};
use rand::Rng;
#[cfg(feature = "wiring")]
use wiring::prelude::{Unwiring, Wiring};

#[cfg(feature = "capnp")]
use crate::bench_capnp;
#[cfg(feature = "flatbuffers")]
use crate::bench_flatbuffers;
#[cfg(feature = "prost")]
use crate::bench_prost;
#[cfg(feature = "protobuf")]
use crate::bench_protobuf;
use crate::datasets::BorrowableData;
use crate::Generate;

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(feature = "nibblecode", derive(nibblecode::Serialize))]
#[cfg_attr(feature = "nibblecode", nibblecode(archived = NibblecodeAddress, compare(PartialEq)))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring))]
pub struct Address {
    #[cfg_attr(feature = "wiring", fixed)]
    #[cfg_attr(feature = "minicbor", n(0))]
    pub x0: u8,
    #[cfg_attr(feature = "minicbor", n(1))]
    pub x1: u8,
    #[cfg_attr(feature = "minicbor", n(2))]
    pub x2: u8,
    #[cfg_attr(feature = "minicbor", n(3))]
    pub x3: u8,
}

impl Generate for Address {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        Self {
            x0: rand.gen_range(0..=255),
            x1: rand.gen_range(0..=255),
            x2: rand.gen_range(0..=255),
            x3: rand.gen_range(0..=255),
        }
    }
}

#[cfg(feature = "bilrost")]
mod bilrost_address_encoding {
    //! We implement the encoding for Address here via "proxied" encoding: bilrost has facilities to
    //! encode and decode types via proxy after a transformation. In this case it is much more
    //! efficient, and it makes sense for this use case because ipv4 address components are never
    //! going to be larger than u8 or more numerous than four.
    //!
    //! In a real program it may be that a different type would be chosen that already encodes
    //! efficiently, but this is included as example to show that *how* a field encodes can be
    //! changed without actually affecting the definition of the struct being used.

    use super::Address;
    use bilrost::encoding::{Fixed, ForOverwrite, Proxiable};
    use bilrost::DecodeErrorKind;

    // If `Address` implemented `Default` there is a single macro for both of these.
    impl ForOverwrite<(), Address> for () {
        fn for_overwrite() -> Address {
            Address {
                x0: 0,
                x1: 0,
                x2: 0,
                x3: 0,
            }
        }
    }

    bilrost::empty_state_via_for_overwrite!(Address);

    // Proxiable defines conversions to and from the type that will really be encoded and decoded,
    // and should ideally be bijective and have matching empty states.
    impl Proxiable for Address {
        type Proxy = [u8; 4];

        fn encode_proxy(&self) -> Self::Proxy {
            let Address { x0, x1, x2, x3 } = *self;
            [x0, x1, x2, x3]
        }

        fn decode_proxy(&mut self, proxy: Self::Proxy) -> Result<(), DecodeErrorKind> {
            let [x0, x1, x2, x3] = proxy;
            *self = Address { x0, x1, x2, x3 };
            Ok(())
        }
    }

    // When proxiable is implemented, the Proxied encoding value-encodes our type as  via the proxy,
    // and we can delegate to it.
    bilrost::delegate_proxied_encoding!(
        use encoding (Fixed) to encode proxied type (Address) with general encodings
    );
}

#[cfg(feature = "flatbuffers")]
impl From<Address> for fb::Address {
    #[inline]
    fn from(value: Address) -> Self {
        Self::new(value.x0, value.x1, value.x2, value.x3)
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Address {
    type Reader = cp::address::Reader<'a>;
    type Builder = cp::address::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_x0(self.x0);
        builder.set_x1(self.x1);
        builder.set_x2(self.x2);
        builder.set_x3(self.x3);
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Address {
    type Message = log_prost::Address;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        Self::Message {
            x0: self.x0 as u32,
            x1: self.x1 as u32,
            x2: self.x2 as u32,
            x3: self.x3 as u32,
        }
    }
}

#[cfg(feature = "prost")]
impl From<log_prost::Address> for Address {
    fn from(value: log_prost::Address) -> Self {
        Address {
            x0: value.x0.try_into().unwrap(),
            x1: value.x1.try_into().unwrap(),
            x2: value.x2.try_into().unwrap(),
            x3: value.x3.try_into().unwrap(),
        }
    }
}

#[cfg(feature = "protobuf")]
impl bench_protobuf::Serialize for Address {
    type Message = log_protobuf::log::Address;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        Self::Message {
            x0: self.x0 as u32,
            x1: self.x1 as u32,
            x2: self.x2 as u32,
            x3: self.x3 as u32,
            special_fields: protobuf::SpecialFields::new(),
        }
    }
}

#[cfg(feature = "protobuf")]
impl From<log_protobuf::log::Address> for Address {
    fn from(value: log_protobuf::log::Address) -> Self {
        Address {
            x0: value.x0.try_into().unwrap(),
            x1: value.x1.try_into().unwrap(),
            x2: value.x2.try_into().unwrap(),
            x3: value.x3.try_into().unwrap(),
        }
    }
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(feature = "nibblecode", derive(nibblecode::Serialize))]
#[cfg_attr(feature = "nibblecode", nibblecode(archived = NibblecodeLog, compare(PartialEq)))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring))]
pub struct Log {
    #[cfg_attr(feature = "minicbor", n(0))]
    pub address: Address,
    #[cfg_attr(feature = "minicbor", b(1))]
    pub identity: String,
    #[cfg_attr(feature = "minicbor", b(2))]
    pub userid: String,
    #[cfg_attr(feature = "minicbor", b(3))]
    pub date: String,
    #[cfg_attr(feature = "minicbor", b(4))]
    pub request: String,
    #[cfg_attr(feature = "wiring", fixed)]
    #[cfg_attr(feature = "minicbor", n(5))]
    pub code: u16,
    #[cfg_attr(feature = "minicbor", n(6))]
    pub size: u64,
}

#[derive(PartialEq)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::BorrowDecode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct BorrowLog<'a> {
    #[cfg_attr(feature = "minicbor", n(0))]
    pub address: Address,
    #[cfg_attr(feature = "minicbor", b(1))]
    pub identity: &'a str,
    #[cfg_attr(feature = "minicbor", b(2))]
    pub userid: &'a str,
    #[cfg_attr(feature = "minicbor", b(3))]
    pub date: &'a str,
    #[cfg_attr(feature = "minicbor", b(4))]
    pub request: &'a str,
    #[cfg_attr(feature = "minicbor", n(5))]
    pub code: u16,
    #[cfg_attr(feature = "minicbor", n(6))]
    pub size: u64,
}

impl From<BorrowLog<'_>> for Log {
    fn from(value: BorrowLog<'_>) -> Self {
        Log {
            address: value.address,
            identity: value.identity.to_owned(),
            userid: value.userid.to_owned(),
            date: value.date.to_owned(),
            request: value.request.to_owned(),
            code: value.code,
            size: value.size,
        }
    }
}

impl<'a> From<&'a Log> for BorrowLog<'a> {
    fn from(value: &'a Log) -> Self {
        BorrowLog {
            address: value.address,
            identity: value.identity.as_str(),
            userid: value.userid.as_str(),
            date: value.date.as_str(),
            request: value.request.as_str(),
            code: value.code,
            size: value.size,
        }
    }
}

impl Generate for Log {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        const USERID: [&str; 9] = [
            "-", "alice", "bob", "carmen", "david", "eric", "frank", "george", "harry",
        ];
        const MONTHS: [&str; 12] = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        const TIMEZONE: [&str; 25] = [
            "-1200", "-1100", "-1000", "-0900", "-0800", "-0700", "-0600", "-0500", "-0400",
            "-0300", "-0200", "-0100", "+0000", "+0100", "+0200", "+0300", "+0400", "+0500",
            "+0600", "+0700", "+0800", "+0900", "+1000", "+1100", "+1200",
        ];
        let date = format!(
            "{}/{}/{}:{}:{}:{} {}",
            rand.gen_range(1..=28),
            MONTHS[rand.gen_range(0..12)],
            rand.gen_range(1970..=2021),
            rand.gen_range(0..24),
            rand.gen_range(0..60),
            rand.gen_range(0..60),
            TIMEZONE[rand.gen_range(0..25)],
        );
        const CODES: [u16; 63] = [
            100, 101, 102, 103, 200, 201, 202, 203, 204, 205, 206, 207, 208, 226, 300, 301, 302,
            303, 304, 305, 306, 307, 308, 400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410,
            411, 412, 413, 414, 415, 416, 417, 418, 421, 422, 423, 424, 425, 426, 428, 429, 431,
            451, 500, 501, 502, 503, 504, 505, 506, 507, 508, 510, 511,
        ];
        const METHODS: [&str; 5] = ["GET", "POST", "PUT", "UPDATE", "DELETE"];
        const ROUTES: [&str; 7] = [
            "/favicon.ico",
            "/css/index.css",
            "/css/font-awsome.min.css",
            "/img/logo-full.svg",
            "/img/splash.jpg",
            "/api/login",
            "/api/logout",
        ];
        const PROTOCOLS: [&str; 4] = ["HTTP/1.0", "HTTP/1.1", "HTTP/2", "HTTP/3"];
        let request = format!(
            "{} {} {}",
            METHODS[rand.gen_range(0..5)],
            ROUTES[rand.gen_range(0..7)],
            PROTOCOLS[rand.gen_range(0..4)],
        );
        Self {
            address: Address::generate(rand),
            identity: "-".into(),
            userid: USERID[rand.gen_range(0..USERID.len())].into(),
            date,
            request,
            code: CODES[rand.gen_range(0..CODES.len())],
            size: rand.gen_range(0..100_000_000),
        }
    }
}

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for Log {
    type Target = fb::Log<'a>;

    #[inline]
    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let address = self.address.into();

        let identity = fbb.create_string(&self.identity);
        let userid = fbb.create_string(&self.userid);
        let date = fbb.create_string(&self.date);
        let request = fbb.create_string(&self.request);

        let mut builder = fb::LogBuilder::new(fbb);
        builder.add_address(&address);
        builder.add_identity(identity);
        builder.add_userid(userid);
        builder.add_date(date);
        builder.add_request(request);
        builder.add_code(self.code);
        builder.add_size(self.size);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Log {
    type Reader = cp::log::Reader<'a>;
    type Builder = cp::log::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        use capnp::text::Reader;

        self.address
            .serialize_capnp(&mut builder.reborrow().init_address());
        builder.set_identity(Reader(self.identity.as_bytes()));
        builder.set_userid(Reader(self.userid.as_bytes()));
        builder.set_date(Reader(self.date.as_bytes()));
        builder.set_request(Reader(self.request.as_bytes()));
        builder.set_code(self.code);
        builder.set_size(self.size);
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Log {
    type Message = log_prost::Log;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        Self::Message {
            address: Some(self.address.serialize_pb()),
            identity: self.identity.clone(),
            userid: self.userid.clone(),
            date: self.date.clone(),
            request: self.request.clone(),
            code: self.code as u32,
            size: self.size,
        }
    }
}

#[cfg(feature = "prost")]
impl From<log_prost::Log> for Log {
    fn from(value: log_prost::Log) -> Self {
        Log {
            address: value.address.unwrap().into(),
            identity: value.identity,
            userid: value.userid,
            date: value.date,
            request: value.request,
            code: value.code.try_into().unwrap(),
            size: value.size,
        }
    }
}

#[cfg(feature = "protobuf")]
impl bench_protobuf::Serialize for Log {
    type Message = log_protobuf::log::Log;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        Self::Message {
            address: protobuf::MessageField::some(self.address.serialize_pb()),
            identity: self.identity.clone(),
            userid: self.userid.clone(),
            date: self.date.clone(),
            request: self.request.clone(),
            code: self.code as u32,
            size: self.size,
            special_fields: protobuf::SpecialFields::new(),
        }
    }
}

#[cfg(feature = "protobuf")]
impl From<log_protobuf::log::Log> for Log {
    fn from(value: log_protobuf::log::Log) -> Self {
        Log {
            address: value.address.unwrap().into(),
            identity: value.identity,
            userid: value.userid,
            date: value.date,
            request: value.request,
            code: value.code.try_into().unwrap(),
            size: value.size,
        }
    }
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(feature = "nibblecode", derive(nibblecode::Serialize))]
#[cfg_attr(feature = "nibblecode", nibblecode(archived = NibblecodeLogs, compare(PartialEq)))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring))]
pub struct Logs {
    #[cfg_attr(feature = "bilrost", bilrost(encoding(packed)))]
    #[cfg_attr(feature = "minicbor", n(0))]
    pub logs: Vec<Log>,
}

#[derive(PartialEq)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::BorrowDecode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct BorrowLogs<'a> {
    #[cfg_attr(feature = "bilrost", bilrost(encoding(packed)))]
    #[cfg_attr(feature = "minicbor", b(0))]
    #[serde(borrow)]
    logs: Vec<BorrowLog<'a>>,
}

impl From<BorrowLogs<'_>> for Logs {
    fn from(value: BorrowLogs<'_>) -> Self {
        Logs {
            logs: value.logs.into_iter().map(Into::into).collect(),
        }
    }
}

impl<'a> From<&'a Logs> for BorrowLogs<'a> {
    fn from(value: &'a Logs) -> Self {
        BorrowLogs {
            logs: value.logs.iter().map(Into::into).collect(),
        }
    }
}

impl BorrowableData for Logs {
    type Borrowed<'a> = BorrowLogs<'a>;
}

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for Logs {
    type Target = fb::Logs<'a>;

    #[inline]
    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let mut logs = Vec::new();
        for log in self.logs.iter() {
            logs.push(log.serialize_fb(fbb));
        }
        let logs = fbb.create_vector(&logs);

        let mut builder = fb::LogsBuilder::new(fbb);
        builder.add_logs(logs);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Logs {
    type Reader = cp::logs::Reader<'a>;
    type Builder = cp::logs::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        let mut logs = builder.reborrow().init_logs(self.logs.len() as u32);
        for (i, value) in self.logs.iter().enumerate() {
            value.serialize_capnp(&mut logs.reborrow().get(i as u32));
        }
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Logs {
    type Message = log_prost::Logs;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        for log in self.logs.iter() {
            result.logs.push(log.serialize_pb());
        }
        result
    }
}

#[cfg(feature = "prost")]
impl From<log_prost::Logs> for Logs {
    fn from(value: log_prost::Logs) -> Self {
        Logs {
            logs: value.logs.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(feature = "protobuf")]
impl bench_protobuf::Serialize for Logs {
    type Message = log_protobuf::log::Logs;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::new();
        for log in self.logs.iter() {
            result.logs.push(log.serialize_pb());
        }
        result
    }
}

#[cfg(feature = "protobuf")]
impl From<log_protobuf::log::Logs> for Logs {
    fn from(value: log_protobuf::log::Logs) -> Self {
        Logs {
            logs: value.logs.into_iter().map(Into::into).collect(),
        }
    }
}
